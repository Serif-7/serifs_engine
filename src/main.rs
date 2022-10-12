use game::Coord;

mod game;
mod search;

/*
*
* TODO: Move enumeration (Pseudo legal)
* TODO: Filter out illegal moves
* TODO: Search algorithm
*/

pub static LOOKUP: Lazy<[Coord; 64]> = Lazy::new(|| {
    let mut a =  [Coord; 64];
    for i in 0..8 {

        for j in 0..8 {
            
            let coord = Coord(i, j);
            let sq = i * 8 + j;
            a[sq] = coord;
        }
    }
});

fn main() {

    let mut game = game::Game::default();

    let board = &game.board;

    game.print();

}



