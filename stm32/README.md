# 組み込み for STM32

Add build target ARMv7E-M.
```shell
rustup target add thumbv7em-none-eabi
```

install Cortex-M4 binary tools.
```shell
cargo install cargo-binutils
rustup component add llvm-tools-preview
```

Build qemu docker image
```shell
# platform flag for Apple Silicon Mac
docker build --platform linux/x86_64 -t qemu .
```

Check version of qemu docker container.
```shell
docker run --platform linux/x86_64 --rm qemu qemu-system-gnuarmeclipse --version
```

If there is no problems with qemu emulator, flash the STM32 board(STM32F407)

```shell
git clone https://github.com/florisla/stm32loader.git ~/.stm32loader
cd ~/.stm32loader
pip install -e .
```

## Read more
* [cortex-m-rtic](https://rtic.rs)
* [svd2rust](https://github.com/rust-embedded/svd2rust/)
* [Awesome-embedded-rust](https://github.com/rust-embedded/awesome-embedded-rust#peripheral-access-crates)
* [Embedded devices Working Group](https://github.com/rust-embedded/wg)
* [The Embedded Rust Book](https://rust-embedded.github.io/book)
* [Discovery](https://rust-embedded.github.io/discovery)
* [The Embedonomicon](https://docs.rust-embedded.org/embedonomicon)

## Rust and OS
* [Redox](https://www.redox-os.org)
* [Fuchsia](https://fuchsia.dev)
* [Tock](https://www.tockos.org)
* [OpenTitan](https://opentitan.org)
* [Writing an OS in Rust](https://os.phil-opp.com/)