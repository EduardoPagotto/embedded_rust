# Rust Embedded RISCV32IAMC

O que é Embedded rust

- Projetos sem standard #![no_std] library (libstd)
- Usa o libcore
- libcore é leve sem dependencias com o Sistema Operacional

## Instalação
Tools
```bash
# instalaão de app de dev
cargo search cargo-objdump
cargo install cargo-binutils
rustup component add llvm-toolscargo
```

Target Arch
```bash
# Exibe target disponiveis
rustc --print target-list

# Instala target riscv32imac-unknown-none-elf
rustup target add riscv32imac-unknown-none-elf
```

## Worksapce
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

## Setup do ./cargo/config
```bash
mkdir .cargo
touch .cargo/config
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
# llvm-objdump --arch-name=riscv32 -dC target/riscv32imac-unknown-none-elf/debug/librtos.rlib

rust-objdump  -dC target/riscv32imac-unknown-none-elf/debug/
rust-objdump  -dC target/riscv32imac-unknown-none-elf/debug/librtos.rlib
```


refs:
- https://doc.rust-lang.org/core/
- https://doc.rust-lang.org/std/index.html
- https://doc.rust-lang.org/reference/attributes.html
