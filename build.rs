use std::env;
use std::path::{Path, PathBuf};

use coin_build_tools::{coinbuilder, link, utils};

const LIB_NAME: &str = "Bonmin";

fn main() {
    println!(
        "cargo:rerun-if-changed={}_lib_sources.txt",
        LIB_NAME.to_ascii_lowercase()
    );
    println!(
        "cargo:rerun-if-env-changed=CARGO_{}_STATIC",
        LIB_NAME.to_ascii_uppercase()
    );
    println!(
        "cargo:rerun-if-env-changed=CARGO_{}_SYSTEM",
        LIB_NAME.to_ascii_uppercase()
    );

    if want_system && link::link_lib_system_if_supported(LIB_NAME) {
        let mut coinflags = vec!["BONMIN".to_string()];

        if cfg!(feature = "filtersqp") {
            coinflags.push("FILTERSQP".to_string());
        }
        if cfg!(feature = "cplex") {
            coinflags.push("CPX".to_string());
        }
        let (_, coinflags_other) = coinbuilder::get_metadata_from("Cbc");
        coinflags.extend(coinflags_other);
    
        let (_, coinflags_other) = coinbuilder::get_metadata_from("Ipopt");
        coinflags.extend(coinflags_other);
    
        coinbuilder::print_metadata(includes_dir.clone(), coinflags.clone());

        coinbuilder::print_metadata(Vec::new(), coinflags);
        return;
    }

    if !Path::new(&format!("{}/LICENSE", LIB_NAME)).exists() {
        utils::update_submodules(env::var("CARGO_MANIFEST_DIR").unwrap());
    }
    build_lib_and_link();
}

fn build_lib_and_link() {
    let src_dir = format!(
        "{}",
        PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap())
            .join(LIB_NAME)
            .join(LIB_NAME)
            .join("src")
            .display()
    );

    let src_dir_interface = format!(
        "{}",
        PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap())
            .join("BonminCInterface")
            .join("src")
            .display()
    );

    let mut includes_dir = vec![
        format!("{}/Algorithms", src_dir),
        format!("{}/Algorithms/Branching", src_dir),
        format!("{}/Algorithms/OaGenerators", src_dir),
        format!("{}/Algorithms/QuadCuts", src_dir),
        format!("{}/Apps/", src_dir),
        format!("{}/CbcBonmin", src_dir),
        format!("{}/CbcBonmin/Heuristics", src_dir),
        format!("{}/Interfaces", src_dir),
        format!("{}/Interfaces/Ipopt", src_dir),
    ];

    let mut lib_sources = include_str!("bonmin_lib_sources.txt")
        .trim()
        .split('\n')
        .map(|file| format!("{}/{}", src_dir, file.trim()))
        .collect::<Vec<String>>();

    includes_dir.push(src_dir_interface.to_string());
    lib_sources.extend(vec![
        format!("{}/BonStdCInterface.cpp", src_dir_interface),
        format!("{}/BonStdInterfaceTMINLP.cpp", src_dir_interface),
    ]);

    let mut coinflags = vec!["Bonmin".to_string()];

    if cfg!(feature = "filtersqp") {
        includes_dir.push(format!("{}/Interfaces/Filter", src_dir));
        lib_sources.extend(vec![
            format!("{}/Interfaces/Filter/BonBqpdSolver.cpp", src_dir),
            format!("{}/Interfaces/Filter/BonBqpdWarmStart.cpp", src_dir),
            format!("{}/Interfaces/Filter/BonFilterSolver.cpp", src_dir),
            format!("{}/Interfaces/Filter/BonFilterWarmStart.cpp", src_dir),
        ]);

        coinflags.push("FILTERSQP".to_string());
    }
    if cfg!(feature = "cplex") {
        coinflags.push("CPX".to_string());
    }

    let (include_other, coinflags_other) = coinbuilder::get_metadata_from("Cbc");
    includes_dir.extend(include_other);
    coinflags.extend(coinflags_other);

    let (include_other, coinflags_other) = coinbuilder::get_metadata_from("Ipopt");
    includes_dir.extend(include_other);
    coinflags.extend(coinflags_other);

    coinbuilder::print_metadata(includes_dir.clone(), coinflags.clone());

    let mut config = coinbuilder::init_builder();
    coinflags.iter().for_each(|flag| {
        config.define(&format!("COIN_HAS_{}", flag), None);
    });
    config.define("BONMINLIB_BUILD", None);
    config.includes(includes_dir);
    config.files(lib_sources);

    config.compile(LIB_NAME);
}
