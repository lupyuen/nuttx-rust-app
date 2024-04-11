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

TODO: riscv32gc-unknown-none-elf.json

```json
{
  "arch": "riscv32",
  "cpu": "generic-rv32",
  "data-layout": "e-m:e-p:32:32-i64:64-n32-S128",
  "eh-frame-header": false,
  "emit-debug-gdb-scripts": false,
  "features": "+m,+a,+f,+d,+c",
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

- Removed `"is-builtin": true`

TODO: Compile Rust App

```bash
rustc \
  --print cfg \
  --target riscv32gc-unknown-none-elf.json

cargo new app
pushd app
cargo build \
  -Zbuild-std=core,alloc \
  --target ../riscv32gc-unknown-none-elf.json
popd

## Changed target to riscv32gc-unknown-none-elf.json
# rustc \
#   --edition 2021 \
#   --emit obj \
#   -g \
#   --target riscv32gc-unknown-none-elf.json \
#   -C panic=abort \
#   -O \
#   ../apps/examples/hello_rust/hello_rust_main.rs \
#   -o ../apps/examples/hello_rust/*hello_rust.o

rustc \
  --edition 2021 \
  --emit obj \
  -g \
  --target /Users/Luppy/riscv/nuttx-rust-app/riscv32gc-unknown-none-elf.json \
  -C panic=abort \
  -O \
  ../apps/examples/hello_rust/hello_rust_main.rs \
  -o ../apps/examples/hello_rust/*hello_rust.o \
  \
  -C incremental=/Users/Luppy/riscv/nuttx-rust-app/app/target/riscv32gc-unknown-none-elf/debug/incremental \
  -L dependency=/Users/Luppy/riscv/nuttx-rust-app/app/target/riscv32gc-unknown-none-elf/debug/deps \
  -L dependency=/Users/Luppy/riscv/nuttx-rust-app/app/target/debug/deps \
  --extern 'noprelude:alloc=/Users/Luppy/riscv/nuttx-rust-app/app/target/riscv32gc-unknown-none-elf/debug/deps/liballoc-5d7bc2e4f3c29e08.rlib' \
  --extern 'noprelude:compiler_builtins=/Users/Luppy/riscv/nuttx-rust-app/app/target/riscv32gc-unknown-none-elf/debug/deps/libcompiler_builtins-cd0d33c2bd30ca51.rlib' \
  --extern 'noprelude:core=/Users/Luppy/riscv/nuttx-rust-app/app/target/riscv32gc-unknown-none-elf/debug/deps/libcore-d271c6ebb87f9b41.rlib' \
  -Z unstable-options

## Should show:
## Flags: 0x4, double-float ABI
riscv64-unknown-elf-readelf \
  -h -A \
  ../apps/examples/hello_rust/*hello_rust_1.o

cp \
  ../apps/examples/hello_rust/*hello_rust.o \
  ../apps/examples/hello_rust/*hello_rust_1.o

make
```

TODO

```bash
→ cargo build -v \
  -Zbuild-std=core,alloc \
  --target ../riscv32gc-unknown-none-elf.json

   Compiling compiler_builtins v0.1.101
   Compiling core v0.0.0 (/Users/Luppy/.rustup/toolchains/nightly-x86_64-apple-darwin/lib/rustlib/src/rust/library/core)
     Running `/Users/Luppy/.rustup/toolchains/nightly-x86_64-apple-darwin/bin/rustc --crate-name build_script_build --edition=2018 /Users/Luppy/.cargo/registry/src/index.crates.io-6f17d22bba15001f/compiler_builtins-0.1.101/build.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --diagnostic-width=94 --crate-type bin --emit=dep-info,link -C embed-bitcode=no -C debuginfo=2 -C split-debuginfo=unpacked --cfg 'feature="compiler-builtins"' --cfg 'feature="core"' --cfg 'feature="default"' --cfg 'feature="rustc-dep-of-std"' -C metadata=9bd0bac7535b33a8 -C extra-filename=-9bd0bac7535b33a8 --out-dir /Users/Luppy/riscv/nuttx-rust-app/app/target/debug/build/compiler_builtins-9bd0bac7535b33a8 -Z force-unstable-if-unmarked -L dependency=/Users/Luppy/riscv/nuttx-rust-app/app/target/debug/deps --cap-lints allow`
     Running `/Users/Luppy/.rustup/toolchains/nightly-x86_64-apple-darwin/bin/rustc --crate-name core --edition=2021 /Users/Luppy/.rustup/toolchains/nightly-x86_64-apple-darwin/lib/rustlib/src/rust/library/core/src/lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --diagnostic-width=94 --crate-type lib --emit=dep-info,metadata,link -C embed-bitcode=no -C debuginfo=2 -C metadata=d271c6ebb87f9b41 -C extra-filename=-d271c6ebb87f9b41 --out-dir /Users/Luppy/riscv/nuttx-rust-app/app/target/riscv32gc-unknown-none-elf/debug/deps --target /Users/Luppy/riscv/nuttx-rust-app/riscv32gc-unknown-none-elf.json -Z force-unstable-if-unmarked -L dependency=/Users/Luppy/riscv/nuttx-rust-app/app/target/riscv32gc-unknown-none-elf/debug/deps -L dependency=/Users/Luppy/riscv/nuttx-rust-app/app/target/debug/deps --cap-lints allow`
     Running `/Users/Luppy/riscv/nuttx-rust-app/app/target/debug/build/compiler_builtins-9bd0bac7535b33a8/build-script-build`
   Compiling rustc-std-workspace-core v1.99.0 (/Users/Luppy/.rustup/toolchains/nightly-x86_64-apple-darwin/lib/rustlib/src/rust/library/rustc-std-workspace-core)
     Running `/Users/Luppy/.rustup/toolchains/nightly-x86_64-apple-darwin/bin/rustc --crate-name rustc_std_workspace_core --edition=2021 /Users/Luppy/.rustup/toolchains/nightly-x86_64-apple-darwin/lib/rustlib/src/rust/library/rustc-std-workspace-core/lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --diagnostic-width=94 --crate-type lib --emit=dep-info,metadata,link -C embed-bitcode=no -C debuginfo=2 -C metadata=52e0df2b2cc19b6e -C extra-filename=-52e0df2b2cc19b6e --out-dir /Users/Luppy/riscv/nuttx-rust-app/app/target/riscv32gc-unknown-none-elf/debug/deps --target /Users/Luppy/riscv/nuttx-rust-app/riscv32gc-unknown-none-elf.json -Z force-unstable-if-unmarked -L dependency=/Users/Luppy/riscv/nuttx-rust-app/app/target/riscv32gc-unknown-none-elf/debug/deps -L dependency=/Users/Luppy/riscv/nuttx-rust-app/app/target/debug/deps --extern core=/Users/Luppy/riscv/nuttx-rust-app/app/target/riscv32gc-unknown-none-elf/debug/deps/libcore-d271c6ebb87f9b41.rmeta --cap-lints allow`
     Running `/Users/Luppy/.rustup/toolchains/nightly-x86_64-apple-darwin/bin/rustc --crate-name compiler_builtins --edition=2018 /Users/Luppy/.cargo/registry/src/index.crates.io-6f17d22bba15001f/compiler_builtins-0.1.101/src/lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --diagnostic-width=94 --crate-type lib --emit=dep-info,metadata,link -C embed-bitcode=no -C debuginfo=2 --cfg 'feature="compiler-builtins"' --cfg 'feature="core"' --cfg 'feature="default"' --cfg 'feature="rustc-dep-of-std"' -C metadata=cd0d33c2bd30ca51 -C extra-filename=-cd0d33c2bd30ca51 --out-dir /Users/Luppy/riscv/nuttx-rust-app/app/target/riscv32gc-unknown-none-elf/debug/deps --target /Users/Luppy/riscv/nuttx-rust-app/riscv32gc-unknown-none-elf.json -Z force-unstable-if-unmarked -L dependency=/Users/Luppy/riscv/nuttx-rust-app/app/target/riscv32gc-unknown-none-elf/debug/deps -L dependency=/Users/Luppy/riscv/nuttx-rust-app/app/target/debug/deps --extern core=/Users/Luppy/riscv/nuttx-rust-app/app/target/riscv32gc-unknown-none-elf/debug/deps/librustc_std_workspace_core-52e0df2b2cc19b6e.rmeta --cap-lints allow --cfg 'feature="unstable"' --cfg 'feature="mem"'`
   Compiling alloc v0.0.0 (/Users/Luppy/.rustup/toolchains/nightly-x86_64-apple-darwin/lib/rustlib/src/rust/library/alloc)
     Running `/Users/Luppy/.rustup/toolchains/nightly-x86_64-apple-darwin/bin/rustc --crate-name alloc --edition=2021 /Users/Luppy/.rustup/toolchains/nightly-x86_64-apple-darwin/lib/rustlib/src/rust/library/alloc/src/lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --diagnostic-width=94 --crate-type lib --emit=dep-info,metadata,link -C embed-bitcode=no -C debuginfo=2 -C metadata=5d7bc2e4f3c29e08 -C extra-filename=-5d7bc2e4f3c29e08 --out-dir /Users/Luppy/riscv/nuttx-rust-app/app/target/riscv32gc-unknown-none-elf/debug/deps --target /Users/Luppy/riscv/nuttx-rust-app/riscv32gc-unknown-none-elf.json -Z force-unstable-if-unmarked -L dependency=/Users/Luppy/riscv/nuttx-rust-app/app/target/riscv32gc-unknown-none-elf/debug/deps -L dependency=/Users/Luppy/riscv/nuttx-rust-app/app/target/debug/deps --extern compiler_builtins=/Users/Luppy/riscv/nuttx-rust-app/app/target/riscv32gc-unknown-none-elf/debug/deps/libcompiler_builtins-cd0d33c2bd30ca51.rmeta --extern core=/Users/Luppy/riscv/nuttx-rust-app/app/target/riscv32gc-unknown-none-elf/debug/deps/libcore-d271c6ebb87f9b41.rmeta --cap-lints allow`
   Compiling app v0.1.0 (/Users/Luppy/riscv/nuttx-rust-app/app)
     Running `/Users/Luppy/.rustup/toolchains/nightly-x86_64-apple-darwin/bin/rustc --crate-name app --edition=2021 src/main.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --diagnostic-width=94 --crate-type bin --emit=dep-info,link -C embed-bitcode=no -C debuginfo=2 -C metadata=1ff442e6481e1397 -C extra-filename=-1ff442e6481e1397 --out-dir /Users/Luppy/riscv/nuttx-rust-app/app/target/riscv32gc-unknown-none-elf/debug/deps --target /Users/Luppy/riscv/nuttx-rust-app/riscv32gc-unknown-none-elf.json -C incremental=/Users/Luppy/riscv/nuttx-rust-app/app/target/riscv32gc-unknown-none-elf/debug/incremental -L dependency=/Users/Luppy/riscv/nuttx-rust-app/app/target/riscv32gc-unknown-none-elf/debug/deps -L dependency=/Users/Luppy/riscv/nuttx-rust-app/app/target/debug/deps --extern 'noprelude:alloc=/Users/Luppy/riscv/nuttx-rust-app/app/target/riscv32gc-unknown-none-elf/debug/deps/liballoc-5d7bc2e4f3c29e08.rlib' --extern 'noprelude:compiler_builtins=/Users/Luppy/riscv/nuttx-rust-app/app/target/riscv32gc-unknown-none-elf/debug/deps/libcompiler_builtins-cd0d33c2bd30ca51.rlib' --extern 'noprelude:core=/Users/Luppy/riscv/nuttx-rust-app/app/target/riscv32gc-unknown-none-elf/debug/deps/libcore-d271c6ebb87f9b41.rlib' -Z unstable-options`
error[E0463]: can't find crate for `std`
  |
  = note: the `riscv32gc-unknown-none-elf` target may not support the standard library
  = note: `std` is required by `app` because it does not declare `#![no_std]`
  = help: consider building the standard library from source with `cargo build -Zbuild-std`

error: cannot find macro `println` in this scope
 --> src/main.rs:2:5
  |
2 |     println!("Hello, world!");
  |     ^^^^^^^

error: `#[panic_handler]` function required, but not found

For more information about this error, try `rustc --explain E0463`.
error: could not compile `app` (bin "app") due to 3 previous errors

Caused by:
  process didn't exit successfully: `/Users/Luppy/.rustup/toolchains/nightly-x86_64-apple-darwin/bin/rustc --crate-name app --edition=2021 src/main.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --diagnostic-width=94 --crate-type bin --emit=dep-info,link -C embed-bitcode=no -C debuginfo=2 -C metadata=1ff442e6481e1397 -C extra-filename=-1ff442e6481e1397 --out-dir /Users/Luppy/riscv/nuttx-rust-app/app/target/riscv32gc-unknown-none-elf/debug/deps --target /Users/Luppy/riscv/nuttx-rust-app/riscv32gc-unknown-none-elf.json -C incremental=/Users/Luppy/riscv/nuttx-rust-app/app/target/riscv32gc-unknown-none-elf/debug/incremental -L dependency=/Users/Luppy/riscv/nuttx-rust-app/app/target/riscv32gc-unknown-none-elf/debug/deps -L dependency=/Users/Luppy/riscv/nuttx-rust-app/app/target/debug/deps --extern 'noprelude:alloc=/Users/Luppy/riscv/nuttx-rust-app/app/target/riscv32gc-unknown-none-elf/debug/deps/liballoc-5d7bc2e4f3c29e08.rlib' --extern 'noprelude:compiler_builtins=/Users/Luppy/riscv/nuttx-rust-app/app/target/riscv32gc-unknown-none-elf/debug/deps/libcompiler_builtins-cd0d33c2bd30ca51.rlib' --extern 'noprelude:core=/Users/Luppy/riscv/nuttx-rust-app/app/target/riscv32gc-unknown-none-elf/debug/deps/libcore-d271c6ebb87f9b41.rlib' -Z unstable-options` (exit status: 1)
```
