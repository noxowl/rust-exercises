# Practial Cargo

## Structure of project directory
Default structure of project.
```shell
┝Cargo.toml
└src
  └main.rs
```
All directories of project.
```shell
┝benches
┃ ┝multi_bench
┃ ┃ └main.rs
┃ └single_bench.rs
┝Cargo.toml
┝examples
┃ ┝multi_example
┃ ┃ └main.rs
┃ └single_example.rs
┝src
┃ ┝bin
┃ ┃ ┝multi_bin
┃ ┃ ┃ └main.rs
┃ ┃ └single_bin.rs
┃ └main.rs
└tests
  ┝multi_test
  ┃ └main.rs
  └single_test.rs
```
### src/bin
### tests
```shell
cargo test --test single_test
cargo test --test multi_test
```

### examples
```shell
cargo run --example single_example
cargo run --example multi_example
```

### benches
```shell
cargo bench --bench single_bench
cargo bench --bench multi_bench
```

### Feature
```shell
cargo new --lib library
cargo new app
cd library
vim Cargo.toml
```
```toml
# toml:library/Cargo.toml
[features]
default = ["parallel", "serde"]

special = []

parallel = ["crossbeam"]

nightly = ["crossbeam/nightly"]

full = ["crossbeam/nightly", "parallel", "serde"]

[dependencies]
crossbeam = {version = "0.7.3", optional = true}
serde = {version = "1.0.111", optional = true}
```
Write some test in lib.rs and execute cargo test.
```shell
cargo test
---
running 2 tests
test tests::test_parallel ... ok
test tests::test_serde ... ok

cargo test --features special
---
running 3 tests
test tests::test_serde ... ok
test tests::test_special ... ok
test tests::test_parallel ... ok

cargo test --no-default-features
---
running 0 tests
```

## Workspace
```shell
mkdir sample
cd sample
cargo new main
```
```toml
# toml:main/Cargo.toml
[workspace]

members = [
    "main"
]
```
```shell
cargo new sub --lib
```
```toml
# toml:main/Cargo.toml
[workspace]

members = [
    "main",
    "sub"
]
```
```rust
// sub/src/lib.rs
pub fn hello() {
    println!("Hello");
}
```
```toml
# toml:main/Cargo.toml
[dependencies]
sub = {path = "../sub"}
```
```rust
// main/src/main.rs
use sub::hello;

fn main() {
    hello();
}
```
```toml
# toml:main/Cargo.toml
[dependencies]
sub = {path = "../sub", version = "0.1.0"}
```

## Subcommands
* cargo-add
* cargo-outdated
* cargo-release
* cargo-watch

## Formatter, Linter
### rustfmt
```shell
cargo fmt -- --check
```
```shell
cargo clippy
```

## Code Coverage
### cargo-tarpaulin
```shell
cargo install cargo-trapaulin
cargo trapaulin
```

## Benchmark, Profiler
### cargo bench
Only works in nightly (Rust 1.44)
```rust
#![feature(test)]

pub fn hash(x: u64) -> u64 {
    let mut y = x;
    for _ in 0..512 {
        y = y << 5;
        y = y ^ x;
    }
    y
}

#[cfg(test)]
mod tests {
    extern crate test;
    
    use super::*;
    use test::Bencher;
    
    #[bench]
    fn bench_hash(b: &mut Bencher) {
        b.iter(|| {
            let n = test::black_box(2);
            hash(n)
        });
    }
}
```
```shell
cargo +nightly bench 
```

### Benchmark as test method
```rust
#![feature(test)]

extern crate test;

use sample::hash;
use test::Bencher;

#[bench]
fn bench_hash(b: &mut Bencher) {
    b.iter(|| {
        let n = test::black_box(2);
        hash(n)
    });
}
```
```shell
cargo +nightly bench
```

### criterion
```shell
cargo new --lib sample
cargo add --dev criterion@0.3.4
```
```toml
# Cargo.toml
[[bench]]
name = "benchmark"
harness = false
```
```rust
// benches/benchmark.rs
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use sample::hash;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("hash 2", |b| {
        b.iter(|| {
            let n = black_box(2);
            hash(n)
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
```
```shell
cargo bench
```

### cargo-profiler
```shell
sudo apt-get install valgrind
cargo install cargo-profiler
```
```shell
cargo new sample
cd sample
cargo build --release
```
```shell
cargo profiler callgrind --bin ./target/release/sample
```

### flamegraph
```shell
sudo apt install -y linux-tools-common linux-tools-generic
cargo install flamegraph
```
```shell
cargo new sample
cd sample
cargo add rand@0.8.3
```
```rust
// src/main.rs
use rand::Rng;

fn fn0(x: u64) -> u64 {
    if x % 2 == 0 {
        fn2(x)
    } else {
        fn3(x)
    }
}

fn fn1(x: u64) -> u64 {
    if x % 2 == 0  {
        fn4(x)
    } else {
        fn5(x)
    }
}

fn fn2(x: u64) -> u64 {
    repeat(x, |x| x / 3 + 20)
}

fn fn3(x: u64) -> u64 {
    repeat(x, |x| x / 5 / 7 + 10)
}

fn fn4(x: u64) -> u64 {
    repeat(x, |x| x * 11 + 20)
}

fn fn5(x: u64) -> u64 {
    repeat(x, |x| x * 13 * 5 + 10)
}

fn repeat(x: u64, f: F) -> u64 {
where
    F: Fn(u64) -> u64,
    {
        let mut y = x;
        for _ in 0..100 {
            y = f(y);
        }
    }
    y
}

fn main() {
    let mut x = 1;
    let mut rng = rand::thread_rng();
    
    for _ in 0..1000000000 {
        let i: u64 = rng.gen();
        match i % 6 {
            0 => x = fn0(x),
            1 => x = fn1(x),
            2 => x = fn2(x),
            3 => x = fn3(x),
            4 => x = fn4(x),
            5 => x = fn5(x),
            _ => ()
        } 
    }
    
    println!("{}", x)
}
```
```toml
# Cargo.toml
[profile.release]
debug = true
```
```shell
cargo flamegraph
```

### hyperfine
```shell
cargo install hyperfine
hyperfine "program1 data.txt" "program2 data.txt                                                                                                 "
```

## Read more
* CI with Travis/Github Action
* Link Time Optimization (LTO)
* Profile-Guided Optimization
* Code security
  * rustsec.org
  * cargo-audit
  * cargo-crev
* Fuzzing
  * cargo-fuzz
* private crate
  * [package] publish = false // cargo publish <- can't publish