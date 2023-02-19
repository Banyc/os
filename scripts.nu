# Usage:
#   source scripts.nu
#   build

def build [] {
    cargo build
    # rust-objcopy target/riscv64gc-unknown-none-elf/debug/os --strip-all -O binary target/riscv64gc-unknown-none-elf/debug/os.bin
}

def install-tools [] {
    rustup target add riscv64gc-unknown-none-elf
    cargo install cargo-binutils
    rustup component add llvm-tools-preview
    brew install qemu
}

def readelf [] {
    rust-objdump -d target/riscv64gc-unknown-none-elf/debug/os
}

def run [] {
    build
    qemu-system-riscv64 -M virt -kernel target/riscv64gc-unknown-none-elf/debug/os -nographic
}

def debug [] {
    build
    qemu-system-riscv64 -M virt -kernel target/riscv64gc-unknown-none-elf/debug/os -nographic -s -S
}

def ll-db [] {
    lldb target/riscv64gc-unknown-none-elf/debug/os -o "gdb-remote localhost:1234"
}
