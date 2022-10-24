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