
use crate::game::{
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
        PieceType::Rook => (),
        PieceType::Bishop => (),
        PieceType::King => (),
        PieceType::Knight => (),
        PieceType::Queen => (),
        
    }

    return board_states;
}


fn enumerate_pawn(game: Game, c: Color, sq: u8 ) {

    if c == Color::Black {

    }

    else {

    }

    let mut g = game.clone();
    g.make_move(sq, 1);

    
}

fn enumerate_knight(game: Game, c: Color, sq: u8) {
    
    let mut board_states: Vec<[Option<Piece>; 64]>;



}
