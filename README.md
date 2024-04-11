![Rust Apps on Apache NuttX RTOS and QEMU RISC-V](https://lupyuen.github.io/images/rust3-title.png)

# Rust Apps for Apache NuttX RTOS and QEMU RISC-V

Read the article...

- ["Rust Apps on Apache NuttX RTOS and QEMU RISC-V"](https://lupyuen.github.io/articles/rust3)

# Rust Custom Target for QEMU RISC-V on Apache NuttX RTOS

TODO

https://docs.rust-embedded.org/embedonomicon/custom-target.html

```bash
$ rustc \
  +nightly \
  -Z unstable-options \
  --print target-spec-json \
  --target riscv32i-unknown-none-elf

{
  "arch": "riscv32",
  "atomic-cas": false,
  "cpu": "generic-rv32",
  "data-layout": "e-m:e-p:32:32-i64:64-n32-S128",
  "eh-frame-header": false,
  "emit-debug-gdb-scripts": false,
  "is-builtin": true,
  "linker": "rust-lld",
  "linker-flavor": "ld.lld",
  "llvm-target": "riscv32",
  "max-atomic-width": 0,
  "panic-strategy": "abort",
  "relocation-model": "static",
  "target-pointer-width": "32"
}

$ rustc \
  +nightly \
  -Z unstable-options \
  --print target-spec-json \
  --target riscv64gc-unknown-none-elf  

{
  "arch": "riscv64",
  "code-model": "medium",
  "cpu": "generic-rv64",
  "data-layout": "e-m:e-p:64:64-i64:64-i128:128-n32:64-S128",
  "eh-frame-header": false,
  "emit-debug-gdb-scripts": false,
  "features": "+m,+a,+f,+d,+c",
  "is-builtin": true,
  "linker": "rust-lld",
  "linker-flavor": "ld.lld",
  "llvm-abiname": "lp64d",
  "llvm-target": "riscv64",
  "max-atomic-width": 64,
  "panic-strategy": "abort",
  "relocation-model": "static",
  "supported-sanitizers": [
    "kernel-address"
  ],
  "target-pointer-width": "64"
}
```

TODO: riscv32gc-unknown-none-elf

```json
{
  "arch": "riscv32",
  "cpu": "generic-rv32",
  "data-layout": "e-m:e-p:32:32-i64:64-n32-S128",
  "eh-frame-header": false,
  "emit-debug-gdb-scripts": false,
  "features": "+m,+a,+f,+d,+c",
  "is-builtin": false,
  "linker": "rust-lld",
  "linker-flavor": "ld.lld",
  "llvm-target": "riscv32",
  "max-atomic-width": 0,
  "panic-strategy": "abort",
  "relocation-model": "static",
  "target-pointer-width": "32"
}
```

- Removed `"atomic-cas": false`

- Added `"features": "+m,+a,+f,+d,+c"`

- Changed to `"is-builtin": false`
