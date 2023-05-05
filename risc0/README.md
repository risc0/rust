# Rics0 toolchain release scripts

These are scripts to build a release for the risc0 toolchain.  Ideally
we can eventually merge the changes we need upstream and decommission
this process.

Since we build both "rustc" and "cargo", we want to build those for
all host architectures we can.

Build scripts will copy artifacts to CLOUD_STAGING (specified in
common.sh), and the publish script will make those available in
CLOUD_DIST, where "cargo risc0 update" can find them.

(CLOUD_STAGING and CLOUD_DIST are both specified as "rclone" path specifications)

All these scripts assume they'll be executed in the "risc0"
subdirectory of the "rust" tree.  For instance, run "./publish.sh"
instead of "risc0/publish.sh".

Since much of this infrastructure is designed to work within Rust's CI
environment, some harmless errors may show up, for instance:

```
WARNING: `toolstates-linux.json` missing/malformed; assuming all tools failed
Miri tests are not passing, removing component
```

## Updating to upstream

To integrate a new upstream rust release, merge upstream rust
(github.com/rust-lang/rust) into the risc0-toolchain branch on
github.com/risc0/rust.

If upstream has supplied a new release of cargo, you should probably
update cargo as well.  To do this, merge upstream
github.com/rust-lang/cargo into the "risc0-toolchain" branch on
github.com/risc0/cargo.  Then, update the "src/tools/cargo" submodule
to point at the new cargo HEAD on the risc0-toolchain branch.

## Building and testing locally

If you're not familiar with x.py and the rust compiler build system,
here is how to build a local toolchain to use:

* Make sure "lld" in installed ("apt install lld" or equivalent)

* Copy config.toml.risc0 to ../config.toml and edit "prefix" to point
  to where you want your local toolchain to be installed

* ```cd .. && ./x.py install compiler/rustc library/std cargo```

* Add an alias for this toolchain so you can use it.  For instance,
  ```rustup toolchain link dev-risc0 /home/YOURUSERNAME/rustroot```.

* Now you can build with this toolchain using e.g. "cargo +dev-risc0 build".

FIXME: For some reason this doesn't seem to install "rust-lld" in my
case.  However, copying rust-lld and libLLVM-...-nightly.so from
```~/.rustup/toolchains/*-x64_64-unknown-linux-gnu/...``` to
```~/.risc0/rustup/toolchains/*-x86_64-unknown-linux/gnu/...``` seems to work

## Building for distribution

### X86-based builds

Many architectures can be built in x86_64 docker containers.  To build
these, run:

```
./build-host-x86_64.sh <date>
```

TODO: Add other docker containers to compile for more than just
x86_64-unknown-linux-gnu.

### Mac OSX

Run:

```
./build-macos.sh <date>
```

TODO: implement

### Aarch64

Several architectures are aarch64 based and can be built in docker containers.  To build these, run:

```
./build-host-aarch64.sh <date>
```

TODO: Implement

## Microsoft Windows

Run:

```
./build-windows.sh <date>
```

TODO: implement

## Publishing

Once builds are completed for all architectures, you can publish all
the builds from staging by running:

```
./publish.sh <date>
```

# Long term TODO

* Submit compiler target for riscv32im-risc0-zkvm-elf target upstream

* Once rustc supports riscv32im-risc0-zkvm-elf and cargo bindeps hits
  stable (https://github.com/rust-lang/cargo/issues/9096) we can stop
  having to compile for all architecetures, and will only need to
  build rust-std.

* Once we finish getting our std library changes merged upstream, we
  can decommission this toolchain entirely.
