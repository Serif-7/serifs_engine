
mod game;
mod search;


/* accessing off board returns 'None' so currently there is no way to distinguish between off board
* and an empty square. 
*
* TODO: Add an 'empty' type to the piecetype enum and rework the code to work that way.
 */

fn main() {
    // print!("Hello");

    let mut game = game::Game::default();

    let board = &game.board;

    game.print();

}



