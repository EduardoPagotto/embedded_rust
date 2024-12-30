# Rust Embedded RISCV32IAMC

- Projetos sem standard #![no_std] library (libstd)
- Usa o libcore
- libcore é leve sem dependencias com o Sistema Operacional
- Processadores RISCV

## Instalação
Tools:
```bash
# instalaão de app de dev
cargo search cargo-objdump
cargo install cargo-binutils
rustup component add llvm-toolscargo
```

Opcional em Fedora 41, instalação LLVM
```bash
# LLVM / LLDM /CMAKE
sudo dnf install clang clang-tools-extra
sudo dnf install cmake cmake-data cmake-rpm-macros jsoncpp libstdc++-static llvm-static llvm-devel llvm-test autoconf automake
sudo dnf install lldb lldb-devel
sudo dnf install compiler-rt
```

Target Arch:
```bash
# Exibe target disponiveis
rustc --print target-list

# Instala target riscv32imac-unknown-none-elf
rustup target add riscv32imac-unknown-none-elf
```

## Workspace
```bash
# Cria diretorio de worksapce
mkdir wsbootstrap
cd wsbootstrap

# cria packages
cargo new --bin kernel
cargo new --lib rtos

# build all
cargo build

# dump na lib
rust-objdump -dC target/debug/librtos.rlib

# size
rust-size target/debug/kernel

# apaga o target
cargo clean
```

## Setup do ./cargo/config.toml
```bash
mkdir .cargo
touch .cargo/config.toml
```

.cargo/config
```file
[build]
target = "riscv32imac-unknown-none-elf"
rustflags = [
    "-C", "link-dead-code",
    "-C", "link-args=-Thifive1-revb-rom.lscript"
]

```

Teste do binario mostra assembler RISCV32
```bash

# Com o LLVM

# llvm-objdump --arch-name=riscv32 -dC target/riscv32imac-unknown-none-elf/debug/kernel
rust-objdump  -dC target/riscv32imac-unknown-none-elf/debug/kernel

# llvm-objdump --arch-name=riscv32 -dC target/riscv32imac-unknown-none-elf/debug/librtos.rlib
rust-objdump  -dC target/riscv32imac-unknown-none-elf/debug/librtos.rlib
```

refs:
- https://doc.rust-lang.org/core/
- https://doc.rust-lang.org/std/index.html
- https://doc.rust-lang.org/reference/attributes.html
