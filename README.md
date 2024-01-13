# MLoader

## Compiling
### Prerequisites
- [CMake](https://cmake.org/)
- [WindowsSDK](https://developer.microsoft.com/en-us/windows/downloads/windows-sdk/)
- Compiler ([MSVC](https://visualstudio.microsoft.com/ru/), [Clang](https://clang.llvm.org/))
  
- [Rust](https://www.rust-lang.org/)

## Compiling from source
- Open **MLoader.code-workspace** in [Visual Studio Code](https://code.visualstudio.com/).
- Install dependencies

### Build C++

Configuration:
```console
cmake .
```

Build:
```console
cmake --build .
```

### Build Rust
Build:
```console
cargo build
```

Run:
```console
cargo run
```

## Libraries
### C++
- [BlackBone](https://github.com/DarthTon/Blackbone)
- [WNetWrap](https://github.com/hack-tramp/WNetWrap)

### Rust
- [serde](https://github.com/serde-rs/serde)
- [serde_json](https://github.com/serde-rs/json)
- [tokio](https://github.com/tokio-rs/tokio)
- [libloading](https://github.com/nagisa/rust_libloading/)
- [git2](https://github.com/rust-lang/git2-rs)
