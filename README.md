# OS

- target triple
  - e.g.: `x86_64-unknown-linux-gnu`
    - `x86_64`: CPU architecture
    - `unknown`: vendor
    - `linux`: OS
    - `gnu`:
      - ABI
      - uses `glibc` (GNU C library)
  - e.g.: `thumbv7em-none-eabihf`
    - `thumbv7em`:
      - ARMv7 CPU architecture
      - `em`: for embedded systems
    - `none`:
      - OS
      - not running on any OS
    - `eabihf`:
      - ABI
      - "Embedded ABI, hard float"

## References

- x86: <https://os.phil-opp.com/>
- RISC-V OS demo: <https://github.com/dazhi0619/myos>
- QEMU RISC-V emulator: <https://www.qemu.org/docs/master/system/target-riscv.html>
- RISC-V SBI Spec: <https://github.com/riscv-non-isa/riscv-sbi-doc/blob/master/riscv-sbi.adoc>
- RISC-V: <https://osblog.stephenmarz.com/index.html>
