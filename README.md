# sudoku-rs

I have created a program in Rust that solves Sudoku puzzles.
And I have created my first Markdown document.

## Usage
```rust
# new instance.
let mut s = Sudoku::new();

# Setting up the initial state.
s.static_init();

# Solving Sudoku.
s.solve();
```

![Start](/docs/2023-08-24%20175352.png)
![End](/docs/2023-08-24%20175259.png)