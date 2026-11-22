use std::{env, fs, path::PathBuf};

fn main() {
    println!("cargo::rerun-if-changed=build.rs");

    let to_bytes = match env::var("CARGO_CFG_TARGET_ENDIAN").unwrap().as_str() {
        "little" => u32::to_le_bytes,
        "big" => u32::to_be_bytes,
        other => panic!("unsupported target endianness: {other}"),
    };

    let bytes: Vec<u8> = primal::Primes::all()
        .map_while(|p| u32::try_from(p).ok())
        .flat_map(to_bytes)
        .collect();

    let path = PathBuf::from(env::var_os("OUT_DIR").unwrap()).join("primes.bin");
    fs::write(path, bytes).unwrap();
}
