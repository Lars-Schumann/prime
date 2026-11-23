use core::hint::black_box;

use criterion::{Criterion, Throughput, criterion_group, criterion_main};
use prime::{is_prime, is_prime_smort};

fn splitmix64(state: &mut u64) -> u64 {
    *state = state.wrapping_add(0x9E37_79B9_7F4A_7C15);
    let mut z = *state;
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^ (z >> 31)
}

const N: usize = 1 << 16;

fn uniform_inputs() -> Vec<u32> {
    let mut state = 0;
    (0..N)
        .map(|_| (splitmix64(&mut state) >> 32) as u32)
        .collect()
}

fn bench_with(c: &mut Criterion, group_name: &str, f: impl Fn(u32) -> bool) {
    let inputs = uniform_inputs();

    let mut group = c.benchmark_group(group_name);
    group.throughput(Throughput::Elements(N as u64));
    group.bench_function("uniform_u32", |b| {
        b.iter(|| {
            black_box(&inputs)
                .iter()
                .filter(|&&n| f(black_box(n)))
                .count()
        })
    });
    group.finish();
}

fn bench_is_prime(c: &mut Criterion) {
    bench_with(c, "is_prime", is_prime);
}

fn bench_is_prime_smort(c: &mut Criterion) {
    bench_with(c, "is_prime_interpolation", is_prime_smort);
}

criterion_group!(benches, bench_is_prime, bench_is_prime_smort);
criterion_main!(benches);
