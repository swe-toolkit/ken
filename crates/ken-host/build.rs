mod build_support;

use sha2::{Digest, Sha256};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

const SCHEMA_VERSION: u32 = 2;

fn main() {
    println!("cargo:rerun-if-changed=abi_probe.c");
    println!("cargo:rerun-if-changed=effect_abi_probe.c");
    println!("cargo:rerun-if-changed=effect_abi_v1.catalog");
    println!("cargo:rerun-if-changed=src/abi_v1/sigpipe.c");
    println!("cargo:rerun-if-changed=build_support.rs");
    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=../ken-interp/src/eval.rs");
    println!("cargo:rerun-if-changed=Cargo.toml");
    println!("cargo:rerun-if-env-changed=KEN_HOST_ABI_TEST_MISMATCH");
    println!("cargo:rerun-if-env-changed=RUSTFLAGS");

    let target = env::var("TARGET").expect("Cargo provides TARGET");
    let host = env::var("HOST").expect("Cargo provides HOST");
    let target_os = env::var("CARGO_CFG_TARGET_OS").expect("Cargo provides target OS");
    let target_arch = env::var("CARGO_CFG_TARGET_ARCH").expect("Cargo provides target arch");
    let target_endianness =
        env::var("CARGO_CFG_TARGET_ENDIAN").expect("Cargo provides target endianness");
    if target_os != "linux" || target != host {
        panic!(
            "HostEffectAbiV1 layout generation is unavailable for target {target}; \
             cross-target or non-Linux effect ABI generation fails closed"
        );
    }
    let encoded_rustflags = env::var("CARGO_ENCODED_RUSTFLAGS").unwrap_or_default();
    let rustflags = env::var("RUSTFLAGS").unwrap_or_default();
    assert!(
        !encoded_rustflags.contains("rustix_use_libc")
            && !rustflags.contains("rustix_use_libc")
            && env::var_os("CARGO_CFG_MIRI").is_none(),
        "PX2 requires rustix's linux_raw backend; libc and Miri backends fail closed"
    );
    let manifest = fs::read_to_string("Cargo.toml").expect("read ken-host Cargo.toml");
    assert!(
        manifest.contains(
            "rustix = { version = \"=1.1.4\", default-features = false, features = [\"std\", \"fs\", \"process\", \"try_close\"] }"
        ),
        "PX2 manifest identity requires the exact audited rustix pin and features"
    );
    assert!(
        manifest.contains("libc = { version = \"=0.2.186\", default-features = false }"),
        "PX16 manifest identity requires the exact audited libc pin"
    );
    let workspace = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap())
        .parent()
        .and_then(Path::parent)
        .expect("ken-host is in crates/")
        .to_path_buf();
    let lock = fs::read_to_string(workspace.join("Cargo.lock")).expect("read workspace Cargo.lock");
    let dependencies = [
        package_identity(&lock, "rustix", "1.1.4", "std,fs,process,try_close"),
        package_identity(&lock, "bitflags", "2.13.0", ""),
        package_identity(&lock, "linux-raw-sys", "0.12.1", "std,general,errno"),
        package_identity(&lock, "libc", "0.2.186", ""),
    ];

    if target_os == "linux" {
        compile_abi_v1_companion(&target, &host);
    }

    let backend = "linux_raw";
    let facts = linux_raw_facts();
    verify_boundary_inventory(&facts);
    run_probe(&target, &host, &facts);

    let effect_layout = run_effect_abi_probe(&target, &host);
    let effect_catalog = parse_effect_catalog();
    write_host_effect_generated(&target, &effect_catalog, &effect_layout);

    let families = group_facts_by_family(&facts);
    let canonical = canonical_manifest(
        &target,
        &target_os,
        &target_arch,
        &target_endianness,
        backend,
        &dependencies,
        &families,
    );
    let hash: [u8; 32] = Sha256::digest(canonical.as_bytes()).into();
    write_generated(
        &target,
        &target_os,
        &target_arch,
        &target_endianness,
        backend,
        &dependencies,
        &facts,
        &families,
        &canonical,
        &hash,
    );
}

#[derive(Clone, Debug)]
struct EffectOp {
    name: String,
    id: u16,
    availability: String,
    request: String,
    request_arity: u8,
    reply: String,
    reply_arity: u8,
}

#[derive(Clone, Debug)]
struct EffectCatalog {
    operations: Vec<EffectOp>,
    bindings: Vec<(String, String, u64)>,
}

fn parse_effect_catalog() -> EffectCatalog {
    let source = fs::read_to_string("effect_abi_v1.catalog").expect("read effect ABI catalog");
    let mut operations = Vec::new();
    let mut bindings = Vec::new();
    for line in source.lines().filter(|line| !line.trim().is_empty()) {
        let fields = line.split('|').collect::<Vec<_>>();
        match fields[0] {
            "operation" => {
                assert_eq!(fields.len(), 8, "effect operation rows have eight fields");
                operations.push(EffectOp {
                    name: fields[1].to_string(),
                    id: u16::from_str_radix(fields[2], 16).expect("effect op id is hex"),
                    availability: fields[3].to_string(),
                    request: fields[4].to_string(),
                    request_arity: fields[5].parse().expect("request arity is u8"),
                    reply: fields[6].to_string(),
                    reply_arity: fields[7].parse().expect("reply arity is u8"),
                });
                let operation = operations.last().unwrap();
                assert!(
                    matches!(operation.availability.as_str(), "native" | "unavailable"),
                    "effect availability is closed"
                );
                assert!(
                    operation.request.ends_with("RequestV1")
                        && operation.reply.ends_with("ReplyV1"),
                    "effect wire records are named V1 records"
                );
            }
            "schema" => {
                assert_eq!(fields.len(), 2, "effect schema row has two fields");
                bindings.push((
                    fields[0].to_string(),
                    "version".to_string(),
                    fields[1].parse().expect("effect schema is u64"),
                ));
            }
            "lifetime" | "limit" | "tag" | "error" => {
                assert_eq!(fields.len(), 3, "effect binding rows have three fields");
                bindings.push((
                    fields[0].to_string(),
                    fields[1].to_string(),
                    fields[2].parse().expect("effect binding is u64"),
                ));
            }
            kind => panic!("unknown effect catalog row {kind}"),
        }
    }
    assert_eq!(operations.len(), 25, "HostOpV1 catalog is closed at 25");
    let mut ids = operations
        .iter()
        .map(|operation| operation.id)
        .collect::<Vec<_>>();
    ids.sort_unstable();
    ids.dedup();
    assert_eq!(ids.len(), operations.len(), "effect op ids are unique");
    let mut names = operations
        .iter()
        .map(|operation| operation.name.as_str())
        .collect::<Vec<_>>();
    names.sort_unstable();
    names.dedup();
    assert_eq!(names.len(), operations.len(), "effect op names are unique");
    let mut binding_keys = bindings
        .iter()
        .map(|(kind, name, _)| (kind, name))
        .collect::<Vec<_>>();
    binding_keys.sort_unstable();
    binding_keys.dedup();
    assert_eq!(
        binding_keys.len(),
        bindings.len(),
        "effect bindings are unique"
    );
    EffectCatalog {
        operations,
        bindings,
    }
}

fn run_effect_abi_probe(target: &str, host: &str) -> Vec<(String, u64)> {
    assert_eq!(target, host, "effect ABI headers attest only their target");
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let executable = out_dir.join("ken-host-effect-abi-probe");
    let compiler = cc::Build::new().target(target).host(host).get_compiler();
    let mut compile = compiler.to_command();
    compile
        .arg("-Wall")
        .arg("-Wextra")
        .arg("-Werror")
        .arg("effect_abi_probe.c")
        .arg("-o")
        .arg(&executable);
    assert!(compile
        .status()
        .expect("compile effect ABI probe")
        .success());
    let output = Command::new(executable)
        .output()
        .expect("run effect ABI probe");
    assert!(output.status.success(), "effect ABI probe failed closed");
    let stdout = String::from_utf8(output.stdout).expect("effect probe protocol is ASCII");
    let mut facts = stdout
        .lines()
        .map(|line| {
            let (name, value) = line
                .split_once('=')
                .expect("effect ABI probe emits NAME=INTEGER");
            assert!(
                !name.is_empty()
                    && name
                        .bytes()
                        .all(|byte| byte.is_ascii_alphanumeric() || byte == b'_'),
                "effect ABI fact name is closed ASCII"
            );
            (
                name.to_string(),
                value.parse::<u64>().expect("effect ABI fact is u64"),
            )
        })
        .collect::<Vec<_>>();
    facts.sort_by(|left, right| left.0.cmp(&right.0));
    facts
}

fn write_host_effect_generated(target: &str, catalog: &EffectCatalog, facts: &[(String, u64)]) {
    let mut canonical = format!("target={target}\n");
    for (kind, name, value) in &catalog.bindings {
        canonical.push_str(&format!("{kind}={name}|{value}\n"));
    }
    for operation in &catalog.operations {
        canonical.push_str(&format!(
            "operation={}|{:04x}|{}|{}|{}|{}|{}\n",
            operation.name,
            operation.id,
            operation.availability,
            operation.request,
            operation.request_arity,
            operation.reply,
            operation.reply_arity
        ));
    }
    for (name, value) in facts {
        canonical.push_str(&format!("layout={name}|{value}\n"));
    }
    let hash: [u8; 32] = Sha256::digest(canonical.as_bytes()).into();
    let fact_source = facts
        .iter()
        .map(|(name, value)| format!("({name:?}, {value}),"))
        .collect::<String>();
    let catalog_source = catalog
        .operations
        .iter()
        .map(|operation| {
            format!(
                "({:?}, {}, {:?}, {:?}, {}, {:?}, {}),",
                operation.name,
                operation.id,
                operation.availability,
                operation.request,
                operation.request_arity,
                operation.reply,
                operation.reply_arity
            )
        })
        .collect::<String>();
    let binding_source = catalog
        .bindings
        .iter()
        .map(|(kind, name, value)| format!("({kind:?}, {name:?}, {value}),"))
        .collect::<String>();
    let generated = format!(
        "pub const HOST_EFFECT_ABI_V1_CANONICAL: &str = {canonical:?};\n\
         pub const HOST_EFFECT_ABI_V1_HASH: [u8; 32] = {hash:?};\n\
         pub const HOST_EFFECT_ABI_V1_FACTS: &[(&str, u64)] = &[{fact_source}];\n\
         pub const HOST_EFFECT_ABI_V1_CATALOG: &[(&str, u16, &str, &str, u8, &str, u8)] = &[{catalog_source}];\n\
         pub const HOST_EFFECT_ABI_V1_BINDINGS: &[(&str, &str, u64)] = &[{binding_source}];\n"
    );
    fs::write(
        PathBuf::from(env::var("OUT_DIR").unwrap()).join("host_effect_abi_v1.rs"),
        generated,
    )
    .expect("write generated host effect ABI");
}

fn compile_abi_v1_companion(target: &str, host: &str) {
    cc::Build::new()
        .target(target)
        .host(host)
        .file("src/abi_v1/sigpipe.c")
        .warnings(true)
        .warnings_into_errors(true)
        .compile("ken_host_abi_v1_posture");
}

fn verify_boundary_inventory(facts: &[(&str, u64)]) {
    let build = fs::read_to_string("build.rs").expect("read landed ABI fact producer");
    let source = fs::read_to_string("src/lib.rs").expect("read landed ken-host producer");
    let consumer = fs::read_to_string("../ken-interp/src/eval.rs")
        .expect("read landed interpreter host-boundary consumer");
    let probe = fs::read_to_string("abi_probe.c").expect("read target ABI observer");
    build_support::verify_inventory_closure(&build, &source, &consumer, &probe, facts)
        .expect("producer, manifest, and observer ABI inventories must be identical");
}

fn package_identity(
    lock: &str,
    name: &str,
    version: &str,
    features: &str,
) -> (String, String, String, String) {
    for section in lock.split("[[package]]").skip(1) {
        let field = |key: &str| {
            section.lines().find_map(|line| {
                line.strip_prefix(&format!("{key} = \""))
                    .and_then(|value| value.strip_suffix('"'))
            })
        };
        if field("name") == Some(name) && field("version") == Some(version) {
            return (
                name.to_owned(),
                version.to_owned(),
                field("checksum")
                    .expect("registry dependency has checksum")
                    .to_owned(),
                features.to_owned(),
            );
        }
    }
    panic!("Cargo.lock lacks exact {name} {version}");
}

/// How one family-schema row claims facts.
///
/// Target identity is an exact compatibility vector: a broad `C_` prefix would
/// silently absorb a future record-layout fact. Open-ended constant families
/// use their closed namespace prefixes instead.
#[derive(Clone, Copy)]
enum FactSelector {
    Exact(&'static [&'static str]),
    Prefix(&'static [&'static str]),
}

impl FactSelector {
    fn claims(self, name: &str) -> bool {
        match self {
            Self::Exact(names) => names.contains(&name),
            Self::Prefix(prefixes) => prefixes.iter().any(|prefix| name.starts_with(prefix)),
        }
    }
}

const TARGET_IDENTITY_FACTS: &[&str] = &[
    "POINTER_WIDTH",
    "POINTER_ALIGNMENT",
    "C_CHAR_WIDTH",
    "C_CHAR_ALIGNMENT",
    "C_SHORT_WIDTH",
    "C_SHORT_ALIGNMENT",
    "C_INT_WIDTH",
    "C_INT_ALIGNMENT",
    "C_LONG_WIDTH",
    "C_LONG_ALIGNMENT",
    "C_LONG_LONG_WIDTH",
    "C_LONG_LONG_ALIGNMENT",
    "C_FLOAT_WIDTH",
    "C_FLOAT_ALIGNMENT",
    "C_DOUBLE_WIDTH",
    "C_DOUBLE_ALIGNMENT",
];

/// **`ABI-M1` `D1` -- the family schema, one row per `AbiFamily` variant.**
///
/// `build.rs` cannot import `ken_host::AbiFamily`, so it names each family by
/// the VARIANT PATH it emits into `OUT_DIR`. That text is compiler-checked
/// where `lib.rs` includes it: a family named here that the enum does not have
/// fails to compile, and a variant added to the enum without threading is
/// `error[E0004]` in `lib.rs`. Neither side keeps a private list the other can
/// drift from.
///
/// The selector assigns facts to families. Assignment is TOTAL and UNIQUE at
/// generation: a fact matching zero or multiple selectors aborts the build
/// rather than landing in a default or first-match family.
const FAMILY_SCHEMA: &[(&str, &str, u32, FactSelector)] = &[
    // (canonical name, emitted variant path, facility ABI version, fact selector)
    (
        "target_identity",
        "AbiFamily::TargetIdentity",
        1,
        FactSelector::Exact(TARGET_IDENTITY_FACTS),
    ),
    (
        "open_flags",
        "AbiFamily::OpenFlags",
        1,
        FactSelector::Prefix(&["O_"]),
    ),
    (
        "at_flags",
        "AbiFamily::AtFlags",
        1,
        FactSelector::Prefix(&["AT_"]),
    ),
    (
        "mode",
        "AbiFamily::Mode",
        1,
        FactSelector::Prefix(&["MODE_"]),
    ),
    (
        "syscall_number",
        "AbiFamily::SyscallNumber",
        1,
        FactSelector::Prefix(&["SYS_"]),
    ),
    (
        "errno",
        "AbiFamily::Errno",
        1,
        FactSelector::Prefix(&["ERRNO_"]),
    ),
];

/// Assign every fact to exactly one family, in schema order.
///
/// Fails closed in both directions: a fact claimed by no family would be
/// dropped, while a fact claimed by multiple families would be assigned by
/// schema order rather than identity. Both are malformed schema states.
fn group_facts_by_family<'a>(
    facts: &[(&'a str, u64)],
) -> Vec<(&'static str, &'static str, u32, Vec<(&'a str, u64)>)> {
    let mut grouped = FAMILY_SCHEMA
        .iter()
        .map(|(canonical, variant, version, _)| (*canonical, *variant, *version, Vec::new()))
        .collect::<Vec<_>>();
    for (name, value) in facts {
        let matches = FAMILY_SCHEMA
            .iter()
            .enumerate()
            .filter(|(_, (_, _, _, selector))| selector.claims(name))
            .map(|(index, _)| index)
            .collect::<Vec<_>>();
        assert_eq!(
            matches.len(),
            1,
            "ABI-M1: manifest fact {name} must belong to exactly one family; \
             matched family schema rows {matches:?}"
        );
        grouped[matches[0]].3.push((*name, *value));
    }
    for (canonical, _, _, selector) in FAMILY_SCHEMA {
        match selector {
            FactSelector::Exact(declared) => {
                for name in *declared {
                    let emitted = facts
                        .iter()
                        .filter(|(candidate, _)| candidate == name)
                        .count();
                    assert_eq!(
                        emitted, 1,
                        "ABI-M1: exact family {canonical} declares fact {name}, but the \
                         producer emitted it {emitted} times"
                    );
                }
            }
            FactSelector::Prefix(_) => {}
        }
    }
    grouped
}

/// One family's canonical projection -- the bytes its projection hash covers.
fn canonical_family_projection(
    canonical: &str,
    facility_version: u32,
    target_identity: Option<(&str, &str)>,
    facts: &[(&str, u64)],
) -> String {
    let mut out = format!("family={canonical}\nfacility_version={facility_version}\n");
    if let Some((target_arch, target_endianness)) = target_identity {
        out.push_str(&format!(
            "target_arch={target_arch}\ntarget_endianness={target_endianness}\n"
        ));
    }
    out.push_str(&format!("fact_count={}\n", facts.len()));
    for (name, value) in facts {
        out.push_str(&format!("fact={name}|{value}\n"));
    }
    out
}

#[cfg(target_os = "linux")]
fn linux_raw_facts() -> Vec<(&'static str, u64)> {
    use linux_raw_sys::{errno, general};
    vec![
        layout_fact("POINTER_WIDTH", bit_width::<*const core::ffi::c_void>()),
        layout_fact(
            "POINTER_ALIGNMENT",
            byte_alignment::<*const core::ffi::c_void>(),
        ),
        layout_fact("C_CHAR_WIDTH", bit_width::<core::ffi::c_char>()),
        layout_fact("C_CHAR_ALIGNMENT", byte_alignment::<core::ffi::c_char>()),
        layout_fact("C_SHORT_WIDTH", bit_width::<core::ffi::c_short>()),
        layout_fact("C_SHORT_ALIGNMENT", byte_alignment::<core::ffi::c_short>()),
        layout_fact("C_INT_WIDTH", bit_width::<core::ffi::c_int>()),
        layout_fact("C_INT_ALIGNMENT", byte_alignment::<core::ffi::c_int>()),
        layout_fact("C_LONG_WIDTH", bit_width::<core::ffi::c_long>()),
        layout_fact("C_LONG_ALIGNMENT", byte_alignment::<core::ffi::c_long>()),
        layout_fact("C_LONG_LONG_WIDTH", bit_width::<core::ffi::c_longlong>()),
        layout_fact(
            "C_LONG_LONG_ALIGNMENT",
            byte_alignment::<core::ffi::c_longlong>(),
        ),
        layout_fact("C_FLOAT_WIDTH", bit_width::<core::ffi::c_float>()),
        layout_fact("C_FLOAT_ALIGNMENT", byte_alignment::<core::ffi::c_float>()),
        layout_fact("C_DOUBLE_WIDTH", bit_width::<core::ffi::c_double>()),
        layout_fact(
            "C_DOUBLE_ALIGNMENT",
            byte_alignment::<core::ffi::c_double>(),
        ),
        ("O_RDONLY", general::O_RDONLY.into()),
        ("O_WRONLY", general::O_WRONLY.into()),
        ("O_RDWR", general::O_RDWR.into()),
        ("O_APPEND", general::O_APPEND.into()),
        ("O_CREAT", general::O_CREAT.into()),
        ("O_EXCL", general::O_EXCL.into()),
        ("O_TRUNC", general::O_TRUNC.into()),
        ("O_DIRECTORY", general::O_DIRECTORY.into()),
        ("O_NOFOLLOW", general::O_NOFOLLOW.into()),
        ("O_CLOEXEC", general::O_CLOEXEC.into()),
        ("AT_REMOVEDIR", general::AT_REMOVEDIR.into()),
        (
            "MODE_FILE_CREATE",
            (general::S_IRUSR
                | general::S_IWUSR
                | general::S_IRGRP
                | general::S_IWGRP
                | general::S_IROTH
                | general::S_IWOTH)
                .into(),
        ),
        (
            "MODE_DIRECTORY_CREATE",
            (general::S_IRWXU | general::S_IRWXG | general::S_IRWXO).into(),
        ),
        ("SYS_OPENAT", general::__NR_openat.into()),
        ("SYS_MKDIRAT", general::__NR_mkdirat.into()),
        ("SYS_UNLINKAT", general::__NR_unlinkat.into()),
        ("SYS_RENAMEAT", general::__NR_renameat.into()),
        ("SYS_READLINKAT", general::__NR_readlinkat.into()),
        ("SYS_FCHMOD", general::__NR_fchmod.into()),
        ("ERRNO_ENOENT", errno::ENOENT.into()),
        ("ERRNO_EEXIST", errno::EEXIST.into()),
    ]
}

#[cfg(target_os = "linux")]
fn bit_width<T>() -> u64 {
    (core::mem::size_of::<T>() * u8::BITS as usize) as u64
}

#[cfg(target_os = "linux")]
fn byte_alignment<T>() -> u64 {
    core::mem::align_of::<T>() as u64
}

#[cfg(target_os = "linux")]
fn layout_fact(name: &'static str, value: u64) -> (&'static str, u64) {
    (name, value)
}

#[cfg(not(target_os = "linux"))]
fn linux_raw_facts() -> Vec<(&'static str, u64)> {
    Vec::new()
}

fn run_probe(target: &str, host: &str, expected: &[(&str, u64)]) {
    assert_eq!(
        target, host,
        "system headers may only attest their own target"
    );
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let executable = out_dir.join("ken-host-abi-probe");
    let compiler = cc::Build::new().target(target).host(host).get_compiler();
    let mut compile = compiler.to_command();
    compile.arg("abi_probe.c").arg("-o").arg(&executable);
    let status = compile.status().expect("run target-qualified C compiler");
    assert!(
        status.success(),
        "target ABI probe compilation failed closed"
    );
    let output = Command::new(&executable)
        .output()
        .expect("run target ABI probe for the manifested target");
    assert!(
        output.status.success(),
        "target ABI probe execution failed closed"
    );
    let stdout = String::from_utf8(output.stdout).expect("probe protocol is ASCII");
    let observed = build_support::parse_probe(&stdout).expect("parse closed FACT=INTEGER protocol");
    let mut checked = expected.to_vec();
    if env::var_os("KEN_HOST_ABI_TEST_MISMATCH").is_some() {
        checked[0].1 ^= 1;
    }
    build_support::verify_probe(&checked, &observed)
        .expect("system headers disagree with linux-raw-sys");
}

/// **`ABI-M1` `D1` -- the v2 manifest hash COMPOSES per-family projections.**
///
/// v1 hashed one flat fact list, so every fact was in one undifferentiated
/// digest and nothing could say which family moved. v2 hashes each family's own
/// canonical projection, then hashes over those projection digests. That is
/// what makes `AC-2` checkable: mutating one family's fact flips exactly that
/// family's projection and the composed top hash, and no other family's.
///
/// The flat fact lines are deliberately NOT repeated here -- they are covered
/// transitively through the projections. Listing them twice would still hash
/// correctly and would quietly make the composition decorative.
fn canonical_manifest(
    target: &str,
    target_os: &str,
    target_arch: &str,
    target_endianness: &str,
    backend: &str,
    dependencies: &[(String, String, String, String)],
    families: &[(&str, &str, u32, Vec<(&str, u64)>)],
) -> String {
    let mut out = format!(
        "schema={SCHEMA_VERSION}\ntarget={target}\ntarget_os={target_os}\nbackend={backend}\n"
    );
    for (name, version, checksum, features) in dependencies {
        out.push_str(&format!(
            "dependency={name}|{version}|{checksum}|{features}\n"
        ));
    }
    out.push_str(&format!("family_count={}\n", families.len()));
    for (canonical, _variant, facility_version, facts) in families {
        let target_identity =
            (*canonical == "target_identity").then_some((target_arch, target_endianness));
        let projection =
            canonical_family_projection(canonical, *facility_version, target_identity, facts);
        let digest: [u8; 32] = Sha256::digest(projection.as_bytes()).into();
        out.push_str(&format!(
            "family={canonical}|{facility_version}|{}|{}\n",
            facts.len(),
            hex_lower(&digest)
        ));
    }
    out
}

fn hex_lower(bytes: &[u8; 32]) -> String {
    let mut out = String::with_capacity(64);
    for byte in bytes {
        out.push_str(&format!("{byte:02x}"));
    }
    out
}

fn write_generated(
    target: &str,
    target_os: &str,
    target_arch: &str,
    target_endianness: &str,
    backend: &str,
    dependencies: &[(String, String, String, String)],
    facts: &[(&str, u64)],
    families: &[(&str, &str, u32, Vec<(&str, u64)>)],
    canonical: &str,
    hash: &[u8; 32],
) {
    let dependencies = dependencies.iter().map(|(name, version, checksum, features)| {
        format!("DependencyIdentity {{ name: {name:?}, version: {version:?}, checksum: {checksum:?}, features: &{:?} }},", features.split(',').filter(|feature| !feature.is_empty()).collect::<Vec<_>>())
    }).collect::<String>();
    let facts = facts
        .iter()
        .map(|(name, value)| format!("AbiFact {{ name: {name:?}, value: {value} }},"))
        .collect::<String>();
    // `ABI-M1` `D1` -- each family emits its own projection hash. The variant
    // path comes from FAMILY_SCHEMA and is compiler-checked where lib.rs
    // includes this file, so a family this generator names that the enum does
    // not have fails to compile rather than landing as data.
    let families = families
        .iter()
        .map(|(canonical_name, variant, facility_version, members)| {
            let target_identity = (*canonical_name == "target_identity")
                .then_some((target_arch, target_endianness));
            let projection = canonical_family_projection(
                canonical_name,
                *facility_version,
                target_identity,
                members,
            );
            let digest: [u8; 32] = Sha256::digest(projection.as_bytes()).into();
            let member_facts = members
                .iter()
                .map(|(name, value)| format!("AbiFact {{ name: {name:?}, value: {value} }},"))
                .collect::<String>();
            format!(
                "AbiFamilyProjection {{ family: {variant}, facility_version: {facility_version}, facts: &[{member_facts}], projection_hash: {digest:?} }},"
            )
        })
        .collect::<String>();
    let generated = format!(
        "pub const TARGET_ABI_CANONICAL: &str = {canonical:?};\n\
         pub const TARGET_ABI_MANIFEST_HASH: [u8; 32] = {hash:?};\n\
         pub const TARGET_ABI: TargetAbi = TargetAbi {{ schema_version: {SCHEMA_VERSION}, target: {target:?}, target_os: {target_os:?}, target_arch: {target_arch:?}, target_endianness: {target_endianness:?}, backend: {backend:?}, dependencies: &[{dependencies}], fact_count: {fact_count}, facts: &[{facts}], families: &[{families}], manifest_hash: TARGET_ABI_MANIFEST_HASH }};\n",
        fact_count = facts.matches("AbiFact").count(),
    );
    fs::write(
        PathBuf::from(env::var("OUT_DIR").unwrap()).join("target_abi.rs"),
        generated,
    )
    .expect("write generated TargetAbi module");
}
