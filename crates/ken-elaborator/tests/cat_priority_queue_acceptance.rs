//! CAT-PRIORITY-QUEUE tested-computation acceptance.
//!
//! Authority: `spec/50-stdlib/58a-priority-queues.md` and
//! `conformance/stdlib/collections/seed-priority-queue.md`.
//!
//! These tests execute the selected persistent leftist realization through its
//! exact public identities and inspect its private nodes only at a Rust test
//! boundary. They provide finite computational evidence, not the deferred
//! general queue laws or a machine-checked complexity theorem.

use std::collections::{BTreeMap, BTreeSet};
use std::path::PathBuf;
use std::rc::Rc;

use ken_elaborator::{ElabEnv, ElabError};
use ken_interp::eval::{apply, eval, EvalStore, EvalVal};
use ken_kernel::{Decl, GlobalId, KernelError, Term};

const MODULE: &str = "Data.Collections.PriorityQueue";
const LAWFUL: &str = "Core.Classes.LawfulClasses";

fn catalog_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("catalog/packages")
}

fn load_module() -> (ElabEnv, Vec<GlobalId>) {
    let mut env = ElabEnv::new().expect("base environment");
    let ids = env
        .elaborate_module_from_roots(&[catalog_root()], MODULE)
        .expect("PriorityQueue must elaborate through its declared provider import");
    (env, ids)
}

fn term_mentions(term: &Term, target: GlobalId) -> bool {
    matches!(term, Term::Const { id, .. } if *id == target)
        || term
            .children()
            .into_iter()
            .any(|child| term_mentions(child, target))
}

fn assert_transparent_body_mentions(env: &ElabEnv, wrapper: &str, target: GlobalId) {
    let (_, body) = env
        .env
        .transparent_body(env.globals[wrapper])
        .unwrap_or_else(|| panic!("{wrapper} must be transparent"));
    assert!(
        term_mentions(&body, target),
        "{wrapper} must retain the exact selected public provider identity"
    );
}

fn expect_unbound(result: Result<Vec<GlobalId>, ElabError>, expected: &str) {
    match result {
        Err(ElabError::UnboundName { name, .. }) => assert_eq!(name, expected),
        Err(other) => panic!("expected UnboundName for {expected}, got {other:?}"),
        Ok(_) => panic!("private name {expected} unexpectedly resolved"),
    }
}

fn canonical_ord_nat_id(env: &ElabEnv) -> GlobalId {
    let ord = env.globals["Ord"];
    let nat = env.globals["Nat"];
    let mut matches = env
        .class_env
        .instances
        .iter()
        .filter_map(|((class, head), info)| {
            (env.globals.get(class) == Some(&ord) && env.globals.get(head) == Some(&nat))
                .then_some(info.instance_id)
        });
    let id = matches.next().expect("canonical Ord Nat instance");
    assert!(matches.next().is_none(), "exactly one canonical Ord Nat");
    id
}

fn global_value(env: &ElabEnv, store: &mut EvalStore, id: GlobalId) -> EvalVal {
    match env.env.lookup(id) {
        Some(Decl::Transparent { body, .. }) => eval(&[], body, &env.env, store),
        Some(Decl::Inductive(_)) => EvalVal::IndFormerVal { id },
        other => panic!("global {id:?} must be evaluable, got {other:?}"),
    }
}

fn call_id(
    env: &ElabEnv,
    store: &mut EvalStore,
    id: GlobalId,
    args: impl IntoIterator<Item = EvalVal>,
) -> EvalVal {
    let mut value = global_value(env, store, id);
    for argument in args {
        value = apply(value, argument, &env.env, store);
    }
    let names = env
        .globals
        .iter()
        .filter_map(|(name, candidate)| (*candidate == id).then_some(name.as_str()))
        .collect::<Vec<_>>();
    assert!(
        !matches!(value, EvalVal::Unknown | EvalVal::Neutral),
        "closed computation through global {id:?} {names:?} must not become unknown or neutral"
    );
    value
}

fn ctor_value(env: &ElabEnv, store: &mut EvalStore, id: GlobalId) -> EvalVal {
    eval(&[], &Term::constructor(id, vec![]), &env.env, store)
}

fn nat_value(env: &ElabEnv, store: &mut EvalStore, n: u32) -> EvalVal {
    let mut value = ctor_value(env, store, env.prelude_env.zero_id);
    for _ in 0..n {
        value = apply(
            ctor_value(env, store, env.prelude_env.suc_id),
            value,
            &env.env,
            store,
        );
    }
    value
}

fn decode_nat(env: &ElabEnv, value: &EvalVal) -> u32 {
    match value {
        EvalVal::Ctor { id, .. } if *id == env.prelude_env.zero_id => 0,
        EvalVal::Ctor { id, args, .. } if *id == env.prelude_env.suc_id => {
            1 + decode_nat(env, &args[0])
        }
        other => panic!("expected Nat constructor value, got {other:?}"),
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Order {
    Up,
    Down,
}

impl Order {
    fn allows(self, parent: u32, child: u32) -> bool {
        match self {
            Self::Up => parent <= child,
            Self::Down => parent >= child,
        }
    }

    fn ordered(self, left: u32, right: u32) -> bool {
        self.allows(left, right)
    }
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Entry {
    priority: u32,
    tag: u8,
}

struct Api {
    carrier: GlobalId,
    empty: GlobalId,
    insert: GlobalId,
    find_min: GlobalId,
    pop_min: GlobalId,
    merge: GlobalId,
    empty_ctor: GlobalId,
    node_ctor: GlobalId,
    rank: GlobalId,
    make_node: GlobalId,
    meld: GlobalId,
}

impl Api {
    fn from_env(env: &ElabEnv) -> Self {
        let id = |name: &str| env.globals[&format!("{MODULE}.{name}")];
        Self {
            carrier: id("PriorityQueue"),
            empty: id("empty"),
            insert: id("insert"),
            find_min: id("find_min"),
            pop_min: id("pop_min"),
            merge: id("merge"),
            empty_ctor: id("Empty"),
            node_ctor: id("Node"),
            rank: id("rank"),
            make_node: id("make_node"),
            meld: id("meld"),
        }
    }
}

struct Values {
    nat_ty: EvalVal,
    tag_ty: EvalVal,
    up: EvalVal,
    up2: EvalVal,
    down: EvalVal,
    tags: [EvalVal; 6],
    tag_ids: [GlobalId; 6],
}

fn reify_checked_record(term: &Term, env: &ElabEnv, store: &mut EvalStore) -> EvalVal {
    match term {
        Term::Pair(first, rest) => EvalVal::Pair {
            fst: Rc::new(eval(&[], first, &env.env, store)),
            snd: Rc::new(reify_checked_record(rest, env, store)),
            slot: 0,
        },
        other => eval(&[], other, &env.env, store),
    }
}

fn runtime_ord_dictionary(env: &ElabEnv, store: &mut EvalStore, id: GlobalId) -> EvalVal {
    // `Ord` is proof-carrying. Strict evaluation of its proposition-valued
    // record terminator makes the whole ordinary record `Unknown`. Reify the
    // already kernel-checked transparent record field-by-field so all five real
    // fields remain present and only the unused terminal stays opaque.
    let (_, body) = env
        .env
        .transparent_body(id)
        .unwrap_or_else(|| panic!("Ord dictionary {id:?} must be transparent"));
    let dictionary = reify_checked_record(&body, env, store);
    let mut current = &dictionary;
    let mut fields = 0;
    while let EvalVal::Pair { fst, snd, .. } = current {
        assert!(
            !matches!(**fst, EvalVal::Unknown | EvalVal::Neutral),
            "every reified Ord field must be a real checked runtime value"
        );
        fields += 1;
        current = snd;
    }
    assert_eq!(
        fields, 5,
        "Ord runtime view must retain all five law fields"
    );
    dictionary
}

fn install_test_values(env: &mut ElabEnv, store: &mut EvalStore) -> Values {
    env.elaborate_file(
        "import Core.Classes.LawfulClasses (Ord, leq_nat)\n\
         data Tag = A | B | C | D | E | F\n\
         const cat_pq_up2 : Ord Nat = {\n\
           leq = leq_nat,\n\
           refl = (Ord_instance_Nat).refl,\n\
           antisym = (Ord_instance_Nat).antisym,\n\
           trans = (Ord_instance_Nat).trans,\n\
           total = (Ord_instance_Nat).total\n\
         }\n\
         fn cat_pq_down_leq (x : Nat) (y : Nat) : Bool = leq_nat y x\n\
         const cat_pq_down : Ord Nat = {\n\
           leq = cat_pq_down_leq,\n\
           refl = λx.(Ord_instance_Nat).refl x,\n\
           antisym = λx.λy.λxy.λyx.(Ord_instance_Nat).antisym x y yx xy,\n\
           trans = λx.λy.λz.λxy.λyz.(Ord_instance_Nat).trans z y x yz xy,\n\
           total = λx.λy.(Ord_instance_Nat).total y x\n\
         }",
    )
    .expect("separate lawful up2/down dictionaries and unordered Tag payload must elaborate");

    let up_id = canonical_ord_nat_id(env);
    let up2_id = env.globals["cat_pq_up2"];
    let down_id = env.globals["cat_pq_down"];
    let up = runtime_ord_dictionary(env, store, up_id);
    let up2 = runtime_ord_dictionary(env, store, up2_id);
    let down = runtime_ord_dictionary(env, store, down_id);
    let nat_ty = EvalVal::IndFormerVal {
        id: env.prelude_env.nat_id,
    };
    let zero = nat_value(env, store, 0);
    let one = nat_value(env, store, 1);
    let leq_probe = call_id(
        env,
        store,
        env.globals[&format!("{LAWFUL}.ord_leq_at")],
        [nat_ty.clone(), up.clone(), zero, one],
    );
    assert!(
        matches!(leq_probe, EvalVal::Ctor { id, .. } if id == env.numeric_env.bool_true_id),
        "canonical Ord Nat comparator must execute on the public projection"
    );
    let tag_ids = ["A", "B", "C", "D", "E", "F"].map(|name| env.globals[name]);
    let tags = tag_ids.map(|id| ctor_value(env, store, id));
    Values {
        nat_ty,
        tag_ty: EvalVal::IndFormerVal {
            id: env.globals["Tag"],
        },
        up,
        up2,
        down,
        tags,
        tag_ids,
    }
}

fn dict(values: &Values, order: Order) -> EvalVal {
    match order {
        Order::Up => values.up.clone(),
        Order::Down => values.down.clone(),
    }
}

fn empty_queue(
    env: &ElabEnv,
    store: &mut EvalStore,
    api: &Api,
    values: &Values,
    order: Order,
) -> EvalVal {
    call_id(
        env,
        store,
        api.empty,
        [
            values.nat_ty.clone(),
            values.tag_ty.clone(),
            dict(values, order),
        ],
    )
}

fn insert_entry(
    env: &ElabEnv,
    store: &mut EvalStore,
    api: &Api,
    values: &Values,
    order: Order,
    entry: Entry,
    queue: EvalVal,
) -> EvalVal {
    let priority = nat_value(env, store, entry.priority);
    call_id(
        env,
        store,
        api.insert,
        [
            values.nat_ty.clone(),
            values.tag_ty.clone(),
            dict(values, order),
            priority,
            values.tags[entry.tag as usize].clone(),
            queue,
        ],
    )
}

fn merge_queues(
    env: &ElabEnv,
    store: &mut EvalStore,
    api: &Api,
    values: &Values,
    order: Order,
    left: EvalVal,
    right: EvalVal,
) -> EvalVal {
    call_id(
        env,
        store,
        api.merge,
        [
            values.nat_ty.clone(),
            values.tag_ty.clone(),
            dict(values, order),
            left,
            right,
        ],
    )
}

fn decode_tag(values: &Values, value: &EvalVal) -> u8 {
    let EvalVal::Ctor { id, args, .. } = value else {
        panic!("expected Tag constructor, got {value:?}");
    };
    assert!(args.is_empty(), "Tag constructors have no arguments");
    values
        .tag_ids
        .iter()
        .position(|candidate| candidate == id)
        .unwrap_or_else(|| panic!("unknown Tag constructor {id:?}")) as u8
}

fn decode_entry(env: &ElabEnv, values: &Values, value: &EvalVal) -> Entry {
    let EvalVal::Pair { fst, snd, .. } = value else {
        panic!("expected Pair priority payload, got {value:?}");
    };
    Entry {
        priority: decode_nat(env, fst),
        tag: decode_tag(values, snd),
    }
}

fn decode_option<'a>(env: &ElabEnv, value: &'a EvalVal) -> Option<&'a EvalVal> {
    match value {
        EvalVal::Ctor { id, .. } if *id == env.prelude_env.none_id => None,
        EvalVal::Ctor { id, args, .. } if *id == env.prelude_env.some_id => Some(&args[1]),
        other => panic!("expected Option constructor, got {other:?}"),
    }
}

fn find_entry(
    env: &ElabEnv,
    store: &mut EvalStore,
    api: &Api,
    values: &Values,
    order: Order,
    queue: EvalVal,
) -> Option<Entry> {
    let observed = call_id(
        env,
        store,
        api.find_min,
        [
            values.nat_ty.clone(),
            values.tag_ty.clone(),
            dict(values, order),
            queue,
        ],
    );
    decode_option(env, &observed).map(|entry| decode_entry(env, values, entry))
}

fn pop_entry(
    env: &ElabEnv,
    store: &mut EvalStore,
    api: &Api,
    values: &Values,
    order: Order,
    queue: EvalVal,
) -> Option<(Entry, EvalVal)> {
    let observed = call_id(
        env,
        store,
        api.pop_min,
        [
            values.nat_ty.clone(),
            values.tag_ty.clone(),
            dict(values, order),
            queue,
        ],
    );
    decode_option(env, &observed).map(|payload| {
        let EvalVal::Pair { fst, snd, .. } = payload else {
            panic!("pop_min Some payload must be an entry/remainder Pair")
        };
        (decode_entry(env, values, fst), (**snd).clone())
    })
}

fn drain(
    env: &ElabEnv,
    store: &mut EvalStore,
    api: &Api,
    values: &Values,
    order: Order,
    queue: EvalVal,
) -> Vec<Entry> {
    let mut current = queue;
    let mut result = Vec::new();
    for _ in 0..256 {
        match pop_entry(env, store, api, values, order, current) {
            None => return result,
            Some((entry, remainder)) => {
                result.push(entry);
                current = remainder;
            }
        }
    }
    panic!("finite test queue did not drain within its explicit observation cap")
}

fn build_queue(
    env: &ElabEnv,
    store: &mut EvalStore,
    api: &Api,
    values: &Values,
    order: Order,
    entries: &[Entry],
) -> EvalVal {
    entries.iter().copied().fold(
        empty_queue(env, store, api, values, order),
        |queue, entry| insert_entry(env, store, api, values, order, entry, queue),
    )
}

fn multiset(entries: &[Entry]) -> BTreeMap<Entry, usize> {
    let mut result = BTreeMap::new();
    for entry in entries {
        *result.entry(*entry).or_insert(0) += 1;
    }
    result
}

fn assert_drain_matches(order: Order, actual: &[Entry], expected: &[Entry]) {
    assert_eq!(multiset(actual), multiset(expected));
    assert!(
        actual
            .windows(2)
            .all(|pair| order.ordered(pair[0].priority, pair[1].priority)),
        "drain priorities must follow the supplied lawful order: {actual:?}"
    );
}

fn remove_one(entries: &[Entry], removed: Entry) -> Vec<Entry> {
    let mut result = entries.to_vec();
    let position = result
        .iter()
        .position(|entry| *entry == removed)
        .unwrap_or_else(|| {
            panic!("returned entry {removed:?} must occur in the reference multiset")
        });
    result.remove(position);
    result
}

struct NodeView<'a> {
    cached: u32,
    priority: u32,
    tag: u8,
    left: &'a EvalVal,
    right: &'a EvalVal,
    identity: usize,
}

fn node_view<'a>(
    env: &ElabEnv,
    api: &Api,
    values: &Values,
    queue: &'a EvalVal,
) -> Option<NodeView<'a>> {
    match queue {
        EvalVal::Ctor { id, args, .. } if *id == api.empty_ctor => {
            assert_eq!(args.len(), 3, "Empty carries its three type parameters");
            None
        }
        EvalVal::Ctor { id, args, .. } if *id == api.node_ctor => {
            assert_eq!(
                args.len(),
                8,
                "Node carries three parameters and five fields"
            );
            Some(NodeView {
                cached: decode_nat(env, &args[3]),
                priority: decode_nat(env, &args[4]),
                tag: decode_tag(values, &args[5]),
                left: &args[6],
                right: &args[7],
                identity: Rc::as_ptr(args) as usize,
            })
        }
        other => panic!("expected a private PriorityQueue constructor, got {other:?}"),
    }
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum ValidityFault {
    Cache,
    Balance,
    HeapOrder,
}

#[derive(Clone, Debug, Default)]
struct ShapeObservation {
    faults: BTreeSet<ValidityFault>,
    entries: usize,
    rank: u32,
    right_spine: u32,
    saw_equal_child_ranks: bool,
    saw_unequal_child_ranks: bool,
}

fn inspect_shape(
    env: &ElabEnv,
    api: &Api,
    values: &Values,
    order: Order,
    queue: &EvalVal,
) -> ShapeObservation {
    let Some(node) = node_view(env, api, values, queue) else {
        return ShapeObservation::default();
    };
    let left = inspect_shape(env, api, values, order, node.left);
    let right = inspect_shape(env, api, values, order, node.right);
    let mut faults = left.faults.clone();
    faults.extend(right.faults.iter().copied());
    let recomputed_rank = right.rank + 1;
    if node.cached != recomputed_rank {
        faults.insert(ValidityFault::Cache);
    }
    if left.rank < right.rank {
        faults.insert(ValidityFault::Balance);
    }
    for child in [node.left, node.right] {
        if let Some(child) = node_view(env, api, values, child) {
            if !order.allows(node.priority, child.priority) {
                faults.insert(ValidityFault::HeapOrder);
            }
        }
    }
    let entries = 1 + left.entries + right.entries;
    let right_spine = 1 + right.right_spine;
    assert!(
        right_spine < usize::BITS,
        "bounded fixture right spine must fit the host shift"
    );
    if faults.is_empty() {
        assert!(
            (1usize << right_spine) <= entries + 1,
            "valid selected-leftist queue must satisfy the seed's concrete spine bound"
        );
    }
    ShapeObservation {
        faults,
        entries,
        rank: recomputed_rank,
        right_spine,
        saw_equal_child_ranks: left.saw_equal_child_ranks
            || right.saw_equal_child_ranks
            || left.rank == right.rank,
        saw_unequal_child_ranks: left.saw_unequal_child_ranks
            || right.saw_unequal_child_ranks
            || left.rank != right.rank,
    }
}

fn raw_entries(
    env: &ElabEnv,
    api: &Api,
    values: &Values,
    queue: &EvalVal,
    output: &mut Vec<Entry>,
) {
    let Some(node) = node_view(env, api, values, queue) else {
        return;
    };
    output.push(Entry {
        priority: node.priority,
        tag: node.tag,
    });
    raw_entries(env, api, values, node.left, output);
    raw_entries(env, api, values, node.right, output);
}

fn rewrite_root(api: &Api, queue: &EvalVal, rewrite: impl FnOnce(&mut Vec<EvalVal>)) -> EvalVal {
    let EvalVal::Ctor { id, args, .. } = queue else {
        panic!("fixture root must be a constructor")
    };
    assert_eq!(*id, api.node_ctor, "fixture root must be Node");
    let mut rewritten = (**args).clone();
    rewrite(&mut rewritten);
    EvalVal::Ctor {
        id: *id,
        args: Rc::new(rewritten),
        slot: 0,
    }
}

#[derive(Clone, Debug, Default)]
struct MeldCharge {
    two_nonempty: usize,
    worker_invocations: usize,
    priority_comparisons: usize,
    visited_roots: BTreeSet<usize>,
}

fn meld_charge(
    env: &ElabEnv,
    api: &Api,
    values: &Values,
    order: Order,
    first: &EvalVal,
    second: &EvalVal,
) -> MeldCharge {
    let mut current_first = first;
    let mut current_second = second;
    let mut charge = MeldCharge::default();
    loop {
        charge.worker_invocations += 1;
        let first_node = node_view(env, api, values, current_first);
        let second_node = node_view(env, api, values, current_second);
        if let Some(node) = &first_node {
            charge.visited_roots.insert(node.identity);
        }
        if let Some(node) = &second_node {
            charge.visited_roots.insert(node.identity);
        }
        let (Some(first_node), Some(second_node)) = (first_node, second_node) else {
            return charge;
        };
        charge.two_nonempty += 1;
        charge.priority_comparisons += 1;
        if order.allows(first_node.priority, second_node.priority) {
            current_first = first_node.right;
        } else {
            current_second = second_node.right;
        }
    }
}

fn right_spine_identities(
    env: &ElabEnv,
    api: &Api,
    values: &Values,
    queue: &EvalVal,
) -> BTreeSet<usize> {
    let mut result = BTreeSet::new();
    let mut current = queue;
    while let Some(node) = node_view(env, api, values, current) {
        result.insert(node.identity);
        current = node.right;
    }
    result
}

fn term_reference_count(term: &Term, target: GlobalId) -> usize {
    usize::from(matches!(term, Term::Const { id, .. } if *id == target))
        + term
            .children()
            .into_iter()
            .map(|child| term_reference_count(child, target))
            .sum::<usize>()
}

fn elim_family_count(term: &Term, target: GlobalId) -> usize {
    usize::from(matches!(term, Term::Elim { fam, .. } if *fam == target))
        + term
            .children()
            .into_iter()
            .map(|child| elim_family_count(child, target))
            .sum::<usize>()
}

fn transparent_body(env: &ElabEnv, id: GlobalId) -> Term {
    env.env
        .transparent_body(id)
        .unwrap_or_else(|| panic!("{id:?} must be transparent"))
        .1
}

fn module_transparent_references(env: &ElabEnv, term: &Term) -> BTreeSet<String> {
    let prefix = format!("{MODULE}.");
    env.globals
        .iter()
        .filter_map(|(name, id)| {
            let local = name.strip_prefix(&prefix)?;
            (matches!(env.env.lookup(*id), Some(Decl::Transparent { .. }))
                && term_mentions(term, *id))
            .then_some(local.to_owned())
        })
        .collect()
}

/// Promise class: normative compatibility vector.
///
/// MEASURED: real roots-loader selective imports and kernel bodies retain all
/// six exact exported GlobalIds; real evaluation constructs and drains values
/// through the five public operations. CLAIMED: the six-name abstract public
/// API is usable. THE GAP: constructor/worker privacy is independently checked
/// below; finite execution is not a general queue law.
#[test]
fn public_client_constructs_peeks_merges_and_drains_exact_exports() {
    let (mut env, _) = load_module();
    let api = Api::from_env(&env);
    env.elaborate_file(
        "import Core.Classes.LawfulClasses (Ord)\n\
         import Data.Collections.PriorityQueue\n\
           (PriorityQueue, empty, insert, find_min, pop_min, merge)\n\
         fn cat_pq_client_empty (k : Type) (v : Type) (d : Ord k)\n\
           : PriorityQueue k v (Core.Classes.LawfulClasses.ord_leq_at k d) =\n\
           empty k v d\n\
         fn cat_pq_client_insert (k : Type) (v : Type) (d : Ord k)\n\
           (priority : k) (payload : v)\n\
           (q : PriorityQueue k v (Core.Classes.LawfulClasses.ord_leq_at k d))\n\
           : PriorityQueue k v (Core.Classes.LawfulClasses.ord_leq_at k d) =\n\
           insert k v d priority payload q\n\
         fn cat_pq_client_find (k : Type) (v : Type) (d : Ord k)\n\
           (q : PriorityQueue k v (Core.Classes.LawfulClasses.ord_leq_at k d))\n\
           : Option (Pair k v) = find_min k v d q\n\
         fn cat_pq_client_pop (k : Type) (v : Type) (d : Ord k)\n\
           (q : PriorityQueue k v (Core.Classes.LawfulClasses.ord_leq_at k d))\n\
           : Option (Pair (Pair k v)\n\
             (PriorityQueue k v (Core.Classes.LawfulClasses.ord_leq_at k d))) =\n\
           pop_min k v d q\n\
         fn cat_pq_client_merge (k : Type) (v : Type) (d : Ord k)\n\
           (left : PriorityQueue k v (Core.Classes.LawfulClasses.ord_leq_at k d))\n\
           (right : PriorityQueue k v (Core.Classes.LawfulClasses.ord_leq_at k d))\n\
           : PriorityQueue k v (Core.Classes.LawfulClasses.ord_leq_at k d) =\n\
           merge k v d left right",
    )
    .expect("fresh external client must selectively import and use every public identity");
    for (wrapper, id) in [
        ("cat_pq_client_empty", api.empty),
        ("cat_pq_client_insert", api.insert),
        ("cat_pq_client_find", api.find_min),
        ("cat_pq_client_pop", api.pop_min),
        ("cat_pq_client_merge", api.merge),
    ] {
        assert_transparent_body_mentions(&env, wrapper, id);
    }

    let mut store = EvalStore::new();
    let values = install_test_values(&mut env, &mut store);
    let left = build_queue(
        &env,
        &mut store,
        &api,
        &values,
        Order::Up,
        &[
            Entry {
                priority: 3,
                tag: 0,
            },
            Entry {
                priority: 1,
                tag: 1,
            },
        ],
    );
    assert_eq!(
        find_entry(&env, &mut store, &api, &values, Order::Up, left.clone()),
        Some(Entry {
            priority: 1,
            tag: 1
        })
    );
    let right = build_queue(
        &env,
        &mut store,
        &api,
        &values,
        Order::Up,
        &[
            Entry {
                priority: 2,
                tag: 2,
            },
            Entry {
                priority: 0,
                tag: 3,
            },
        ],
    );
    let merged = merge_queues(&env, &mut store, &api, &values, Order::Up, left, right);
    assert_eq!(
        drain(&env, &mut store, &api, &values, Order::Up, merged),
        vec![
            Entry {
                priority: 0,
                tag: 3
            },
            Entry {
                priority: 1,
                tag: 1
            },
            Entry {
                priority: 2,
                tag: 2
            },
            Entry {
                priority: 3,
                tag: 0
            },
        ]
    );
}

/// Promise class: durable invariant.
///
/// MEASURED: the provider-only trusted-base set is unchanged after the package
/// roots-loads. CLAIMED: this package introduces no Axiom, postulate, primitive,
/// or trusted carrier. THE GAP: inherited provider assumptions remain inherited
/// and are not reclassified by this zero-new-delta check.
#[test]
fn package_adds_zero_trusted_declarations() {
    let mut env = ElabEnv::new().expect("base environment");
    env.elaborate_module_from_roots(&[catalog_root()], LAWFUL)
        .expect("lawful provider must roots-load");
    let before: BTreeSet<_> = env.env.trusted_base().into_iter().collect();
    env.elaborate_module_from_roots(&[catalog_root()], MODULE)
        .expect("PriorityQueue must roots-load after its provider");
    let after: BTreeSet<_> = env.env.trusted_base().into_iter().collect();
    assert_eq!(before, after, "PriorityQueue must add zero trust");
}

/// Promise class: normative compatibility vector.
///
/// MEASURED: every direct declaration is classified by the real selective-
/// import resolver; exactly the six pinned names accept. CLAIMED: this is the
/// complete public inventory. THE GAP: the separate qualified-use and positive
/// public-client controls distinguish interface privacy from artifact absence.
#[test]
fn exact_six_name_export_inventory_and_private_names_refuse() {
    let (mut env, direct_ids) = load_module();
    let direct_ids: BTreeSet<_> = direct_ids.into_iter().collect();
    let mut direct_names = env
        .globals
        .iter()
        .filter_map(|(name, id)| {
            let local = name.strip_prefix(&format!("{MODULE}."))?;
            direct_ids.contains(id).then_some(local.to_owned())
        })
        .collect::<BTreeSet<_>>();
    assert!(direct_names.remove("PriorityQueue"));
    assert!(direct_names.remove("rank"));
    assert!(direct_names.remove("make_node"));
    assert!(direct_names.remove("meld"));
    for name in ["empty", "insert", "find_min", "pop_min", "merge"] {
        assert!(
            direct_names.remove(name),
            "missing direct declaration {name}"
        );
    }
    assert!(
        direct_names.is_empty(),
        "unexpected direct declarations: {direct_names:?}"
    );
    let api = Api::from_env(&env);
    assert!(
        !direct_ids.contains(&api.empty_ctor) && !direct_ids.contains(&api.node_ctor),
        "constructors are attached to the direct inductive rather than top-level module items"
    );

    let expected = BTreeSet::from([
        "PriorityQueue".to_owned(),
        "empty".to_owned(),
        "insert".to_owned(),
        "find_min".to_owned(),
        "pop_min".to_owned(),
        "merge".to_owned(),
    ]);
    let mut observed = BTreeSet::new();
    for name in [
        "PriorityQueue",
        "Empty",
        "Node",
        "rank",
        "make_node",
        "meld",
        "empty",
        "insert",
        "find_min",
        "pop_min",
        "merge",
    ] {
        match env.elaborate_file(&format!("import {MODULE} ({name})")) {
            Ok(_) => {
                observed.insert(name.to_owned());
            }
            Err(ElabError::UnboundName { name: rejected, .. }) => {
                assert_eq!(rejected, format!("{MODULE}.{name}"));
            }
            Err(other) => panic!("unexpected selective-import result for {name}: {other:?}"),
        }
    }
    assert_eq!(observed, expected);

    env.elaborate_file(
        "import Core.Classes.LawfulClasses (Ord, ord_leq_at)\n\
         import Data.Collections.PriorityQueue (PriorityQueue, empty, merge, pop_min)\n\
         fn cat_pq_privacy_positive (k : Type) (v : Type) (d : Ord k)\n\
           : PriorityQueue k v (ord_leq_at k d) = empty k v d",
    )
    .expect("same-boundary positive public client");
    for private in ["Empty", "Node", "meld", "make_node"] {
        expect_unbound(
            env.elaborate_file(&format!("import {MODULE} ({private})")),
            &format!("{MODULE}.{private}"),
        );
    }

    for (private, expression) in [
        (
            "Empty",
            "Data.Collections.PriorityQueue.Empty k v (ord_leq_at k d)",
        ),
        (
            "Node",
            "Data.Collections.PriorityQueue.Node k v (ord_leq_at k d)",
        ),
        (
            "meld",
            "Data.Collections.PriorityQueue.meld k v (ord_leq_at k d) q q",
        ),
        (
            "make_node",
            "Data.Collections.PriorityQueue.make_node k v (ord_leq_at k d)",
        ),
    ] {
        expect_unbound(
            env.elaborate_file(&format!(
                "fn cat_pq_qualified_{private} (k : Type) (v : Type) (d : Ord k) \
                 (q : PriorityQueue k v (ord_leq_at k d)) \
                 : PriorityQueue k v (ord_leq_at k d) = {expression}"
            )),
            &format!("{MODULE}.{private}"),
        );
    }

    // Keep the exact internal identities live for later private-boundary tests.
    assert!(matches!(
        env.env.lookup(api.carrier),
        Some(Decl::Inductive(_))
    ));
    assert!(env.env.constructor(api.empty_ctor).is_some());
    assert!(env.env.constructor(api.node_ctor).is_some());
    for id in [api.rank, api.make_node, api.meld] {
        assert!(matches!(env.env.lookup(id), Some(Decl::Transparent { .. })));
    }
}

/// Promise class: durable invariant.
///
/// MEASURED: independent empty/singleton observations and exact literal entry
/// counts are driven through the public evaluator. CLAIMED: Option behavior,
/// exactly-one removal, and persistence hold on the seed fixtures. THE GAP:
/// bounded traces below broaden the finite population; arbitrary queues remain
/// the deferred proof obligation.
#[test]
fn empty_singleton_duplicates_and_pop_persistence_match_seed() {
    let (mut env, _) = load_module();
    let api = Api::from_env(&env);
    let mut store = EvalStore::new();
    let values = install_test_values(&mut env, &mut store);

    let q0 = empty_queue(&env, &mut store, &api, &values, Order::Up);
    assert_eq!(
        find_entry(&env, &mut store, &api, &values, Order::Up, q0.clone()),
        None
    );
    assert!(pop_entry(&env, &mut store, &api, &values, Order::Up, q0).is_none());

    let one = Entry {
        priority: 2,
        tag: 0,
    };
    let q1 = build_queue(&env, &mut store, &api, &values, Order::Up, &[one]);
    assert_eq!(
        find_entry(&env, &mut store, &api, &values, Order::Up, q1.clone()),
        Some(one)
    );
    let (popped, remainder) =
        pop_entry(&env, &mut store, &api, &values, Order::Up, q1.clone()).expect("singleton pop");
    assert_eq!(popped, one);
    assert_eq!(
        find_entry(
            &env,
            &mut store,
            &api,
            &values,
            Order::Up,
            remainder.clone()
        ),
        None
    );
    assert!(pop_entry(&env, &mut store, &api, &values, Order::Up, remainder).is_none());
    assert_eq!(
        drain(&env, &mut store, &api, &values, Order::Up, q1),
        vec![one]
    );

    let twice = build_queue(&env, &mut store, &api, &values, Order::Up, &[one, one]);
    let (first, rest) = pop_entry(&env, &mut store, &api, &values, Order::Up, twice).unwrap();
    let (second, empty) = pop_entry(&env, &mut store, &api, &values, Order::Up, rest).unwrap();
    assert_eq!((first, second), (one, one));
    assert!(pop_entry(&env, &mut store, &api, &values, Order::Up, empty).is_none());
}

/// Promise class: durable invariant.
///
/// MEASURED: fixed nonmonotone values drain through two separately lawful
/// dictionaries, while separately-built equal-function dictionaries typecheck
/// together and opposite functions fail at the kernel boundary. CLAIMED: order
/// is operational and compatibility is comparator-function, not record identity.
/// THE GAP: lawfulness comes from the provider's already-checked Nat proofs;
/// this test does not re-prove those laws.
#[test]
fn two_lawful_orders_equal_function_interop_and_opposite_type_refusal() {
    let (mut env, _) = load_module();
    let api = Api::from_env(&env);
    let mut store = EvalStore::new();
    let values = install_test_values(&mut env, &mut store);
    let entries = [
        Entry {
            priority: 3,
            tag: 2,
        },
        Entry {
            priority: 1,
            tag: 0,
        },
        Entry {
            priority: 4,
            tag: 3,
        },
        Entry {
            priority: 2,
            tag: 1,
        },
    ];
    let up_queue = build_queue(&env, &mut store, &api, &values, Order::Up, &entries);
    assert_eq!(
        drain(&env, &mut store, &api, &values, Order::Up, up_queue)
            .iter()
            .map(|entry| entry.priority)
            .collect::<Vec<_>>(),
        vec![1, 2, 3, 4]
    );
    let down_queue = build_queue(&env, &mut store, &api, &values, Order::Down, &entries);
    assert_eq!(
        drain(&env, &mut store, &api, &values, Order::Down, down_queue)
            .iter()
            .map(|entry| entry.priority)
            .collect::<Vec<_>>(),
        vec![4, 3, 2, 1]
    );

    env.elaborate_file(
        "import Core.Classes.LawfulClasses (Ord, ord_leq_at)\n\
         import Data.Collections.PriorityQueue (PriorityQueue, empty, insert, merge)\n\
         const cat_pq_eq_q_up : PriorityQueue Nat Tag\n\
           (ord_leq_at Nat Ord_instance_Nat) =\n\
           insert Nat Tag Ord_instance_Nat (Suc Zero) A\n\
             (empty Nat Tag Ord_instance_Nat)\n\
         const cat_pq_eq_q_up2 : PriorityQueue Nat Tag\n\
           (ord_leq_at Nat cat_pq_up2) =\n\
           insert Nat Tag cat_pq_up2 (Suc (Suc Zero)) B\n\
             (empty Nat Tag cat_pq_up2)\n\
         const cat_pq_equal_function_merge : PriorityQueue Nat Tag\n\
           (ord_leq_at Nat Ord_instance_Nat) =\n\
           merge Nat Tag Ord_instance_Nat cat_pq_eq_q_up cat_pq_eq_q_up2",
    )
    .expect("separate proof records with definitionally equal leq_nat fields must interoperate");
    let q_up = build_queue(
        &env,
        &mut store,
        &api,
        &values,
        Order::Up,
        &[Entry {
            priority: 1,
            tag: 0,
        }],
    );
    let q_up2_empty = call_id(
        &env,
        &mut store,
        api.empty,
        [
            values.nat_ty.clone(),
            values.tag_ty.clone(),
            values.up2.clone(),
        ],
    );
    let up2_priority = nat_value(&env, &mut store, 2);
    let q_up2 = call_id(
        &env,
        &mut store,
        api.insert,
        [
            values.nat_ty.clone(),
            values.tag_ty.clone(),
            values.up2.clone(),
            up2_priority,
            values.tags[1].clone(),
            q_up2_empty,
        ],
    );
    let equal_merge = call_id(
        &env,
        &mut store,
        api.merge,
        [
            values.nat_ty.clone(),
            values.tag_ty.clone(),
            values.up.clone(),
            q_up,
            q_up2,
        ],
    );
    assert_eq!(
        drain(&env, &mut store, &api, &values, Order::Up, equal_merge),
        vec![
            Entry {
                priority: 1,
                tag: 0
            },
            Entry {
                priority: 2,
                tag: 1
            }
        ]
    );

    let mismatch = env.elaborate_file(
        "const cat_pq_q_down : PriorityQueue Nat Tag\n\
           (ord_leq_at Nat cat_pq_down) =\n\
           insert Nat Tag cat_pq_down (Suc (Suc (Suc Zero))) C\n\
             (empty Nat Tag cat_pq_down)\n\
         const cat_pq_bad_opposite_merge : PriorityQueue Nat Tag\n\
           (ord_leq_at Nat Ord_instance_Nat) =\n\
           merge Nat Tag Ord_instance_Nat cat_pq_eq_q_up cat_pq_q_down",
    );
    assert!(
        matches!(
            mismatch,
            Err(ElabError::KernelRejected {
                error: KernelError::TypeMismatch { .. },
                ..
            })
        ),
        "opposite comparator index must fail with kernel TypeMismatch, got {mismatch:?}"
    );
}

/// Promise class: durable invariant.
///
/// MEASURED: fixed literal multisets, three independent predicates, tied
/// peek/pop agreement, both merge orientations, self-merge, and later drains of
/// originals. CLAIMED: entries are a persistent multiset and ties do not detach
/// payloads or acquire stability. THE GAP: the expected literals are external
/// to the heap recurrence; this remains finite tested computation.
#[test]
fn entries_ties_self_merge_count_vectors_and_persistence_match_seed() {
    let (mut env, _) = load_module();
    let api = Api::from_env(&env);
    let mut store = EvalStore::new();
    let values = install_test_values(&mut env, &mut store);

    let tied_entries = [
        Entry {
            priority: 1,
            tag: 2,
        },
        Entry {
            priority: 2,
            tag: 0,
        },
        Entry {
            priority: 2,
            tag: 1,
        },
        Entry {
            priority: 2,
            tag: 0,
        },
        Entry {
            priority: 3,
            tag: 3,
        },
    ];
    let tied = build_queue(&env, &mut store, &api, &values, Order::Up, &tied_entries);
    let tied_drain = drain(&env, &mut store, &api, &values, Order::Up, tied);
    assert_drain_matches(Order::Up, &tied_drain, &tied_entries);
    assert_eq!(
        multiset(&tied_drain),
        BTreeMap::from([
            (
                Entry {
                    priority: 1,
                    tag: 2
                },
                1
            ),
            (
                Entry {
                    priority: 2,
                    tag: 0
                },
                2
            ),
            (
                Entry {
                    priority: 2,
                    tag: 1
                },
                1
            ),
            (
                Entry {
                    priority: 3,
                    tag: 3
                },
                1
            ),
        ])
    );

    let tie_entries = [
        Entry {
            priority: 1,
            tag: 0,
        },
        Entry {
            priority: 1,
            tag: 1,
        },
        Entry {
            priority: 2,
            tag: 2,
        },
    ];
    let tie = build_queue(&env, &mut store, &api, &values, Order::Up, &tie_entries);
    let peeked = find_entry(&env, &mut store, &api, &values, Order::Up, tie.clone()).unwrap();
    let (popped, tie_remainder) =
        pop_entry(&env, &mut store, &api, &values, Order::Up, tie.clone()).unwrap();
    assert_eq!(
        peeked, popped,
        "one fixed tied queue must peek and pop the same occurrence"
    );
    assert!(
        popped
            == Entry {
                priority: 1,
                tag: 0
            }
            || popped
                == Entry {
                    priority: 1,
                    tag: 1
                },
        "the selected occurrence must be one of the two literal tied minima"
    );
    let tie_expected_remainder = remove_one(&tie_entries, popped);
    let tie_remainder_drain = drain(&env, &mut store, &api, &values, Order::Up, tie_remainder);
    assert_drain_matches(Order::Up, &tie_remainder_drain, &tie_expected_remainder);
    assert_drain_matches(
        Order::Up,
        &drain(&env, &mut store, &api, &values, Order::Up, tie),
        &tie_entries,
    );

    let self_entries = [
        Entry {
            priority: 1,
            tag: 2,
        },
        Entry {
            priority: 2,
            tag: 0,
        },
        Entry {
            priority: 2,
            tag: 1,
        },
        Entry {
            priority: 2,
            tag: 0,
        },
    ];
    let original = build_queue(&env, &mut store, &api, &values, Order::Up, &self_entries);
    let doubled = merge_queues(
        &env,
        &mut store,
        &api,
        &values,
        Order::Up,
        original.clone(),
        original.clone(),
    );
    let mut expected_doubled = self_entries.to_vec();
    expected_doubled.extend_from_slice(&self_entries);
    assert_drain_matches(
        Order::Up,
        &drain(&env, &mut store, &api, &values, Order::Up, doubled),
        &expected_doubled,
    );
    assert_drain_matches(
        Order::Up,
        &drain(&env, &mut store, &api, &values, Order::Up, original),
        &self_entries,
    );

    let left_entries = [
        Entry {
            priority: 1,
            tag: 0,
        },
        Entry {
            priority: 2,
            tag: 4,
        },
        Entry {
            priority: 4,
            tag: 3,
        },
    ];
    let right_entries = [
        Entry {
            priority: 2,
            tag: 1,
        },
        Entry {
            priority: 3,
            tag: 2,
        },
        Entry {
            priority: 5,
            tag: 5,
        },
    ];
    let left = build_queue(&env, &mut store, &api, &values, Order::Up, &left_entries);
    let right = build_queue(&env, &mut store, &api, &values, Order::Up, &right_entries);
    let mut combined = left_entries.to_vec();
    combined.extend_from_slice(&right_entries);
    for merged in [
        merge_queues(
            &env,
            &mut store,
            &api,
            &values,
            Order::Up,
            left.clone(),
            right.clone(),
        ),
        merge_queues(
            &env,
            &mut store,
            &api,
            &values,
            Order::Up,
            right.clone(),
            left.clone(),
        ),
    ] {
        assert_drain_matches(
            Order::Up,
            &drain(&env, &mut store, &api, &values, Order::Up, merged),
            &combined,
        );
    }
    assert_drain_matches(
        Order::Up,
        &drain(&env, &mut store, &api, &values, Order::Up, left.clone()),
        &left_entries,
    );
    assert_drain_matches(
        Order::Up,
        &drain(&env, &mut store, &api, &values, Order::Up, right.clone()),
        &right_entries,
    );
    let empty = empty_queue(&env, &mut store, &api, &values, Order::Up);
    for merged in [
        merge_queues(
            &env,
            &mut store,
            &api,
            &values,
            Order::Up,
            empty.clone(),
            left.clone(),
        ),
        merge_queues(
            &env,
            &mut store,
            &api,
            &values,
            Order::Up,
            left.clone(),
            empty,
        ),
    ] {
        assert_drain_matches(
            Order::Up,
            &drain(&env, &mut store, &api, &values, Order::Up, merged),
            &left_entries,
        );
    }

    let count_entries = [
        Entry {
            priority: 1,
            tag: 0,
        },
        Entry {
            priority: 2,
            tag: 0,
        },
        Entry {
            priority: 2,
            tag: 1,
        },
        Entry {
            priority: 3,
            tag: 2,
        },
    ];
    let count_q = build_queue(&env, &mut store, &api, &values, Order::Up, &count_entries);
    let counts = |entries: &[Entry]| {
        (
            entries.len(),
            entries.iter().filter(|entry| entry.tag == 0).count(),
            entries
                .iter()
                .filter(|entry| entry.priority % 2 == 0 && entry.tag == 0)
                .count(),
        )
    };
    let mut raw = Vec::new();
    raw_entries(&env, &api, &values, &count_q, &mut raw);
    assert_eq!(counts(&raw), (4, 2, 1));

    let inserted = insert_entry(
        &env,
        &mut store,
        &api,
        &values,
        Order::Up,
        Entry {
            priority: 2,
            tag: 0,
        },
        count_q.clone(),
    );
    raw.clear();
    raw_entries(&env, &api, &values, &inserted, &mut raw);
    assert_eq!(counts(&raw), (5, 3, 2));
    let (insert_removed, insert_remainder) =
        pop_entry(&env, &mut store, &api, &values, Order::Up, inserted).unwrap();
    assert_eq!(
        insert_removed,
        Entry {
            priority: 1,
            tag: 0
        }
    );
    raw.clear();
    raw_entries(&env, &api, &values, &insert_remainder, &mut raw);
    assert_eq!(counts(&raw), (4, 2, 2));
    assert_eq!(counts(&raw).0 + 1, 5);

    let r_entries = [
        Entry {
            priority: 0,
            tag: 3,
        },
        Entry {
            priority: 2,
            tag: 1,
        },
    ];
    let r = build_queue(&env, &mut store, &api, &values, Order::Up, &r_entries);
    let merged = merge_queues(&env, &mut store, &api, &values, Order::Up, count_q, r);
    raw.clear();
    raw_entries(&env, &api, &values, &merged, &mut raw);
    assert_eq!(counts(&raw), (6, 2, 1));
    let (merge_removed, merge_remainder) =
        pop_entry(&env, &mut store, &api, &values, Order::Up, merged).unwrap();
    assert_eq!(
        merge_removed,
        Entry {
            priority: 0,
            tag: 3
        }
    );
    raw.clear();
    raw_entries(&env, &api, &values, &merge_remainder, &mut raw);
    assert_eq!(counts(&raw), (5, 2, 1));
}

/// Promise class: durable invariant for the selected realization.
///
/// MEASURED: recursive child/rank/cache/order observations on production nodes
/// and one-axis malformed private values. CLAIMED: the selected leftist
/// realization maintains every recursive validity clause. THE GAP: this test
/// boundary observes the current private representation only; the universal
/// preservation proof remains deferred.
#[test]
fn recursive_leftist_validity_and_isolated_malformed_fixtures() {
    let (mut env, _) = load_module();
    let api = Api::from_env(&env);
    let mut store = EvalStore::new();
    let values = install_test_values(&mut env, &mut store);
    let histories = [
        vec![0, 1, 2, 3, 4, 5],
        vec![5, 4, 3, 2, 1, 0],
        vec![0, 5, 1, 4, 2, 3],
        vec![2, 2, 2, 1, 1, 3],
    ];
    let mut saw_equal = false;
    let mut saw_unequal = false;
    for priorities in histories {
        let entries = priorities
            .into_iter()
            .enumerate()
            .map(|(index, priority)| Entry {
                priority,
                tag: index as u8,
            })
            .collect::<Vec<_>>();
        let mut queue = build_queue(&env, &mut store, &api, &values, Order::Up, &entries);
        loop {
            let observation = inspect_shape(&env, &api, &values, Order::Up, &queue);
            assert!(
                observation.faults.is_empty(),
                "production queue invalid: {observation:?}"
            );
            saw_equal |= observation.saw_equal_child_ranks;
            saw_unequal |= observation.saw_unequal_child_ranks;
            match pop_entry(&env, &mut store, &api, &values, Order::Up, queue) {
                None => break,
                Some((_, remainder)) => queue = remainder,
            }
        }
    }
    assert!(
        saw_equal,
        "production histories must reach the child-swap-on-equal arm"
    );
    assert!(
        saw_unequal,
        "production histories must reach unequal child ranks"
    );

    let swap_operand = build_queue(
        &env,
        &mut store,
        &api,
        &values,
        Order::Up,
        &[
            Entry {
                priority: 1,
                tag: 0,
            },
            Entry {
                priority: 2,
                tag: 1,
            },
        ],
    );
    let swap_operand_root = node_view(&env, &api, &values, &swap_operand)
        .expect("nonempty swap operand")
        .identity;
    let swapped = insert_entry(
        &env,
        &mut store,
        &api,
        &values,
        Order::Up,
        Entry {
            priority: 0,
            tag: 2,
        },
        swap_operand,
    );
    assert_eq!(
        node_view(&env, &api, &values, &swapped)
            .and_then(|root| node_view(&env, &api, &values, root.left))
            .expect("inserted root must have a nonempty left child")
            .identity,
        swap_operand_root,
        "a lower-priority singleton with empty left child must swap the recursive operand left"
    );

    let no_swap_left_entries = (0..6)
        .map(|priority| Entry {
            priority,
            tag: (priority % 6) as u8,
        })
        .collect::<Vec<_>>();
    let no_swap_left = build_queue(
        &env,
        &mut store,
        &api,
        &values,
        Order::Up,
        &no_swap_left_entries,
    );
    let retained_left_identity = node_view(&env, &api, &values, &no_swap_left)
        .and_then(|root| node_view(&env, &api, &values, root.left))
        .expect("large left operand root must have a left child")
        .identity;
    let no_swap_right = build_queue(
        &env,
        &mut store,
        &api,
        &values,
        Order::Up,
        &[Entry {
            priority: 0,
            tag: 3,
        }],
    );
    let no_swap_merged = merge_queues(
        &env,
        &mut store,
        &api,
        &values,
        Order::Up,
        no_swap_left,
        no_swap_right,
    );
    let no_swap_root =
        node_view(&env, &api, &values, &no_swap_merged).expect("nonempty no-swap merge");
    let no_swap_left_shape = inspect_shape(&env, &api, &values, Order::Up, no_swap_root.left);
    let no_swap_right_shape = inspect_shape(&env, &api, &values, Order::Up, no_swap_root.right);
    assert!(no_swap_left_shape.rank > no_swap_right_shape.rank);
    assert_eq!(
        node_view(&env, &api, &values, no_swap_root.left)
            .expect("retained left child")
            .identity,
        retained_left_identity,
        "an unequal-rank make_node must retain the already-larger child on the left"
    );
    assert!(
        inspect_shape(&env, &api, &values, Order::Up, &no_swap_merged)
            .faults
            .is_empty()
    );

    let subtree = build_queue(
        &env,
        &mut store,
        &api,
        &values,
        Order::Up,
        &[
            Entry {
                priority: 1,
                tag: 1,
            },
            Entry {
                priority: 2,
                tag: 2,
            },
        ],
    );
    let valid = insert_entry(
        &env,
        &mut store,
        &api,
        &values,
        Order::Up,
        Entry {
            priority: 0,
            tag: 0,
        },
        subtree,
    );
    assert!(inspect_shape(&env, &api, &values, Order::Up, &valid)
        .faults
        .is_empty());

    let bad_cache_value = nat_value(&env, &mut store, 7);
    let bad_cache = rewrite_root(&api, &valid, |args| args[3] = bad_cache_value);
    assert_eq!(
        inspect_shape(&env, &api, &values, Order::Up, &bad_cache).faults,
        BTreeSet::from([ValidityFault::Cache])
    );
    let valid_drain = drain(&env, &mut store, &api, &values, Order::Up, valid.clone());
    let bad_cache_drain = drain(&env, &mut store, &api, &values, Order::Up, bad_cache);
    assert_eq!(
        valid_drain, bad_cache_drain,
        "cache fault is invisible to sorted output"
    );

    let right_heavy_cache = nat_value(&env, &mut store, 2);
    let right_heavy = rewrite_root(&api, &valid, |args| {
        args.swap(6, 7);
        args[3] = right_heavy_cache;
    });
    assert_eq!(
        inspect_shape(&env, &api, &values, Order::Up, &right_heavy).faults,
        BTreeSet::from([ValidityFault::Balance])
    );
    assert_drain_matches(
        Order::Up,
        &drain(&env, &mut store, &api, &values, Order::Up, right_heavy),
        &[
            Entry {
                priority: 0,
                tag: 0,
            },
            Entry {
                priority: 1,
                tag: 1,
            },
            Entry {
                priority: 2,
                tag: 2,
            },
        ],
    );

    let inverted_priority = nat_value(&env, &mut store, 3);
    let inverted = rewrite_root(&api, &valid, |args| args[4] = inverted_priority);
    assert_eq!(
        inspect_shape(&env, &api, &values, Order::Up, &inverted).faults,
        BTreeSet::from([ValidityFault::HeapOrder])
    );
}

/// Promise class: durable invariant for the selected realization.
///
/// MEASURED: exact core call/elim structure plus independently charged traces
/// over actual private children. CLAIMED: merge, insert, pop, empty, and find
/// follow the right-spine cost account without a hidden whole-tree helper. THE
/// GAP: this is a structural review measure, not native timing and not a
/// machine-checked asymptotic theorem.
#[test]
fn meld_charge_and_direct_operation_structure_follow_right_spines() {
    let (mut env, _) = load_module();
    let api = Api::from_env(&env);
    let mut store = EvalStore::new();
    let values = install_test_values(&mut env, &mut store);

    let meld_body = transparent_body(&env, api.meld);
    assert_eq!(
        module_transparent_references(&env, &meld_body),
        BTreeSet::from(["make_node".to_owned(), "meld".to_owned()])
    );
    assert_eq!(term_reference_count(&meld_body, api.meld), 2);
    assert_eq!(term_reference_count(&meld_body, api.make_node), 2);
    assert_eq!(elim_family_count(&meld_body, api.carrier), 2);
    assert_eq!(elim_family_count(&meld_body, env.numeric_env.bool_id), 1);

    let make_node_body = transparent_body(&env, api.make_node);
    assert_eq!(
        module_transparent_references(&env, &make_node_body),
        BTreeSet::from(["rank".to_owned()])
    );
    assert_eq!(term_reference_count(&make_node_body, api.rank), 4);
    assert_eq!(
        term_reference_count(&make_node_body, env.globals[&format!("{LAWFUL}.leq_nat")]),
        1
    );
    assert_eq!(elim_family_count(&make_node_body, api.carrier), 0);
    assert_eq!(
        elim_family_count(&make_node_body, env.numeric_env.bool_id),
        1
    );

    let merge_body = transparent_body(&env, api.merge);
    assert_eq!(
        module_transparent_references(&env, &merge_body),
        BTreeSet::from(["meld".to_owned()])
    );
    assert_eq!(term_reference_count(&merge_body, api.meld), 1);
    assert_eq!(elim_family_count(&merge_body, api.carrier), 0);
    let insert_body = transparent_body(&env, api.insert);
    assert_eq!(
        module_transparent_references(&env, &insert_body),
        BTreeSet::from(["merge".to_owned()])
    );
    assert_eq!(term_reference_count(&insert_body, api.merge), 1);
    assert_eq!(elim_family_count(&insert_body, api.carrier), 0);
    let find_body = transparent_body(&env, api.find_min);
    assert!(module_transparent_references(&env, &find_body).is_empty());
    assert_eq!(term_reference_count(&find_body, api.meld), 0);
    assert_eq!(elim_family_count(&find_body, api.carrier), 1);
    let pop_body = transparent_body(&env, api.pop_min);
    assert_eq!(
        module_transparent_references(&env, &pop_body),
        BTreeSet::from(["merge".to_owned()])
    );
    assert_eq!(term_reference_count(&pop_body, api.merge), 1);
    assert_eq!(elim_family_count(&pop_body, api.carrier), 1);

    let empty = empty_queue(&env, &mut store, &api, &values, Order::Up);
    let singleton = build_queue(
        &env,
        &mut store,
        &api,
        &values,
        Order::Up,
        &[Entry {
            priority: 4,
            tag: 0,
        }],
    );
    for (left, right) in [(&empty, &empty), (&empty, &singleton), (&singleton, &empty)] {
        let charge = meld_charge(&env, &api, &values, Order::Up, left, right);
        assert_eq!(charge.two_nonempty, 0);
        assert_eq!(charge.priority_comparisons, 0);
        assert_eq!(charge.worker_invocations, 1);
    }

    let left = build_queue(
        &env,
        &mut store,
        &api,
        &values,
        Order::Up,
        &[
            Entry {
                priority: 0,
                tag: 0,
            },
            Entry {
                priority: 4,
                tag: 1,
            },
            Entry {
                priority: 6,
                tag: 2,
            },
            Entry {
                priority: 8,
                tag: 3,
            },
        ],
    );
    let right = build_queue(
        &env,
        &mut store,
        &api,
        &values,
        Order::Up,
        &[
            Entry {
                priority: 1,
                tag: 4,
            },
            Entry {
                priority: 3,
                tag: 5,
            },
            Entry {
                priority: 5,
                tag: 0,
            },
        ],
    );
    let charge = meld_charge(&env, &api, &values, Order::Up, &left, &right);
    assert_eq!(charge.priority_comparisons, charge.two_nonempty);
    assert_eq!(charge.worker_invocations, charge.two_nonempty + 1);
    let left_spine = right_spine_identities(&env, &api, &values, &left);
    let right_spine = right_spine_identities(&env, &api, &values, &right);
    let permitted = left_spine
        .union(&right_spine)
        .copied()
        .collect::<BTreeSet<_>>();
    assert!(charge.visited_roots.is_subset(&permitted));
    assert!(
        charge.two_nonempty
            <= inspect_shape(&env, &api, &values, Order::Up, &left).right_spine as usize
                + inspect_shape(&env, &api, &values, Order::Up, &right).right_spine as usize
    );
    let merged = merge_queues(
        &env,
        &mut store,
        &api,
        &values,
        Order::Up,
        left.clone(),
        right.clone(),
    );
    assert!(inspect_shape(&env, &api, &values, Order::Up, &merged)
        .faults
        .is_empty());

    let inserted_entry = Entry {
        priority: 2,
        tag: 2,
    };
    let singleton_for_insert = build_queue(
        &env,
        &mut store,
        &api,
        &values,
        Order::Up,
        &[inserted_entry],
    );
    let insert_charge = meld_charge(&env, &api, &values, Order::Up, &singleton_for_insert, &left);
    assert_eq!(
        insert_charge.priority_comparisons,
        insert_charge.two_nonempty
    );
    assert_eq!(
        insert_charge.worker_invocations,
        insert_charge.two_nonempty + 1
    );
    assert!(
        insert_charge.two_nonempty
            <= 1 + inspect_shape(&env, &api, &values, Order::Up, &left).right_spine as usize
    );
    let inserted = insert_entry(
        &env,
        &mut store,
        &api,
        &values,
        Order::Up,
        inserted_entry,
        left,
    );
    assert!(inspect_shape(&env, &api, &values, Order::Up, &inserted)
        .faults
        .is_empty());

    let root = node_view(&env, &api, &values, &merged).expect("nonempty merged queue");
    assert!(node_view(&env, &api, &values, root.left).is_some());
    assert!(node_view(&env, &api, &values, root.right).is_some());
    let pop_charge = meld_charge(&env, &api, &values, Order::Up, root.left, root.right);
    assert_eq!(pop_charge.priority_comparisons, pop_charge.two_nonempty);
    assert_eq!(pop_charge.worker_invocations, pop_charge.two_nonempty + 1);
    assert!(
        pop_charge.two_nonempty
            <= inspect_shape(&env, &api, &values, Order::Up, root.left).right_spine as usize
                + inspect_shape(&env, &api, &values, Order::Up, root.right).right_spine as usize
    );
    let (_, remainder) = pop_entry(&env, &mut store, &api, &values, Order::Up, merged).unwrap();
    assert!(inspect_shape(&env, &api, &values, Order::Up, &remainder)
        .faults
        .is_empty());
}

fn alphabet() -> [Entry; 6] {
    [
        Entry {
            priority: 0,
            tag: 0,
        },
        Entry {
            priority: 0,
            tag: 1,
        },
        Entry {
            priority: 1,
            tag: 0,
        },
        Entry {
            priority: 1,
            tag: 1,
        },
        Entry {
            priority: 2,
            tag: 0,
        },
        Entry {
            priority: 2,
            tag: 1,
        },
    ]
}

fn finite_histories(
    env: &ElabEnv,
    store: &mut EvalStore,
    api: &Api,
    values: &Values,
    order: Order,
    maximum_length: usize,
) -> Vec<(Vec<Entry>, EvalVal)> {
    let alphabet = alphabet();
    let mut histories = Vec::new();
    for length in 0..=maximum_length {
        let population = alphabet.len().pow(length as u32);
        for mut ordinal in 0..population {
            let mut entries = Vec::with_capacity(length);
            for _ in 0..length {
                entries.push(alphabet[ordinal % alphabet.len()]);
                ordinal /= alphabet.len();
            }
            let queue = build_queue(env, store, api, values, order, &entries);
            histories.push((entries, queue));
        }
    }
    histories
}

fn check_finite_observation(
    env: &ElabEnv,
    store: &mut EvalStore,
    api: &Api,
    values: &Values,
    order: Order,
    model: &[Entry],
    queue: EvalVal,
) {
    let model_empty = model.is_empty();
    let peek = find_entry(env, store, api, values, order, queue.clone());
    let popped = pop_entry(env, store, api, values, order, queue.clone());
    assert_eq!(peek.is_none(), model_empty, "find_min None iff model empty");
    assert_eq!(
        popped.is_none(),
        model_empty,
        "pop_min None iff model empty"
    );
    assert!(inspect_shape(env, api, values, order, &queue)
        .faults
        .is_empty());
    if let Some((removed, remainder)) = popped {
        assert_eq!(
            peek,
            Some(removed),
            "peek and pop select one identical occurrence"
        );
        assert!(
            model
                .iter()
                .all(|entry| order.allows(removed.priority, entry.priority)),
            "returned entry must have a model-minimum priority"
        );
        let remainder_model = remove_one(model, removed);
        assert!(inspect_shape(env, api, values, order, &remainder)
            .faults
            .is_empty());
        assert_drain_matches(
            order,
            &drain(env, store, api, values, order, remainder),
            &remainder_model,
        );
    }
    assert_drain_matches(order, &drain(env, store, api, values, order, queue), model);
}

/// Promise class: durable invariant.
///
/// MEASURED: the pre-counted closed domain from seed `PQ5` executes exactly
/// 4,216 public traces under both lawful orders, including every ordered short
/// merge pair and every self-pair. CLAIMED: the finite domain observes both
/// None equivalences, minimum selection, exact one-occurrence removal,
/// persistence, multiplicity, and valid remainders. THE GAP: this is explicitly
/// bounded evidence and does not prove the general laws deferred by `58a §7`.
#[test]
fn bounded_4216_traces_match_independent_literal_multisets() {
    const INSERTION_HISTORIES: usize = 1 + 6 + 36 + 216;
    const SHORT_OPERANDS: usize = 1 + 6 + 36;
    const ORDERED_MERGES: usize = SHORT_OPERANDS * SHORT_OPERANDS;
    const PER_ORDER: usize = INSERTION_HISTORIES + ORDERED_MERGES;
    const ALL_TRACES: usize = 2 * PER_ORDER;
    assert_eq!(INSERTION_HISTORIES, 259);
    assert_eq!(SHORT_OPERANDS, 43);
    assert_eq!(ORDERED_MERGES, 1_849);
    assert_eq!(ALL_TRACES, 4_216);

    let (mut env, _) = load_module();
    let api = Api::from_env(&env);
    let mut store = EvalStore::new();
    let values = install_test_values(&mut env, &mut store);
    let mut executed = 0;
    for order in [Order::Up, Order::Down] {
        let histories = finite_histories(&env, &mut store, &api, &values, order, 3);
        assert_eq!(histories.len(), INSERTION_HISTORIES);
        for (model, queue) in &histories {
            check_finite_observation(&env, &mut store, &api, &values, order, model, queue.clone());
            executed += 1;
        }

        let short = &histories[..SHORT_OPERANDS];
        let mut merge_count = 0;
        let mut self_count = 0;
        for (left_ordinal, (left_model, left_queue)) in short.iter().enumerate() {
            for (right_ordinal, (right_model, right_queue)) in short.iter().enumerate() {
                let merged = merge_queues(
                    &env,
                    &mut store,
                    &api,
                    &values,
                    order,
                    left_queue.clone(),
                    right_queue.clone(),
                );
                let mut merged_model = left_model.clone();
                merged_model.extend_from_slice(right_model);
                check_finite_observation(
                    &env,
                    &mut store,
                    &api,
                    &values,
                    order,
                    &merged_model,
                    merged,
                );
                merge_count += 1;
                executed += 1;
                if left_ordinal == right_ordinal {
                    self_count += 1;
                }
            }
        }
        assert_eq!(merge_count, ORDERED_MERGES);
        assert_eq!(self_count, SHORT_OPERANDS);
        for (model, original) in short {
            assert_drain_matches(
                order,
                &drain(&env, &mut store, &api, &values, order, original.clone()),
                model,
            );
        }
    }
    assert_eq!(executed, ALL_TRACES);
}
