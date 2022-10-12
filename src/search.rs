use crate::main::LOOKUP;

use crate::game::{
    Coord,
    Piece,
    PieceType,
    Color,
    Game
};


//find the best move for a given color
pub fn search(game: Game, c: Color) {

    let board_states = enumerate_moves(game, c);
}

//enumerate all possible moves for a given color
fn enumerate_moves(game: Game, c: Color) {

    let mut board_states: Vec<[Option<Piece>; 64]>;
    
    for (i, sq) in game.board.iter().enumerate() {

        if let Some(sq) = sq {

            if sq.color == c {

                board_states.append(&mut enumerate_piece(game, sq, i))
                
            }
        }
    }
}

//enumerate all moves for a given piece
fn enumerate_piece(game: Game, p: Piece, sq: u8) -> Vec<[Option<Piece>; 64]> {

    let mut board_states: Vec<[Option<Piece>; 64]>; 

    match p.ptype {
        PieceType::Pawn => board_states.append(enumerate_pawn(game, p.color, sq)),
        PieceType::Rook => board_states.append(enumerate_rook(game, p.color, ),
        PieceType::Bishop => (),
        PieceType::King => (),
        PieceType::Knight => (),
        PieceType::Queen => (),
        
    }

    return board_states;
}

//TODO: check for enemy pieces before adding attack coords to list
//TODO: implement promotion special case
fn enumerate_pawn(game: Game, c: Color, sq: u8 ) {

    let co = Coord::new(x, y);

    let mut board_states: Vec<[Option<Piece>; 64]>;

    let mut coords = Vec::new();

    if c == Color::Black {

        //need to guarantee this doesn't error
        //if there is no piece in front of it
        if let None = game.board[sq - 8] {
            coords.append(Coord::new(co.0 - 1, co.1 - 1));

            //double pawn move
            if let None = game.board[sq - 16]  && co.0 == 2 {
                coords.append(Coord::new(co.0, co.1 - 2));
            }
        }


        //attack moves
        coords.append(&mut vec![Coord::new(co.0 - 1, co.1 - 1), Coord::new(co.0 + 1, co.1 + 1)]);

    }

    else {
    }


    //filter Nones out of coords

    for x in coords {

        let mut g = game.clone();
        g.make_move_clone(co.into_sq(), x.into_sq());
    }

    return board_states;

}

fn enumerate_knight(game: Game, c: Color, co: Coord) {
    
    let mut board_states: Vec<[Option<Piece>; 64]>;
        



}
