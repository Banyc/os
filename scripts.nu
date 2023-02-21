# Usage:
#   source scripts.nu
#   build

def build [] {
    cargo build
    # rust-objcopy target/riscv64gc-unknown-none-elf/debug/main --strip-all -O binary target/riscv64gc-unknown-none-elf/debug/main.bin
}

def install-tools [] {
    rustup target add riscv64gc-unknown-none-elf
    cargo install cargo-binutils
    rustup component add llvm-tools-preview
    brew install qemu
    brew install gdb
}

def readelf [] {
    rust-objdump -d target/riscv64gc-unknown-none-elf/debug/main
}

def run [] {
    build
    qemu-system-riscv64 -M virt -kernel target/riscv64gc-unknown-none-elf/debug/main -nographic
}

def debug [] {
    build
    qemu-system-riscv64 -M virt -kernel target/riscv64gc-unknown-none-elf/debug/main -nographic -s -S
}

def ll-db [] {
    lldb target/riscv64gc-unknown-none-elf/debug/main -o "gdb-remote localhost:1234"
}

def g-db [] {
    rust-gdb target/riscv64gc-unknown-none-elf/debug/main -ex "target remote localhost:1234"
}
