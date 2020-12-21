## Moonlight - a cross-platform mobile framework written in Rust (work in progress)

This started as a proof of concept, but it has potential as there isn't any mobile framework written in Rust, so I am going to continue developing it when I have free time.

Pull requests welcome! If you want to help me build this project, let me know so I can explain what I have in mind for every item in the roadmap.

# Installation
0. Make sure you have the latest versions of Xcode and Rust (+ Cargo)
1. Clone the repo
2. Install `cargo-lipo` (i.e. `cargo install cargo-lipo`)
3. Make sure `xcode-select -p` points to your Xcode location, NOT the command line tools Homebrew loves to install
3. Add the iOS targets to `rustup` (i.e. `rustup target add aarch64-apple-ios x86_64-apple-ios`)
4. Build the library (i.e. `cargo lipo` or `cargo lipo --release`)
5. Build and run the Xcode project

# Done until now (iOS)
- [x] Passing data to Swift through a C bridging header
- [x] Passing data back to Rust for deallocation
- [x] Basic view structure with traits

# Roadmap
- [ ] Reactive views
- [ ] Supporting Android
- [ ] Translating everything from UIKit and Android SDK
- [ ] Plugin support
- [ ] Better file organization
- [ ] Testing
- [ ] Better debugging tools - surprisingly, Xcode has some kind of debug support for Rust files (it shows the line with the exception), but it's not enough
- [ ] Better exception handling
- [ ] Better tree shaking and inlining
- [ ] Writing a CLI - preferably in Rust
- [ ] Writing a website