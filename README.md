
# ZeroMin

ZeroMin is a rust static library for RAN Online, moving the algorithms from C++ to Rust in form of SDK. Making it available for use in other softwares.

## Goals

ZeroMin's goal is to move some algorithms of RAN Online into Rust making it memory safe, fast and usable in other platforms.

### Priorities

- Make the SDK/FFI as stable as possible.
- Crossplatform Client (Not that high of a priority as there is no good DirectX Wrapper for Rust atm.)
- Porting tools to Rust (ItemEditor, SkillEditor and so on).

### Compilation

Note: The FFI Bindings are automatically generated however it would be missing callbacks for the Async Functions
so you need to re-implement it in your header file.

```bash
cargo build --release --target i686-pc-windows-msvc
```

- The library should be under target->i686-pc-windows-msvc->zeromin.lib
- The FFI header should be under target->zeromin->zerominffi.h

## Code of Conduct

**IMPORTANT** : I will not merge your works if you do not follow my rules.

- Create branch from branch master for each tasks / issues that you want to work (one branch, one task / issue).
- The new branch name should be same as task / issue title.
- Provide details for every commit you push.
- Provide change logs in every merge request detail.

## How to Contribute

- Must have knowledge in C++ and Rust.
- Willing to work for the community (for free of course).
