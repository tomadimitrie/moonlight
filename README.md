## Moonlight - a cross-platform mobile framework written in Rust (work in progress)

This started as a proof of concept, but it has potential as there isn't any mobile framework written in Rust, so I am going to continue developing it when I have free time.

Pull requests welcome! If you want to help me build this project, let me know so I can explain what I have in mind for every item in the roadmap.

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