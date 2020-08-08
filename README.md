# Rust - Amethyst - Genetic Algorithm

This project was an exploration of the Amethyst game engine, as well as of 
the Rust programming language. Both were really fun! 

I also made this project using nvim exclusively, which was a text editor
I was eager to learn more about.

## How to run

To run the game, run the following command, which defaults to the `metal` graphics backend:

```bash
cargo run
```

Windows and Linux users may explicitly choose `"vulkan"` with the following command:

```bash
cargo run --no-default-features --features "vulkan"
```

Mac OS X users may explicitly choose `"metal"` with the following command:

```bash
cargo run --no-default-features --features "metal"
```
