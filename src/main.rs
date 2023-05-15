use tetrimino::{Tetrimino, print_tetrimino};
mod tetrimino;

fn main() {
    let tetriminos = [
        Tetrimino::O, Tetrimino::I, Tetrimino::T, Tetrimino::L,
        Tetrimino::J, Tetrimino::S, Tetrimino::Z
    ];

    tetriminos.iter().for_each(|t| {
        println!("--------");
        for n in 0..=3 {
            print_tetrimino(t, n);
            println!("--------");
        }
    });
}