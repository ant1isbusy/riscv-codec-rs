# RISC-V 32-bit instruction encode/decode tool

A simple tool to encode and decode RISC-V 32-bit instructions.

![alt text](img/preview.png)

### Installation

Using cargo:

```sh
cargo install --git https://github.com/ant1isbusy/riscv-codec-rs
```

To use, call:

```sh
rv-codec
```

##### TODO

- add more instructions: fence, ecall, ebreak missing for RV32I, then add RV32M and RV64I: https://msyksphinz-self.github.io/riscv-isadoc
- add verbose option which shows how register fields are arranged.
