# cross-compile rust app with opencv from linux to windows under nixos

Complete working app that uses native opencv4 library and opencv-rust with cross-compilation.

Some explanations how it works:
- first of all you need opencv library compiled for windows. Nixpkgs does not provide one so I copied package file
from nixpkgs-unstable and modified it to able to compile with mingw32-w64. If you need opencv with more options enabled
you should adjust package yourself (see `opencv-win.nix`).

- opencv-rust needs clang to generate bindings but we does not want it to override build environment ($CC variable etc)
so we just place it as dependency in `depsBuildBuild`.

- opencv-rust can use several methods to find opencv files which it uses to generate bindings. In my case using env
variables for opencv-rust was enough (`OPENCV_INCLUDE_PATHS`, `OPENCV_LINK_PATHS` and `OPENCV_LINK_LIBS`). For other
probes variants see opencv-rust documentation. 

- as we define env variables for opencv-rust we does not need to further define additional linker options for our app to build because opencv-rust will propogate correct options.

- after opencv-rust generates bindings for us we need to compile it with mingw32-w64 or it will fail to link. For this we use `pkgsMingw.mkShell` to enter shell with `nix develop` command with prepared build environment with mingw32-w64 compiler. After that opencv-rust will use correct compiler. Probably it is possible to use correct overrides in `.cargo/config.toml` but I did not test it.

- In later stages of building opencv-rust object files will require mcfgthreads library when linking. For this we define `RUSTFLAGS` env variable for rustc/cargo to pass some linker options to underlying linker/compiler.

- Your app will require to link against pthreads BUT we cannot just provide it in `RUSTFLAGS` because opencv-rust builds executable `bindings-generator` and it links against pthreads too but it is linux app and will try to use incorrect pthreads and fail. For correct linking we propagate path for mingw32-w64 pthreads by env variable into our build.rs file and instruct cargo to use it for pthreads.

- At this stage your app should be compiled successfully (if I did not forget something) so you need to run it somehow.
For this wine are used with `CARGO_TARGET_X86_64_PC_WINDOWS_GNU_RUNNER` env variable. Also we need to provide additional paths for runtime dependencies (dlls). For that we define `WINEPATH` env variable with paths from compiler and libraries we used.

App builds and runs under wine in both debug and release builds. 

This repository does not provides some ready nixpkgs package for resulting rust app
but it is good starting point and I think it will not be that hard.

## Easier way
If you don't interested in packaging your app you can use easier way through [cross-rs](https://github.com/cross-rs/cross).

## Used links
- https://github.com/twistedfall/opencv-rust
- https://nixos.org/guides/cross-compilation.html
- https://nixos.org/manual/nixpkgs/stable/#chap-cross
- https://github.com/jraygauthier/jrg-rust-cross-experiment
- https://nixos.org/guides/nix-pills/