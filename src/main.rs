mod sudoku;

use sudoku::Sudoku;

fn main() {
    let mut s = Sudoku::new();
    s.static_init();
    s.solve();
}
