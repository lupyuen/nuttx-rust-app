![Rust Apps on Apache NuttX RTOS and QEMU RISC-V](https://lupyuen.github.io/images/rust4-title.png)

[_Thanks to cool-retro-term!_](https://github.com/Swordfish90/cool-retro-term)

# Rust Apps for Apache NuttX RTOS and QEMU RISC-V

Read the articles...

- ["Rust Standard Library on Apache NuttX RTOS"](https://lupyuen.org/articles/rust7.html)

- ["Early Days of Rust Apps on Apache NuttX RTOS"](https://lupyuen.github.io/articles/rust6)

- ["Rust Apps on Apache NuttX RTOS and QEMU RISC-V"](https://lupyuen.github.io/articles/rust3)

- ["Rust Custom Target for QEMU RISC-V on Apache NuttX RTOS"](https://lupyuen.github.io/articles/rust4)

- ["Rust Apps on Ox64 BL808 RISC-V SBC and Apache NuttX RTOS"](https://lupyuen.github.io/articles/rust5)

Read on to find out how everything all started...

# Rust Custom Target for QEMU RISC-V on Apache NuttX RTOS

Read the article...

- ["Rust Custom Target for QEMU RISC-V on Apache NuttX RTOS"](https://lupyuen.github.io/articles/rust4)

We have a problem compiling [Rust Apps for QEMU RISC-V 32-bit](https://lupyuen.github.io/articles/rust3#software-vs-hardware-floating-point)...

```bash
$ make
LD: nuttx
riscv64-unknown-elf-ld: nuttx/nuttx/staging/libapps.a
  (hello_rust_main.rs...nuttx.apps.examples.hello_rust_1.o):
  can't link soft-float modules with double-float modules

riscv64-unknown-elf-ld: failed to merge target specific data of file
  nuttx/staging/libapps.a
  (hello_rust_main.rs...nuttx.apps.examples.hello_rust_1.o)
```

That's because [NuttX builds Rust Apps](https://lupyuen.github.io/articles/rust3#how-nuttx-compiles-rust-apps) for `riscv32i-unknown-none-elf` (Software Floating-Point)...

```bash
## Compile `hello_rust_main.rs` to `hello_rust.o`
## for Rust Target: RISC-V 32-bit (Soft-Float)
rustc \
  --edition 2021 \
  --emit obj \
  -g \
  --target riscv32i-unknown-none-elf \
  -C panic=abort \
  -O \
  hello_rust_main.rs \
  -o hello_rust_main.rs...apps.examples.hello_rust.o
```

But the rest of NuttX is Double-Precision Hardware Floating-Point!

(`-march=rv32imafdc -mabi=ilp32d`)

```bash
$ make --trace
...
riscv64-unknown-elf-gcc \
  -c \
  -fno-common \
  -Wall \
  -Wstrict-prototypes \
  -Wshadow \
  -Wundef \
  -Wno-attributes \
  -Wno-unknown-pragmas \
  -Wno-psabi \
  -Os \
  -fno-strict-aliasing \
  -fomit-frame-pointer \
  -ffunction-sections \
  -fdata-sections \
  -g \
  -march=rv32imafdc \
  -mabi=ilp32d \
  -isystem nuttx/include \
  -D__NuttX__ \
  -DNDEBUG  \
  -pipe \
  -I "apps/include" \
  -Dmain=hello_main \
  hello_main.c \
  -o  hello_main.c...apps.examples.hello.o \
```

![Double-Float vs Soft-Float: GCC Linker won't link the binaries](https://lupyuen.github.io/images/rust4-flow2.jpg)

_Does Rust support Double-Precision Hardware Floating-Point?_

We're looking for a Rust Target like `riscv32gc-unknown-none-elf`...

```bash
$ rustup target list | grep riscv
riscv32i-unknown-none-elf
riscv32imac-unknown-none-elf
riscv32imc-unknown-none-elf
riscv64gc-unknown-linux-gnu
riscv64gc-unknown-none-elf
riscv64imac-unknown-none-elf
```

But nope it's not supported! So we create a Rust Custom Target for `riscv32gc-unknown-none-elf`...

- [Custom Target for Rust](https://docs.rust-embedded.org/embedonomicon/custom-target.html)

![Creating a Rust Custom Target for riscv32gc-unknown-none-elf](https://lupyuen.github.io/images/rust4-flow.jpg)

Let's dump the Rust Targets `riscv32i` and `riscv64gc` to compare...

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

Based on the above, we create our Rust Custom Target: [riscv32gc-unknown-none-elf.json](riscv32gc-unknown-none-elf.json)

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
  "llvm-abiname": "ilp32d",
  "llvm-target": "riscv32",
  "max-atomic-width": 0,
  "panic-strategy": "abort",
  "relocation-model": "static",
  "target-pointer-width": "32"
}
```

Which is based on `riscv32i` with these changes...

- Removed `"atomic-cas": false`

- Added `"features": "+m,+a,+f,+d,+c"`

- Removed `"is-builtin": true`

- Added `"llvm-abiname": "ilp32d"`

  (`ilp32d` comes from `make --trace` above)

  [(More about `llvm-abiname`)](https://lupyuen.github.io/articles/rust#custom-rust-target-for-bl602)

Now we build the Rust Core Library for `riscv32gc`...

```bash
## Verify our Custom Target
rustc \
  --print cfg \
  --target riscv32gc-unknown-none-elf.json

## Build the Rust Core Library for `riscv32gc`
## Ignore the error: `app already exists`
cargo new app
pushd app
cargo clean
## Ignore the error: `can't find crate for std`
cargo build \
  -Zbuild-std=core,alloc \
  --target ../riscv32gc-unknown-none-elf.json
popd
```

We rebuild our Rust App with the new Rust Custom Target (linked to our Rust Core Library)...

```bash
## Compile our Rust App.
## Changed the target to riscv32gc-unknown-none-elf.json
rustc \
  --edition 2021 \
  --emit obj \
  -g \
  --target riscv32gc-unknown-none-elf.json \
  -C panic=abort \
  -O \
  ../apps/examples/hello_rust/hello_rust_main.rs \
  -o ../apps/examples/hello_rust/*hello_rust.o \
  \
  -C incremental=app/target/riscv32gc-unknown-none-elf/debug/incremental \
  -L dependency=app/target/riscv32gc-unknown-none-elf/debug/deps \
  -L dependency=app/target/debug/deps \
  --extern noprelude:alloc=`ls app/target/riscv32gc-unknown-none-elf/debug/deps/liballoc-*.rlib` \
  --extern noprelude:compiler_builtins=`ls app/target/riscv32gc-unknown-none-elf/debug/deps/libcompiler_builtins-*.rlib` \
  --extern noprelude:core=`ls app/target/riscv32gc-unknown-none-elf/debug/deps/libcore-*.rlib` \
  -Z unstable-options

## Dump the ELF Header. Should show:
## Flags: 0x5, RVC, double-float ABI
riscv64-unknown-elf-readelf \
  -h -A \
  ../apps/examples/hello_rust/*hello_rust.o

## NuttX should link and execute correctly now
cp \
  ../apps/examples/hello_rust/*hello_rust.o \
  ../apps/examples/hello_rust/*hello_rust_1.o

pushd ../nuttx
make
qemu-system-riscv32 \
  -semihosting \
  -M virt,aclint=on \
  -cpu rv32 \
  -smp 8 \
  -bios none \
  -kernel nuttx \
  -nographic
popd
```

And it works!

_Our Rust App links OK! Has the ELF Header changed?_

Yep the ELF Header has changed from Soft-Float to Double-Float...

```bash
## Before Custom Target
$ riscv64-unknown-elf-readelf \
  -h -A \
  ../apps/examples/hello_rust/*hello_rust_1.o

ELF Header:
  Magic:   7f 45 4c 46 01 01 01 00 00 00 00 00 00 00 00 00 
  Class:                             ELF32
  Data:                              2's complement, little endian
  Version:                           1 (current)
  OS/ABI:                            UNIX - System V
  ABI Version:                       0
  Type:                              REL (Relocatable file)
  Machine:                           RISC-V
  Version:                           0x1
  Entry point address:               0x0
  Start of program headers:          0 (bytes into file)
  Start of section headers:          10240 (bytes into file)
  Flags:                             0x0
  Size of this header:               52 (bytes)
  Size of program headers:           0 (bytes)
  Number of program headers:         0
  Size of section headers:           40 (bytes)
  Number of section headers:         29
  Section header string table index: 1
Attribute Section: riscv
File Attributes
  Tag_RISCV_stack_align: 16-bytes
  Tag_RISCV_arch: "rv32i2p1"

## After Custom Target
$ riscv64-unknown-elf-readelf \
  -h -A \
  ../apps/examples/hello_rust/*hello_rust.o

ELF Header:
  Magic:   7f 45 4c 46 01 01 01 00 00 00 00 00 00 00 00 00 
  Class:                             ELF32
  Data:                              2's complement, little endian
  Version:                           1 (current)
  OS/ABI:                            UNIX - System V
  ABI Version:                       0
  Type:                              REL (Relocatable file)
  Machine:                           RISC-V
  Version:                           0x1
  Entry point address:               0x0
  Start of program headers:          0 (bytes into file)
  Start of section headers:          10352 (bytes into file)
  Flags:                             0x5, RVC, double-float ABI
  Size of this header:               52 (bytes)
  Size of program headers:           0 (bytes)
  Number of program headers:         0
  Size of section headers:           40 (bytes)
  Number of section headers:         29
  Section header string table index: 1
Attribute Section: riscv
File Attributes
  Tag_RISCV_stack_align: 16-bytes
  Tag_RISCV_arch: "rv32i2p1_m2p0_a2p1_f2p2_d2p2_c2p0_zicsr2p0"

## Which looks similar to other C Binaries
$ riscv64-unknown-elf-readelf \
  -h -A \
  ../apps/examples/hello/*hello.o                 

ELF Header:
  Magic:   7f 45 4c 46 01 01 01 00 00 00 00 00 00 00 00 00 
  Class:                             ELF32
  Data:                              2's complement, little endian
  Version:                           1 (current)
  OS/ABI:                            UNIX - System V
  ABI Version:                       0
  Type:                              REL (Relocatable file)
  Machine:                           RISC-V
  Version:                           0x1
  Entry point address:               0x0
  Start of program headers:          0 (bytes into file)
  Start of section headers:          3776 (bytes into file)
  Flags:                             0x5, RVC, double-float ABI
  Size of this header:               52 (bytes)
  Size of program headers:           0 (bytes)
  Number of program headers:         0
  Size of section headers:           40 (bytes)
  Number of section headers:         26
  Section header string table index: 25
Attribute Section: riscv
File Attributes
  Tag_RISCV_stack_align: 16-bytes
  Tag_RISCV_arch: "rv32i2p0_m2p0_a2p0_f2p0_d2p0_c2p0"
```

_How did we figure out the rustc options?_

`cargo build` will call `rustc` with a whole bunch of options.

We ran `cargo build -v` to dump the `rustc` options that were used to compile a Rust App with our Custom Rust Core Library for `riscv32gc`...

```bash
$ cargo build -v \
  -Zbuild-std=core,alloc \
  --target ../riscv32gc-unknown-none-elf.json

   Compiling compiler_builtins v0.1.101
   Compiling core v0.0.0 ($HOME/.rustup/toolchains/nightly-x86_64-apple-darwin/lib/rustlib/src/rust/library/core)
     Running `$HOME/.rustup/toolchains/nightly-x86_64-apple-darwin/bin/rustc --crate-name build_script_build --edition=2018 $HOME/.cargo/registry/src/index.crates.io-6f17d22bba15001f/compiler_builtins-0.1.101/build.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --diagnostic-width=94 --crate-type bin --emit=dep-info,link -C embed-bitcode=no -C debuginfo=2 -C split-debuginfo=unpacked --cfg 'feature="compiler-builtins"' --cfg 'feature="core"' --cfg 'feature="default"' --cfg 'feature="rustc-dep-of-std"' -C metadata=9bd0bac7535b33a8 -C extra-filename=-9bd0bac7535b33a8 --out-dir $HOME/riscv/nuttx-rust-app/app/target/debug/build/compiler_builtins-9bd0bac7535b33a8 -Z force-unstable-if-unmarked -L dependency=$HOME/riscv/nuttx-rust-app/app/target/debug/deps --cap-lints allow`
     Running `$HOME/.rustup/toolchains/nightly-x86_64-apple-darwin/bin/rustc --crate-name core --edition=2021 $HOME/.rustup/toolchains/nightly-x86_64-apple-darwin/lib/rustlib/src/rust/library/core/src/lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --diagnostic-width=94 --crate-type lib --emit=dep-info,metadata,link -C embed-bitcode=no -C debuginfo=2 -C metadata=d271c6ebb87f9b41 -C extra-filename=-d271c6ebb87f9b41 --out-dir $HOME/riscv/nuttx-rust-app/app/target/riscv32gc-unknown-none-elf/debug/deps --target $HOME/riscv/nuttx-rust-app/riscv32gc-unknown-none-elf.json -Z force-unstable-if-unmarked -L dependency=$HOME/riscv/nuttx-rust-app/app/target/riscv32gc-unknown-none-elf/debug/deps -L dependency=$HOME/riscv/nuttx-rust-app/app/target/debug/deps --cap-lints allow`
     Running `$HOME/riscv/nuttx-rust-app/app/target/debug/build/compiler_builtins-9bd0bac7535b33a8/build-script-build`
   Compiling rustc-std-workspace-core v1.99.0 ($HOME/.rustup/toolchains/nightly-x86_64-apple-darwin/lib/rustlib/src/rust/library/rustc-std-workspace-core)
     Running `$HOME/.rustup/toolchains/nightly-x86_64-apple-darwin/bin/rustc --crate-name rustc_std_workspace_core --edition=2021 $HOME/.rustup/toolchains/nightly-x86_64-apple-darwin/lib/rustlib/src/rust/library/rustc-std-workspace-core/lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --diagnostic-width=94 --crate-type lib --emit=dep-info,metadata,link -C embed-bitcode=no -C debuginfo=2 -C metadata=52e0df2b2cc19b6e -C extra-filename=-52e0df2b2cc19b6e --out-dir $HOME/riscv/nuttx-rust-app/app/target/riscv32gc-unknown-none-elf/debug/deps --target $HOME/riscv/nuttx-rust-app/riscv32gc-unknown-none-elf.json -Z force-unstable-if-unmarked -L dependency=$HOME/riscv/nuttx-rust-app/app/target/riscv32gc-unknown-none-elf/debug/deps -L dependency=$HOME/riscv/nuttx-rust-app/app/target/debug/deps --extern core=$HOME/riscv/nuttx-rust-app/app/target/riscv32gc-unknown-none-elf/debug/deps/libcore-d271c6ebb87f9b41.rmeta --cap-lints allow`
     Running `$HOME/.rustup/toolchains/nightly-x86_64-apple-darwin/bin/rustc --crate-name compiler_builtins --edition=2018 $HOME/.cargo/registry/src/index.crates.io-6f17d22bba15001f/compiler_builtins-0.1.101/src/lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --diagnostic-width=94 --crate-type lib --emit=dep-info,metadata,link -C embed-bitcode=no -C debuginfo=2 --cfg 'feature="compiler-builtins"' --cfg 'feature="core"' --cfg 'feature="default"' --cfg 'feature="rustc-dep-of-std"' -C metadata=cd0d33c2bd30ca51 -C extra-filename=-cd0d33c2bd30ca51 --out-dir $HOME/riscv/nuttx-rust-app/app/target/riscv32gc-unknown-none-elf/debug/deps --target $HOME/riscv/nuttx-rust-app/riscv32gc-unknown-none-elf.json -Z force-unstable-if-unmarked -L dependency=$HOME/riscv/nuttx-rust-app/app/target/riscv32gc-unknown-none-elf/debug/deps -L dependency=$HOME/riscv/nuttx-rust-app/app/target/debug/deps --extern core=$HOME/riscv/nuttx-rust-app/app/target/riscv32gc-unknown-none-elf/debug/deps/librustc_std_workspace_core-52e0df2b2cc19b6e.rmeta --cap-lints allow --cfg 'feature="unstable"' --cfg 'feature="mem"'`
   Compiling alloc v0.0.0 ($HOME/.rustup/toolchains/nightly-x86_64-apple-darwin/lib/rustlib/src/rust/library/alloc)
     Running `$HOME/.rustup/toolchains/nightly-x86_64-apple-darwin/bin/rustc --crate-name alloc --edition=2021 $HOME/.rustup/toolchains/nightly-x86_64-apple-darwin/lib/rustlib/src/rust/library/alloc/src/lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --diagnostic-width=94 --crate-type lib --emit=dep-info,metadata,link -C embed-bitcode=no -C debuginfo=2 -C metadata=5d7bc2e4f3c29e08 -C extra-filename=-5d7bc2e4f3c29e08 --out-dir $HOME/riscv/nuttx-rust-app/app/target/riscv32gc-unknown-none-elf/debug/deps --target $HOME/riscv/nuttx-rust-app/riscv32gc-unknown-none-elf.json -Z force-unstable-if-unmarked -L dependency=$HOME/riscv/nuttx-rust-app/app/target/riscv32gc-unknown-none-elf/debug/deps -L dependency=$HOME/riscv/nuttx-rust-app/app/target/debug/deps --extern compiler_builtins=$HOME/riscv/nuttx-rust-app/app/target/riscv32gc-unknown-none-elf/debug/deps/libcompiler_builtins-cd0d33c2bd30ca51.rmeta --extern core=$HOME/riscv/nuttx-rust-app/app/target/riscv32gc-unknown-none-elf/debug/deps/libcore-d271c6ebb87f9b41.rmeta --cap-lints allow`
   Compiling app v0.1.0 ($HOME/riscv/nuttx-rust-app/app)
     Running `$HOME/.rustup/toolchains/nightly-x86_64-apple-darwin/bin/rustc --crate-name app --edition=2021 src/main.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --diagnostic-width=94 --crate-type bin --emit=dep-info,link -C embed-bitcode=no -C debuginfo=2 -C metadata=1ff442e6481e1397 -C extra-filename=-1ff442e6481e1397 --out-dir $HOME/riscv/nuttx-rust-app/app/target/riscv32gc-unknown-none-elf/debug/deps --target $HOME/riscv/nuttx-rust-app/riscv32gc-unknown-none-elf.json -C incremental=$HOME/riscv/nuttx-rust-app/app/target/riscv32gc-unknown-none-elf/debug/incremental -L dependency=$HOME/riscv/nuttx-rust-app/app/target/riscv32gc-unknown-none-elf/debug/deps -L dependency=$HOME/riscv/nuttx-rust-app/app/target/debug/deps --extern 'noprelude:alloc=$HOME/riscv/nuttx-rust-app/app/target/riscv32gc-unknown-none-elf/debug/deps/liballoc-5d7bc2e4f3c29e08.rlib' --extern 'noprelude:compiler_builtins=$HOME/riscv/nuttx-rust-app/app/target/riscv32gc-unknown-none-elf/debug/deps/libcompiler_builtins-cd0d33c2bd30ca51.rlib' --extern 'noprelude:core=$HOME/riscv/nuttx-rust-app/app/target/riscv32gc-unknown-none-elf/debug/deps/libcore-d271c6ebb87f9b41.rlib' -Z unstable-options`
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
  process didn't exit successfully: `$HOME/.rustup/toolchains/nightly-x86_64-apple-darwin/bin/rustc --crate-name app --edition=2021 src/main.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --diagnostic-width=94 --crate-type bin --emit=dep-info,link -C embed-bitcode=no -C debuginfo=2 -C metadata=1ff442e6481e1397 -C extra-filename=-1ff442e6481e1397 --out-dir $HOME/riscv/nuttx-rust-app/app/target/riscv32gc-unknown-none-elf/debug/deps --target $HOME/riscv/nuttx-rust-app/riscv32gc-unknown-none-elf.json -C incremental=$HOME/riscv/nuttx-rust-app/app/target/riscv32gc-unknown-none-elf/debug/incremental -L dependency=$HOME/riscv/nuttx-rust-app/app/target/riscv32gc-unknown-none-elf/debug/deps -L dependency=$HOME/riscv/nuttx-rust-app/app/target/debug/deps --extern 'noprelude:alloc=$HOME/riscv/nuttx-rust-app/app/target/riscv32gc-unknown-none-elf/debug/deps/liballoc-5d7bc2e4f3c29e08.rlib' --extern 'noprelude:compiler_builtins=$HOME/riscv/nuttx-rust-app/app/target/riscv32gc-unknown-none-elf/debug/deps/libcompiler_builtins-cd0d33c2bd30ca51.rlib' --extern 'noprelude:core=$HOME/riscv/nuttx-rust-app/app/target/riscv32gc-unknown-none-elf/debug/deps/libcore-d271c6ebb87f9b41.rlib' -Z unstable-options` (exit status: 1)
```

This Rust Compiler Issue might be relevant...

- [Allow building for hard-float targets in RISC-V](https://github.com/rust-lang/rust/issues/65024)

_How to see the Targets supported by GCC?_

Like this...

```bash
$ riscv64-unknown-elf-gcc --target-help

  Supported ABIs (for use with the -mabi= option):
    ilp32 ilp32d ilp32e ilp32f lp64 lp64d lp64f
```

[(As explained here)](https://gcc.gnu.org/onlinedocs/gcc/RISC-V-Options.html#index-mabi-5)

# Rust Apps won't compile for QEMU RISC-V 64-bit

Will Rust Apps run on a 64-bit RISC-V SBC, like Ox64 BL808? Let's find out!

First we test on QEMU RISC-V 64-bit...

```bash
$ tools/configure.sh rv-virt:nsh64
$ make menuconfig
## TODO: Enable "Hello Rust" Example App
## https://github.com/lupyuen2/wip-nuttx/blob/rust/boards/risc-v/qemu-rv/rv-virt/configs/nsh64/defconfig
$ make --trace

## Compile "hello_main.c" with GCC Compiler
riscv64-unknown-elf-gcc \
  -c \
  -fno-common \
  -Wall \
  -Wstrict-prototypes \
  -Wshadow \
  -Wundef \
  -Wno-attributes \
  -Wno-unknown-pragmas \
  -Wno-psabi \
  -Os \
  -fno-strict-aliasing \
  -fomit-frame-pointer \
  -ffunction-sections \
  -fdata-sections \
  -g \
  -mcmodel=medany \
  -march=rv64imafdc \
  -mabi=lp64d \
  -isystem /Users/Luppy/riscv/nuttx/include \
  -D__NuttX__ \
  -DNDEBUG  \
  -pipe \
  -I "/Users/Luppy/riscv/apps/include" \
  -Dmain=hello_main  hello_main.c \
  -o  hello_main.c.Users.Luppy.riscv.apps.examples.hello.o

## Compile "hello_rust_main.rs" with Rust Compiler
rustc \
  --edition 2021 \
  --emit obj \
  -g \
  --target riscv64i-unknown-none-elf \
  -C panic=abort \
  -O   hello_rust_main.rs \
  -o  hello_rust_main.rs.Users.Luppy.riscv.apps.examples.hello_rust.o

error: Error loading target specification: Could not find specification for target "riscv64i-unknown-none-elf". Run `rustc --print target-list` for a list of built-in targets

make[2]: *** [/Users/Luppy/riscv/apps/Application.mk:275: hello_rust_main.rs.Users.Luppy.riscv.apps.examples.hello_rust.o] Error 1
make[1]: *** [Makefile:51: /Users/Luppy/riscv/apps/examples/hello_rust_all] Error 2
make: *** [tools/LibTargets.mk:232: /Users/Luppy/riscv/apps/libapps.a] Error 2
```

But it fails! Rust Compiler says that __`riscv64i`__ isn't a valid Rust Target for 64-bit RISC-V.

So many questions...

1.  Is __`riscv64i`__ the correct target for QEMU?

    [(__Hint:__ See this)](https://www.qemu.org/docs/master/system/riscv/virt.html#supported-devices)

1.  How should we __Fix the Build__?
    
1.  Do we need a __Custom Target__?

    (__Hint:__ Answer is printed above somewhere)

1.  Will it run on [__Ox64 BL808 SBC__](https://www.hackster.io/lupyuen/8-risc-v-sbc-on-a-real-time-operating-system-ox64-nuttx-474358)?

Let's fix this!

# Change riscv64i to riscv64gc

_Is __`riscv64i`__ the correct target for QEMU?_

Nope [QEMU supports riscv64gc](https://www.qemu.org/docs/master/system/riscv/virt.html#supported-devices)!

For building our Rust App: Let's change riscv64i to riscv64gc...

```bash
$ rustup target add riscv64gc-unknown-none-elf
$ pushd ../apps/examples/hello_rust 
$ rustc \
  --edition 2021 \
  --emit obj \
  -g \
  --target riscv64gc-unknown-none-elf \
  -C panic=abort \
  -O   hello_rust_main.rs \
  -o  hello_rust_main.rs.Users.Luppy.riscv.apps.examples.hello_rust.o
$ popd
$ make
```

TODO: Fix the path of hello_rust.o

And our Rust App runs OK on QEMU RISC-V 64-bit yay!

```bash
$ qemu-system-riscv64 -semihosting -M virt,aclint=on -cpu rv64 -smp 8 -bios none -kernel nuttx -nographic
ABCnx_start: Entry
uart_register: Registering /dev/console
uart_register: Registering /dev/ttyS0
nx_start_application: Starting init thread
task_spawn: name=nsh_main entry=0x8000745c file_actions=0 attr=0x8003d798 argv=0x8003d790
nxtask_activate: nsh_main pid=1,TCB=0x8003e820

NuttShell (NSH) NuttX-12.4.0-RC0
nsh> nx_start: CPU0: Beginning Idle Loop

nsh> hello_rust
posix_spawn: pid=0x8003f734 path=hello_rust file_actions=0x8003f738 attr=0x8003f740 argv=0x8003f838
nxposix_spawn_exec: ERROR: exec failed: 2
task_spawn: name=hello_rust entry=0x80018622 file_actions=0x8003f738 attr=0x8003f740 argv=0x8003f840
spawn_execattrs: Setting policy=2 priority=100 for pid=2
nxtask_activate: hello_rust pid=2,TCB=0x8003fda0
Hello, Rust!!
abcd
You entered...
abcd

nxtask_exit: hello_rust pid=2,TCB=0x8003fda0
nsh> 
```

# Rust Apps on Ox64 BL808 SBC

![Rust Apps on Apache NuttX RTOS and Ox64 BL808 SBC](https://lupyuen.github.io/images/rust5-title.jpg)

[_Thanks to cool-retro-term!_](https://github.com/Swordfish90/cool-retro-term)

Let's do the same for Ox64 BL808 SBC...

```bash
$ tools/configure.sh ox64:nsh
$ make menuconfig
## TODO: Enable "Hello Rust" Example App
## https://github.com/lupyuen2/wip-nuttx/blob/rust/boards/risc-v/bl808/ox64/configs/nsh/defconfig
$ make
$ make --trace export
$ pushd ../apps
$ make --trace import

riscv64-unknown-elf-gcc \
  -c \
  -fno-common \
  -Wall \
  -Wstrict-prototypes \
  -Wshadow \
  -Wundef \
  -Wno-attributes \
  -Wno-unknown-pragmas \
  -Wno-psabi \
  -fno-common \
  -pipe  \
  -Os \
  -fno-strict-aliasing \
  -fomit-frame-pointer \
  -ffunction-sections \
  -fdata-sections \
  -g \
  -mcmodel=medany \
  -march=rv64imafdc \
  -mabi=lp64d \
  -isystem /Users/Luppy/ox64/apps/import/include \
  -isystem /Users/Luppy/ox64/apps/import/include \
  -D__NuttX__  \
  -I "/Users/Luppy/ox64/apps/include"   hello_main.c \
  -o  hello_main.c.Users.Luppy.ox64.apps.examples.hello.o

Makefile:52: target '/Users/Luppy/ox64/apps/examples/hello_rust_install' does not exist
make -C /Users/Luppy/ox64/apps/examples/hello_rust install APPDIR="/Users/Luppy/ox64/apps"
make[3]: Entering directory '/Users/Luppy/ox64/apps/examples/hello_rust'
make[3]: *** No rule to make target 'hello_rust_main.rs.Users.Luppy.ox64.apps.examples.hello_rust.o', needed by '/Users/Luppy/ox64/apps/bin/hello_rust'.  Stop.
make[3]: Leaving directory '/Users/Luppy/ox64/apps/examples/hello_rust'
make[2]: *** [Makefile:52: /Users/Luppy/ox64/apps/examples/hello_rust_install] Error 2
make[2]: Leaving directory '/Users/Luppy/ox64/apps'
make[1]: *** [Makefile:78: .import] Error 2
make[1]: Leaving directory '/Users/Luppy/ox64/apps'
make: *** [Makefile:84: import] Error 2
```

Like QEMU, we change riscv64i to riscv64gc...

```bash
$ rustup target add riscv64gc-unknown-none-elf
$ pushd ../apps/examples/hello_rust 
$ rustc \
  --edition 2021 \
  --emit obj \
  -g \
  --target riscv64gc-unknown-none-elf \
  -C panic=abort \
  -O   hello_rust_main.rs \
  -o  hello_rust_main.rs.Users.Luppy.ox64.apps.examples.hello_rust.o
$ popd
$ make import
```

TODO: Fix the path of hello_rust.o

Let's test this...

# Change `hello_rust_main` to `main`

We test it with [Ox64 BL808 Emulator](https://lupyuen.github.io/articles/tinyemu3)...

```bash
+ riscv64-unknown-elf-objdump --syms --source --reloc --demangle --line-numbers --wide --debugging nuttx
+ cp /Users/Luppy/riscv/nuttx-tinyemu/docs/quickjs/root-riscv64.cfg .
+ /Users/Luppy/riscv/ox64-tinyemu/temu root-riscv64.cfg
TinyEMU Emulator for Ox64 BL808 RISC-V SBC

NuttShell (NSH) NuttX-12.4.0-RC0
nsh> hello_rust
nsh: hello_rust: command not found
```

_Huh? Why is hello_rust not found?_

To find out, we [Enable Logging for Binary Loader and Scheduler](https://github.com/lupyuen2/wip-nuttx/commit/dca29d561f44c4749c067b8304dc898b1c6c6e0c)...

```bash
CONFIG_DEBUG_BINFMT=y
CONFIG_DEBUG_BINFMT_ERROR=y
CONFIG_DEBUG_BINFMT_WARN=y
CONFIG_DEBUG_SCHED=y
CONFIG_DEBUG_SCHED_ERROR=y
CONFIG_DEBUG_SCHED_INFO=y
CONFIG_DEBUG_SCHED_WARN=y
```

[(root-riscv64.cfg is here)](https://github.com/lupyuen/nuttx-ox64/raw/main/nuttx.cfg)

Now it tells us why it failed...

```bash
+ riscv64-unknown-elf-objdump --syms --source --reloc --demangle --line-numbers --wide --debugging nuttx
+ cp /Users/Luppy/riscv/nuttx-tinyemu/docs/quickjs/root-riscv64.cfg .
+ /Users/Luppy/riscv/ox64-tinyemu/temu root-riscv64.cfg
TinyEMU Emulator for Ox64 BL808 RISC-V SBC
virtio_console_init
Patched DCACHE.IALL (Invalidate all Page Table Entries in the D-Cache) at 0x5020099a
Patched SYNC.S (Ensure that all Cache Operations are completed) at 0x5020099e
Found ECALL (Start System Timer) at 0x5020bfac
Patched RDTIME (Read System Time) at 0x5020bfb2
elf_len=0
virtio_console_resize_event
ABCnx_start: Entry
uart_register: Registering /dev/console
work_start_lowpri: Starting low-priority kernel worker thread(s)
nxtask_activate: lpwork pid=1,TCB=0x50409110
nxtask_activate: AppBringUp pid=2,TCB=0x50409710
nx_start_application: Starting init task: /system/bin/init
elf_symname: Symbol has no name
elf_symvalue: SHN_UNDEF: Failed to get symbol name: -3
elf_relocateadd: Section 2 reloc 2: Undefined symbol[0] has no name: -3
nxtask_activate: /system/bin/init pid=3,TCB=0x5040b730
nxtask_exit: AppBringUp pid=2,TCB=0x50409710

NuttShell (NSH) NuttX-12.4.0-RC0
nsh> nx_start: CPU0: Beginning Idle Loop

nsh> 
nsh> hello_rust
posix_spawn: pid=0x80202968 path=hello_rust file_actions=0x80202970 attr=0x80202978 argv=0x80202a18
elf_symname: Symbol has no name
elf_symvalue: SHN_UNDEF: Failed to get symbol name: -3
elf_relocateadd: Section 2 reloc 1: Undefined symbol[0] has no name: -3
elf_symvalue: SHN_UNDEF: Exported symbol "main" not found
elf_relocateadd: Section 2 reloc 4: Failed to get value of symbol[7684]: -2
elf_loadbinary: Failed to bind symbols program binary: -2
exec_internal: ERROR: Failed to load program 'hello_rust': -2
nxposix_spawn_exec: ERROR: exec failed: 2
nsh: hello_rust: command not found
nsh> 
```

[(root-riscv64.cfg is here)](https://github.com/lupyuen/nuttx-ox64/raw/main/nuttx.cfg)

hello_rust failed to load because the main() function is missing!

So we change this in hello_rust_main.rs...

```rust
pub extern "C" fn hello_rust_main(_argc: i32, _argv: *const *const u8) -> i32 {
```

To this...

```rust
pub extern "C" fn main(_argc: i32, _argv: *const *const u8) -> i32 {
```

Now our Rust App runs OK on Ox64 BL808 Emulator!

```bash
+ cp /Users/Luppy/riscv/nuttx-tinyemu/docs/quickjs/root-riscv64.cfg .
+ /Users/Luppy/riscv/ox64-tinyemu/temu root-riscv64.cfg
TinyEMU Emulator for Ox64 BL808 RISC-V SBC
virtio_console_init
Patched DCACHE.IALL (Invalidate all Page Table Entries in the D-Cache) at 0x5020099a
Patched SYNC.S (Ensure that all Cache Operations are completed) at 0x5020099e
Found ECALL (Start System Timer) at 0x5020bfac
Patched RDTIME (Read System Time) at 0x5020bfb2
elf_len=0
virtio_console_resize_event
ABCnx_start: Entry
uart_register: Registering /dev/console
work_start_lowpri: Starting low-priority kernel worker thread(s)
nxtask_activate: lpwork pid=1,TCB=0x50409110
nxtask_activate: AppBringUp pid=2,TCB=0x50409710
nx_start_application: Starting init task: /system/bin/init
elf_symname: Symbol has no name
elf_symvalue: SHN_UNDEF: Failed to get symbol name: -3
elf_relocateadd: Section 2 reloc 2: Undefined symbol[0] has no name: -3
nxtask_activate: /system/bin/init pid=3,TCB=0x5040b730
nxtask_exit: AppBringUp pid=2,TCB=0x50409710

NuttShell (NSH) NuttX-12.4.0-RC0
nsh> nx_start: CPU0: Beginning Idle Loop

nsh> hello_rust
posix_spawn: pid=0x80202968 path=hello_rust file_actions=0x80202970 attr=0x80202978 argv=0x80202a18
elf_symname: Symbol has no name
elf_symvalue: SHN_UNDEF: Failed to get symbol name: -3
elf_relocateadd: Section 2 reloc 1: Undefined symbol[0] has no name: -3
nxtask_activate: hello_rust pid=6,TCB=0x50409790
Hello, Rust!!
Hello Ox64!
You entered...
Hello Ox64!

nxtask_exit: hello_rust pid=6,TCB=0x50409790
nsh> 
```

[(root-riscv64.cfg is here)](https://github.com/lupyuen/nuttx-ox64/raw/main/nuttx.cfg)

# Test Rust Apps on Ox64 BL808 SBC

Our Rust App also works OK on a real Ox64 BL808 SBC!

https://gist.github.com/lupyuen/7fabbffd16f22914b299ced3723b9b9b

```bash
Enter choice: 1:.Pine64 0X64 Kernel
Retrieving file: /extlinux/../Image
append: root=PARTLABEL=rootfs rootwait rw rootfstype=ext4 console=ttyS0,2000000 loglevel=8 earlyextlinux/../bl808-pine64-ox64.dt## Flattened Device Tree blob at 51ff8000
   Booting using the fdt blob at 0x51ff8000
Working  51ff8000
   Loading Device Tree to 0000000053f22000, end 0000000053f25fab ... OK
Working FDT set to 53f22000

Starting kernel ...

ABCnx_start: Entry
uart_register: Registering /dev/console
work_start_lowpri: Starting low-priority kernel worker thread(s)
nxtask_activate: lpwork pid=1,TCB=0x50409110
nxtask_activate: AppBringUp pid=2,TCB=0x50409710
nx_start_application: Starting init task: /system/bin/init
elf_symname: Symbol has no name
elf_symvalue: SHN_UNDEF: Failed to get symbol name: -3
elf_relocateadd: Section 2 reloc 2: Undefined symbol[0] has no name: -3
nxtask_activate: /system/bin/init pid=3,TCB=0x5040b730
nxtask_exit: AppBringUp pid=2,TCB=0x50409710

NuttShell (NSH) NuttX-12.4.0-RC0
nsh> nx_start: CPU0: Beginning Idle Loop

nsh> 
nsh> hello_rust
posix_spawn: pid=0x80202968 path=hello_rust file_actions=0x80202970 attr=0x80202978 argv=0x80202a18
elf_symname: Symbol has no name
elf_symvalue: SHN_UNDEF: Failed to get symbol name: -3
elf_relocateadd: Section 2 reloc 1: Undefined symbol[0] has no name: -3
nxtask_activate: hello_rust pid=6,TCB=0x50409790
Hello, Rust!!

You entered...


nxtask_exit: hello_rust pid=6,TCB=0x50409790
nsh> 
```

# NuttX Flat Mode vs Kernel Mode

_Why the funny fixes for NuttX Ox64?_

Earlier we saw 2 workarounds for our Ox64 NuttX Build...

- We renamed the __Main Function__

- We fixed the __Makefile Target__

That's because __Ox64 Apps__ are a little more complicated than __QEMU Apps__...

__NuttX QEMU__ runs in __Flat Mode__ (pic below)

- NuttX Apps are __Statically Linked__ into NuttX Kernel

- __Main Functions__ for Apps are named _hello_main()_, _hello_rust_main()_, ...

- __No Memory Protection__ between Apps and Kernel

- Everything runs in __RISC-V Machine Mode__

- A little easier to troubleshoot

![NuttX Flat Mode](https://lupyuen.github.io/images/rust5-flat.jpg)

__NuttX Ox64__ runs in __Kernel Mode__ (pic below)

- NuttX Apps are __Separate ELF Files__

- __Main Functions__ for Apps are all named _main()_

- Apps and Kernel live in __Protected Memory Regions__

- Kernel runs in __RISC-V Supervisor Mode__

- Apps run in __RISC-V User Mode__

- More realistic for Actual Hardware

![NuttX Kernel Mode](https://lupyuen.github.io/images/rust5-kernel.jpg)

That's why our fixes for Ox64 are complicated than QEMU.
