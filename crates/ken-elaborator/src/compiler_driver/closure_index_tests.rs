use super::*;

const P3: &str = r#"program capabilities FS AFull, RootExecution Allow

import Capability.Process.Exit (host_pure)

fn first_is_slash (path : Bytes) : Bool =
  match bytes_at path (0 : Int) {
    None ↦ False;
    Some first ↦ eq_int (uint8_to_int first) (47 : Int)
  }

fn main
      (input : ProcessInput) (caps : ProgramCaps AFull)
    : HostIO AFull ExitCode =
  match input {
    MkProcessInput arguments _ _ ↦
      match arguments {
        Nil ↦ host_pure AFull ExitCode (ExitCode.Failure 74);
        Cons path _ ↦
          match first_is_slash path {
            False ↦ host_pure AFull ExitCode (ExitCode.Failure 75);
            True ↦ host_pure AFull ExitCode ExitCode.Success
          }
      }
  }
"#;

const P2: &str = r#"program capabilities FS AFull, RootExecution Allow

import Capability.Net.Stream (SocketAddress, socket_unix_path)

import Capability.Process.Exit (host_pure)

fn main
      (input : ProcessInput) (caps : ProgramCaps AFull)
    : HostIO AFull ExitCode =
  match input {
    MkProcessInput arguments _ _ ↦
      match arguments {
        Nil ↦ host_pure AFull ExitCode (ExitCode.Failure 74);
        Cons path _ ↦
          match socket_unix_path path {
            Err _ ↦ host_pure AFull ExitCode (ExitCode.Failure 75);
            Ok _ ↦ host_pure AFull ExitCode ExitCode.Success
          }
      }
  }
"#;

const PX4B: &str = include_str!("../../../ken-cli/tests/fixtures/px4b_process_bytes.ken");

struct EagerDependencyIndex(BTreeMap<StableSymbol, DeclarationDependencies>);

impl EagerDependencyIndex {
    fn new(validated: ValidatedCheckedCorePackage<'_>) -> Self {
        let runtime = declaration_dependency_index(validated);
        let mut semantic = declaration_semantic_dependency_index(validated.package());
        Self(
            runtime
                .into_iter()
                .map(|(owner, runtime)| {
                    let semantic = semantic.remove(&owner).expect("same declaration key set");
                    (owner, DeclarationDependencies { runtime, semantic })
                })
                .collect(),
        )
    }
}

impl DeclarationDependencySource for EagerDependencyIndex {
    fn dependencies(&mut self, symbol: &StableSymbol) -> Option<&DeclarationDependencies> {
        self.0.get(symbol)
    }
}

// Baseline oracle: keep both original per-owner predicates independent of
// production. A single validation is sufficient on the immutable borrow.
fn declaration_dependency_index(
    validated: ValidatedCheckedCorePackage<'_>,
) -> BTreeMap<StableSymbol, BTreeSet<StableSymbol>> {
    let package = validated.package();
    let mut encoded_symbols = package
        .artifact
        .semantic
        .symbols
        .iter()
        .map(|symbol| (symbol.clone(), canonical_symbol_bytes(symbol)))
        .collect::<Vec<_>>();
    encoded_symbols.sort_by(|(left, _), (right, _)| left.cmp(right));

    let mut index = BTreeMap::new();
    for (owner, bytes) in &package.artifact.semantic.declarations {
        let selection = CheckedCoreBodyViewSelection {
            package_identity: package.header.package_identity.clone(),
            package_core_semantic_hash: package.core_semantic_hash,
            package_artifact_hash: package.artifact_hash,
            target_symbol: owner.clone(),
            reachable_declarations: package
                .artifact
                .semantic
                .declarations
                .keys()
                .cloned()
                .collect(),
            external_symbols: BTreeSet::new(),
            dependency_semantic_hashes: package
                .artifact
                .semantic
                .dependency_semantic_hashes
                .clone(),
        };
        let dependencies = match validated_declaration_body_view(validated, &selection, owner) {
            Ok(view) => {
                let mut dependencies = BTreeSet::new();
                collect_runtime_declaration_dependencies(&view.body, &mut dependencies);
                dependencies.remove(owner);
                dependencies
            }
            Err(_) => {
                // Unsupported bodies remain reportable by the established
                // entrypoint lane. Preserve the conservative dependency scan
                // for those declarations; supported bodies use the executable
                // term view so type-only names never enter runtime reachability.
                encoded_symbols
                    .iter()
                    .filter_map(|(candidate, needle)| {
                        (candidate != owner && contains_subslice(bytes, needle))
                            .then_some(candidate.clone())
                    })
                    .collect()
            }
        };
        index.insert(owner.clone(), dependencies);
    }
    index
}

fn declaration_semantic_dependency_index(
    package: &CheckedCorePackage,
) -> BTreeMap<StableSymbol, BTreeSet<StableSymbol>> {
    let encoded_symbols = package
        .artifact
        .semantic
        .symbols
        .iter()
        .map(|symbol| (symbol.clone(), canonical_symbol_bytes(symbol)))
        .collect::<Vec<_>>();

    package
        .artifact
        .semantic
        .declarations
        .iter()
        .map(|(owner, bytes)| {
            let dependencies = encoded_symbols
                .iter()
                .filter_map(|(candidate, needle)| {
                    (candidate != owner && contains_subslice(bytes, needle))
                        .then_some(candidate.clone())
                })
                .collect();
            (owner.clone(), dependencies)
        })
        .collect()
}

/// Promise class: durable invariant. MEASURED: the complete closures of the
/// two dependency sources agree for this target. CLAIMED: on-demand reads
/// preserve the eager dependency result. THE GAP: these routes share the walk
/// and checked body decoder; base-versus-candidate fingerprints independently
/// cover the walk on these fixtures. This test makes no timing claim.
fn assert_indexes_agree(fixture: &'static str, source: &'static str) {
    // Baseline provisioning: use the 256 MiB native-preparation stack, as in
    // checked_core's elaborated host-program pin. The isolated p3 process peaked
    // at 65,088 KiB resident (all mappings, not stack use); this is not a stack
    // headroom assertion.
    const BUILD_STACK_BYTES: usize = 256 * 1024 * 1024;
    std::thread::Builder::new()
        .name(format!("closure-index-{fixture}"))
        .stack_size(BUILD_STACK_BYTES)
        .spawn(move || {
            let root =
                std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../catalog/packages");
            let prepared = prepare_native_program_sources(
                "native-program",
                vec![CompilerSource::new("src/main.ken", source).with_roots(vec![root])],
            )
            .expect("checked native preparation");
            let package = prepared.checked_package_for_test();
            let targets = prepared.selected_targets_for_test();
            assert!(!targets.is_empty(), "{fixture} must select a target");
            let validated = validated_checked_core_package(package).unwrap();
            let mut on_demand = OnDemandDependencyIndex::new(validated);
            let mut eager = EagerDependencyIndex::new(validated);
            for target in targets {
                let demand = build_target_closure(package, target, &mut on_demand).unwrap();
                let baseline = build_target_closure(package, target, &mut eager).unwrap();
                assert_eq!(
                    demand.closure_identity, baseline.closure_identity,
                    "{fixture}: closure identity for {}",
                    target.symbol,
                );
                assert_eq!(demand, baseline, "{fixture}: complete target closure");
            }
        })
        .expect("spawn stated-stack closure comparison")
        .join()
        .expect("closure comparison thread");
}

#[test]
fn p3_closure_equals_eager_index() {
    assert_indexes_agree("p3", P3);
}

#[test]
fn p2_closure_equals_eager_index() {
    assert_indexes_agree("p2", P2);
}

#[test]
fn px4b_closure_equals_eager_index() {
    assert_indexes_agree("px4b", PX4B);
}
