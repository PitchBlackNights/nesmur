use std::{
    env,
    fs::File,
    io::{BufWriter, Write},
    path::{Path, PathBuf},
};
use vergen_git2::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // ======== ENV VARIABLES ========
    //  VERGEN_BUILD_TIMESTAMP
    //  VERGEN_CARGO_TARGET_TRIPLE
    //  VERGEN_GIT_BRANCH
    //  VERGEN_GIT_COMMIT_TIMESTAMP
    //  VERGEN_GIT_SHA
    //  VERGEN_RUSTC_CHANNEL
    //  VERGEN_RUSTC_COMMIT_DATE
    //  VERGEN_RUSTC_COMMIT_HASH
    //  VERGEN_RUSTC_SEMVER

    let build = BuildBuilder::default().build_timestamp(true).build()?;
    let cargo = CargoBuilder::default().target_triple(true).build()?;
    let git2 = Git2Builder::default()
        .branch(true)
        .commit_timestamp(true)
        .sha(false)
        .build()?;
    let rustc = RustcBuilder::default()
        .channel(true)
        .commit_date(true)
        .commit_hash(true)
        .semver(true)
        .build()?;

    Emitter::default()
        .add_instructions(&build)?
        .add_instructions(&cargo)?
        .add_instructions(&git2)?
        .add_instructions(&rustc)?
        .emit_and_set()?;

    let env_list: Vec<&str> = vec![
        "VERGEN_BUILD_TIMESTAMP",
        "VERGEN_CARGO_TARGET_TRIPLE",
        "VERGEN_GIT_SHA",
        "VERGEN_GIT_COMMIT_TIMESTAMP",
        "VERGEN_GIT_BRANCH",
        "VERGEN_RUSTC_CHANNEL",
        "VERGEN_RUSTC_COMMIT_DATE",
        "VERGEN_RUSTC_COMMIT_HASH",
        "VERGEN_RUSTC_SEMVER",
    ];

    let path: PathBuf = Path::new(&env::var("OUT_DIR").unwrap()).join("codegen.rs");
    let mut file: BufWriter<File> = BufWriter::new(File::create(&path).unwrap());

    let mut hashmap_values: Vec<(&str, String)> = Vec::with_capacity(env_list.capacity());
    for environ in env_list {
        if let Ok(value) = env::var(environ) {
            hashmap_values.push((environ, value));
        }
    }

    writeln!(
        &mut file,
        "use std::collections::HashMap;
        use std::sync::LazyLock;
        pub static ENV_VARS: LazyLock<HashMap<&'static str, &'static str>> = LazyLock::new(|| HashMap::from({:?}));",
        hashmap_values
    )
    .unwrap();
    file.flush().unwrap();
    Ok(())
}
