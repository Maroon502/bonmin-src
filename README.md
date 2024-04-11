# Bonmin-src

[![Package][package-img]][package-url] [![Documentation][documentation-img]][documentation-url] [![License][license-img]][license-url]

## description

`Bonmin-src` crate is a *-src crate. This links [Bonmin] libraries to executable build by cargo, but does not provide Rust bindings. [Bomin] is built with [CoinUtils] ([CoinUtils-src]), [Osi] ([Osi-src]), [Cgl] ([Cgl-src]), [Clp] ([Clp-src]), [Cbc] ([Cbc-src]), [Ipopt] ([Ipopt-src]), [Mumps] ([Mumps-src])(Optional), [OpenBLAS] ([OpenBLAS-src])(Optional) and [Intel-mkl] ([intel-mkl-src])(Optional).

By this package, you don't need to worry about installing Bonmin in the system, and it's a package for **all platforms**.

[Bonmin] (Basic Open-source Nonlinear Mixed INteger programming) is an open-source code for solving general MINLP (Mixed Integer NonLinear Programming) problems.

## Usage

1. add the following to your `Cargo.toml`:

    ```toml
    [dependencies]
    bonmin-src = "\*"
    ```

2. add the following to your `lib.rs`:

    ```toml
    extern crate bonmin_src;
    ```

This package does not provide bindings. Please use [coinbonmin-sys], [coinipopt-sys], [coincbc-sys], [coinclp-sys] to use Bonmin, Ipopt, Cbc, Clp, e.g.

```toml
[dependencies]
coinbonmin-sys = { version = "\*" }
```

> Note: The C interface is taken from the master branch of [Bonmin], which is still has a number of todos.

## Configuration

### Features

The following Cargo features are supported:

* `default`;
* `cplex` to build with cplex(Need to install cplex in the system);
* `filtersqp` to build with filtersqp(Need to install filtersqp in the system);

### Environment

The package build from the source and link statically by default. It also provide the following environment variables to allow users to link to system library customly:

* `CARGO_BONMIN_STATIC` to link to Bonmin statically;
* `CARGO_BONMIN_SYSTEM` to link to Bonmin system library;
* `CARGO_IPOPT_STATIC` to link to Ipopt statically;
* `CARGO_IPOPT_SYSTEM` to link to Ipopt system library;
* `CARGO_MUMPS_STATIC` to link to Mumps statically;
* `CARGO_MUMPS_SYSTEM` to link to Mumps system library;
* `CARGO_COINUTILS_STATIC` to link to CoinUtils statically;
* `CARGO_COINUTILS_SYSTEM` to link to CoinUtils system library;
* `CARGO_OSI_STATIC` to link to Osi statically;
* `CARGO_OSI_SYSTEM` to link to Osi system library;
* `CARGO_CLP_STATIC` to link to Clp statically;
* `CARGO_CLP_SYSTEM` to link to Clp system library;
* `CARGO_CGL_STATIC` to link to Cgl statically;
* `CARGO_CGL_SYSTEM` to link to Cgl system library;
* `CARGO_CBC_STATIC` to link to Cbc statically;
* `CARGO_CBC_SYSTEM` to link to Cbc system library;

Set the environment variable to `1` to enable the feature. For example, to link to system library dynamically, set `CARGO_${LIB_NAME}_SYSTEM` to `1`; to link to system library statically, set both `CARGO_${LIB_NAME}_SYSTEM` and `CARGO_${LIB_NAME}_STATIC` to `1`.

If you enable OpenBLAS([openblas-src]), you can also pass env to `make` by `OPENBLAS_*`. Read more at [here](#cross-compilation)

### Others

```toml
bonmin-src = { version = "\*", default-features = no }
ipopt-src = { version = "\*", features = ["intel-mkl", "intel-mkl-lp64-seq"] }
```

ref: [intel-mkl-src], [intel-mkl-src], [ipopt-src], [mumps-src]

## Windows and vcpkg

On Windows, openblas need [vcpkg] to find Bonmin. Before building, you must have the correct Bonmin installed for your target triplet and kind of linking. For instance, to link dynamically for the `x86_64-pc-windows-msvc` toolchain, install  `bonmin` for the `x64-windows` triplet:

```sh
vcpkg install bonmin --triplet x64-windows
```

To link Bonmin statically, install `bonmin` for the `x64-windows-static-md` triplet:

```sh
vcpkg install bonmin --triplet x64-windows-static-md
```

To link Bonmin and C Runtime (CRT) statically, install `bonmin` for the `x64-windows-static` triplet:

```sh
vcpkg install bonmin --triplet x64-windows-static
```

and build with `+crt-static` option

```sh
RUSTFLAGS='-C target-feature=+crt-static' cargo build --target x86_64-pc-windows-msvc
```

Please see the ["Static and dynamic C runtimes" in The Rust reference](https://doc.rust-lang.org/reference/linkage.html#static-and-dynamic-c-runtimes) for detail.

## Cross Compilation

If you use OpenBLAS([openblas-src]), you need to set `OPENBLAS_CC`, `OPENBLAS_FC`, `OPENBLAS_HOSTCC`, and `OPENBLAS_TARGET` to pass env to [OpenBLAS], ref:[openblas-src] and [OpenBLAS]. For example:

```sh
export OPENBLAS_TARGET=ARMV8
export OPENBLAS_HOSTCC=gcc
export OPENBLAS_CC=aarch64-linux-gnu-gcc
export OPENBLAS_FC=aarch64-linux-gnu-gfortran
```

you can compile it for the other target by providing the `--target` option to `cargo build`.

| Target                               |  supported  |
|--------------------------------------|:-----------:|
| `arm-unknown-linux-gnueabi`          | ✓   |
| `arm-unknown-linux-gnueabihf`        | ✓   |
| `armv7-unknown-linux-gnueabi`        | ✓   |
| `armv7-unknown-linux-gnueabihf`      | ✓   |
| `armv7-unknown-linux-musleabi`       | ✓   |
| `armv7-unknown-linux-musleabihf`     | ✓   |
| `aarch64-unknown-linux-gnu`          | ✓   |
| `aarch64-unknown-linux-musl`         | ✓   |
| `riscv64gc-unknown-linux-gnu`        | ✓   |
| `x86_64-pc-windows-msvc`             | ✓   |
| `x86_64-unknown-linux-gnu`           | ✓   |
| others                               | not test   |

Note: `intel-mkl-*` can only be used for `x86_64-*`.`openblas-static` can only be used for `linux`.

## Contribution

Your contribution is highly appreciated. Do not hesitate to open an issue or a
pull request. Note that any contribution submitted for inclusion in the project
will be licensed according to the terms given in [LICENSE](license-url).

[CoinUtils]: https://github.com/coin-or/CoinUtils
[Osi]: https://github.com/coin-or/Osi
[Cgl]: https://github.com/coin-or/Cgl
[Clp]: https://github.com/coin-or/Clp
[Cbc]: https://github.com/coin-or/Cbc
[Ipopt]: https://github.com/coin-or/Ipopt
[Bonmin]: https://github.com/coin-or/Bonmin
[Mumps]: https://mumps-solver.org/
[OpenBLAS]: https://github.com/OpenMathLib/OpenBLAS
[intel-mkl]: https://www.intel.com/content/www/us/en/developer/tools/oneapi/onemkl.html

[CoinUtils-src]: https://github.com/Maroon502/coinutils-src
[Cgl-src]: https://github.com/Maroon502/cgl-src
[Clp-src]: https://github.com/Maroon502/clp-src
[Osi-src]: https://github.com/Maroon502/osi-src
[Ipopt-src]: https://github.com/Maroon502/ipopt-src
[Mumps-src]: https://github.com/Maroon502/mumps-src
[OpenBLAS-src]: https://github.com/blas-lapack-rs/openblas-src
[intel-mkl-src]: https://github.com/rust-math/intel-mkl-src

[coincbc-sys]: https://github.com/Maroon502/coincbc-sys
[coinclp-sys]: https://github.com/Maroon502/coinclp-sys
[coinipopt-sys]: https://github.com/Maroon502/coinipopt-sys
[coinbonmin-sys]: https://github.com/Maroon502/coinbonmin-sys

[vcpkg]: https://github.com/Microsoft/vcpkg

[documentation-img]: https://docs.rs/bonmin-src/badge.svg
[documentation-url]: https://docs.rs/bonmin-src
[package-img]: https://img.shields.io/crates/v/bonmin-src.svg
[package-url]: https://crates.io/crates/bonmin-src
[license-img]: https://img.shields.io/crates/l/bonmin-src.svg
[license-url]: https://github.com/Maroon502/bonmin-src/blob/master/LICENSE.md
