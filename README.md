![Doom Image](./assets/doom.webp)
---
Engine made using SDL2 v0.37.0.

# Doom Engine in Rust 🦀
I started this project in order to learn Rust, so I'm probably not using the best practices and recommendations for the language.

Based on the video made by [The Old School Coder](https://www.youtube.com/watch?v=p7f9p9nDsmc), where he implements the engine using C, so the credit for the implementation is all his. 
- Original Code in C written by [jeuxdemains](https://github.com/jeuxdemains/DOOM-like-game-engine-part-I/tree/main)
- **This project is a rewrite in Rust**.

## 📚 Requeriments
    - Rust
    - SDL2 Library


## 📖 How to use

##### 1) Install SDL2 

- [SDL2 Crate v0.37.0](https://crates.io/crates/sdl2)
- [Rust-SDL2 Documentation](https://rust-sdl2.github.io/rust-sdl2/sdl2/)

```bash
# Linux
# Ubuntu Example
sudo apt-get install libsdl2-dev
# Fedora example
sudo dnf install SDL2-devel

# macOS
# Using Homebrew
# On macOS, it's a good idea to install these via homebrew.
brew install sdl2

# Add SDL2 to your project using cargo
cargo add sdl2
```

##### 2) Run the project
```bash
# Run
cargo run

# Build
cargo build
```
