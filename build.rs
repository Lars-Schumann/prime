use std::{env, fs, path::PathBuf};

const SHIFT: u32 = 8;

fn main() {
    println!("cargo::rerun-if-changed=build.rs");

    let to_bytes = match env::var("CARGO_CFG_TARGET_ENDIAN").unwrap().as_str() {
        "little" => u32::to_le_bytes,
        "big" => u32::to_be_bytes,
        other => panic!("unsupported target endianness: {other}"),
    };

    let primes: Vec<u32> = primal::Primes::all()
        .map_while(|p| u32::try_from(p).ok())
        .collect();
    let pi: Vec<u32> = (0..=1u64 << (32 - SHIFT))
        .map(|k| primes.partition_point(|&p| (p as u64) < k << SHIFT) as u32)
        .collect();

    // Bit `n % 8` of byte `n / 8` is set iff `n` is prime. Byte-granular, so endianness-free.
    let mut bitset = vec![0u8; 1 << (32 - 3)];
    for &p in &primes {
        bitset[(p >> 3) as usize] |= 1 << (p & 7);
    }

    let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    fs::write(out_dir.join("bitset.bin"), bitset).unwrap();
    for (name, data) in [("primes.bin", primes), ("pi.bin", pi)] {
        let bytes: Vec<u8> = data.into_iter().flat_map(to_bytes).collect();
        fs::write(out_dir.join(name), bytes).unwrap();
    }
}
