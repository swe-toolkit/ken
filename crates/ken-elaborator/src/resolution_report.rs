//! Opt-in CLI observation of the checked identities fixed by elaboration.
//! No source spelling is resolved again here: every reference comes from the
//! route's already-checked kernel declarations.

use std::collections::{BTreeMap, BTreeSet};

use ken_kernel::{Decl, GlobalId, Term};

use crate::{ElabEnv, ElabError, ElabResult};

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd)]
enum Reference {
    PreSource(Option<String>),
    Imported(String, String),
    Source(String, Option<String>),
}

fn walk(term: &Term, found: &mut BTreeSet<GlobalId>) {
    let mut pending = vec![term];
    while let Some(term) = pending.pop() {
        match term {
            Term::Const { id, .. } | Term::IndFormer { id, .. } | Term::Constructor { id, .. } => {
                found.insert(*id);
            }
            Term::Elim { fam, .. } => {
                found.insert(*fam);
            }
            _ => {}
        }
        pending.extend(term.children());
    }
}

fn declaration_references(
    env: &ElabEnv,
    id: GlobalId,
    found: &mut BTreeSet<GlobalId>,
) -> Result<(), ElabError> {
    match env.env.lookup(id) {
        Some(Decl::Transparent { ty, body, .. }) => {
            walk(ty, found);
            walk(body, found);
        }
        Some(Decl::Opaque { ty, .. } | Decl::Primitive { ty, .. }) => walk(ty, found),
        Some(Decl::Inductive(ind)) => {
            for param in &ind.params {
                walk(param, found);
            }
            for index in &ind.indices {
                walk(index, found);
            }
            walk(&ind.former_type, found);
            for ctor in &ind.constructors {
                for arg in &ctor.args {
                    walk(arg, found);
                }
                for index in &ctor.target_indices {
                    walk(index, found);
                }
                walk(&ctor.type_, found);
            }
        }
        None => {
            return Err(ElabError::Internal(format!(
                "resolution report: root declaration {id:?} is absent"
            )))
        }
    }
    Ok(())
}

fn ownership(results: &[ElabResult]) -> BTreeMap<GlobalId, (String, Option<String>)> {
    let mut owned = BTreeMap::new();
    for result in results {
        owned.insert(result.def_id, (result.name.clone(), None));
        for id in &result.auxiliary_ids {
            owned.insert(*id, (result.name.clone(), None));
        }
    }
    owned
}

fn add_constructors(
    env: &ElabEnv,
    owned: &mut BTreeMap<GlobalId, (String, Option<String>)>,
) -> Result<(), ElabError> {
    let families: Vec<_> = owned
        .iter()
        .map(|(id, (name, _))| (*id, name.clone()))
        .collect();
    for (id, name) in families {
        if let Some(Decl::Inductive(ind)) = env.env.lookup(id) {
            // The checked family owns its constructor identities. The source
            // constructor spelling is captured at the owning result, never
            // obtained by reverse-searching mutable globals for an identity.
            for ctor in &ind.constructors {
                let key = env
                    .module_state
                    .source_name_for_report(ctor.id)
                    .ok_or_else(|| {
                        ElabError::Internal(format!(
                            "resolution report: constructor {:?} has no captured name",
                            ctor.id
                        ))
                    })?;
                let constructor = key.rsplit('.').next().unwrap_or(key).to_owned();
                owned.insert(ctor.id, (name.clone(), Some(constructor)));
            }
        }
    }
    Ok(())
}

fn root_name<'a>(name: &'a str, module: Option<&str>) -> &'a str {
    module
        .and_then(|m| name.strip_prefix(m).and_then(|rest| rest.strip_prefix('.')))
        .unwrap_or(name)
}

fn quoted(s: &str) -> String {
    serde_json::to_string(s).expect("a Rust string is valid JSON text")
}

fn render_ref(reference: Reference) -> String {
    match reference {
        Reference::PreSource(Some(name)) => {
            format!("{{\"class\":\"pre-source\",\"name\":{}}}", quoted(&name))
        }
        Reference::PreSource(None) => "{\"class\":\"pre-source\",\"name\":null}".into(),
        Reference::Imported(module, declaration) => format!(
            "{{\"class\":\"imported\",\"module\":{},\"declaration\":{}}}",
            quoted(&module),
            quoted(&declaration)
        ),
        Reference::Source(declaration, Some(constructor)) => format!(
            "{{\"class\":\"source-declared\",\"declaration\":{},\"constructor\":{}}}",
            quoted(&declaration),
            quoted(&constructor)
        ),
        Reference::Source(declaration, None) => format!(
            "{{\"class\":\"source-declared\",\"declaration\":{}}}",
            quoted(&declaration)
        ),
    }
}

/// A single deterministic newline-terminated JSON document. `results` must be
/// the root's own results, not the transitive import closure; catalog check may
/// additionally pass its entry's checked-fence results. Never mutates `env`.
pub fn resolution_report(
    env: &ElabEnv,
    results: &[ElabResult],
    route: &str,
    module: Option<&str>,
) -> Result<String, ElabError> {
    if !matches!(route, "check" | "run" | "native-build") {
        return Err(ElabError::Internal(
            "resolution report: unknown route".into(),
        ));
    }
    let mut source = ownership(results);
    add_constructors(env, &mut source)?;
    let mut imported = BTreeMap::new();
    for (unit, declarations) in env.module_state.loaded_results_for_report() {
        if Some(unit.as_str()) == module {
            continue;
        }
        let mut owned = ownership(declarations);
        add_constructors(env, &mut owned)?;
        for (id, (name, constructor)) in owned {
            let declaration = match constructor {
                Some(ctor) => format!("{}.{ctor}", root_name(&name, Some(unit))),
                None => root_name(&name, Some(unit)).to_owned(),
            };
            imported.insert(id, (unit.clone(), declaration));
        }
    }
    let mut declarations = Vec::new();
    for result in results {
        let mut ids = BTreeSet::new();
        declaration_references(env, result.def_id, &mut ids)?;
        for id in &result.auxiliary_ids {
            declaration_references(env, *id, &mut ids)?;
        }
        let mut refs = BTreeSet::new();
        for id in ids {
            let reference = if let Some(name) = env.prelude_env.pre_source_names.get(&id) {
                Reference::PreSource(name.clone())
            } else if let Some((name, constructor)) = source.get(&id) {
                Reference::Source(root_name(name, module).to_owned(), constructor.clone())
            } else if let Some((unit, name)) = imported.get(&id) {
                Reference::Imported(unit.clone(), name.clone())
            } else if env.num_values.contains_key(&id) {
                continue;
            } else {
                return Err(ElabError::Internal(format!(
                    "resolution report: unattributed checked identity {id:?}"
                )));
            };
            refs.insert(reference);
        }
        let refs = refs
            .into_iter()
            .map(render_ref)
            .collect::<Vec<_>>()
            .join(",");
        declarations.push(format!(
            "{{\"declaration\":{},\"references\":[{refs}]}}",
            quoted(root_name(&result.name, module))
        ));
    }
    let module = module.map(quoted).unwrap_or_else(|| "null".into());
    Ok(format!("{{\"schema\":\"ken.resolution/v1\",\"route\":{},\"module\":{module},\"declarations\":[{}]}}\n", quoted(route), declarations.join(",")))
}
