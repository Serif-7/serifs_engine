

/*
* #TYPES
*
* Game: represents a full game state
* Board: represents a chessboard
* Piece: represents a chess piece
*
* 
* */

use std::ops::IndexMut;
use std::ops::Index;

#[derive(Eq)]
pub struct Coord(u8, u8);

impl Coord {

    pub fn into_sq(&self) -> u8 {
            return self.0 * 8 + self.1;
        }
    

    pub fn new(x: u8, y: u8) -> Option<Coord> {
        if (x < 0 || x > 7 || y < 0 || y > 7) {
            return None;
        }
        else {
            return Some(Coord(x, y));
        }

    }

    pub fn new(sq: u8) -> Coord {
        
        return Coord(sq & (!7u8), sq & 7u8);
    }

}


#[derive(Clone, PartialEq)]
pub enum Color {
    Black,
    White,
}

#[derive(Clone)]
pub enum PieceType {
    Pawn,
    Rook,
    Bishop,
    Knight,
    Queen,
    King,
}

#[derive(Clone)]
pub struct Piece {
    pub color: Color,
    pub ptype: PieceType,
}


//full game state
//making all the fields public should be easier...
#[derive(Clone)]
pub struct Game {
    pub board: [Option<Piece>; 64],
    pub moves: i32,
    //castle rights
    pub bqc: bool,
    pub bkc: bool,
    pub wqc: bool,
    pub wkc: bool,

}



impl Game {

    //mutates self
    pub fn make_move(&mut self, src: u8, dest: u8) {

        let attacker = self.board.get(src).take();
        self.board.get(dest).replace(attacker.unwrap());
    }

    //return a new board with the made move
    pub fn make_move_clone(self, src: u8, dest: u8,) -> [Option<Piece>; 64] {

        let mut g = self.clone();
        g.make_move(src, dest);
        return g.board;
    }


    //coords might be wrong, check later
    pub fn castle(&mut self, c: Color, kingside: bool) {
        
        match (c, kingside) {

            (Color::White, true) => {
                //move king
                self.make_move(3, 1);
                //move rook
                self.make_move(0, 2);
            }
            (Color::White, false) => {
                self.make_move(3, 5);
                self.make_move(7, 4);
            }
            (Color::Black, true) => {
                self.make_move(59, 57);
                self.make_move(56, 58);
            }
            (Color::Black, false) => {
                self.make_move(59, 61);
                self.make_move(63, 60);
            }

        }
    }

    //## ENUMERATION LOGIC
    
    fn enumerate_moves(game: Game, c: Color) {

        let mut board_states: Vec<[Option<Piece>; 64]>;

        for (i, sq) in game.board.iter().enumerate() {

            if let Some(sq) = sq {

                if sq.color == c {

                    board_states.append(&mut Self::enumerate_piece(game, sq, i))

                }
            }
        }
    }

    //enumerate all moves for a given piece
    fn enumerate_piece(game: Game, p: Piece, sq: u8) -> Vec<[Option<Piece>; 64]> {

        let mut board_states: Vec<[Option<Piece>; 64]>; 

        match p.ptype {
            PieceType::Pawn => board_states.append(&mut Self::enumerate_pawn(game, p.color, sq)),
            PieceType::Rook => (),
            PieceType::Bishop => (),
            PieceType::King => (),
            PieceType::Knight => (),
            PieceType::Queen => (),

        }

        return board_states;
    }

    fn enumerate_pawn(game: Game, c: Color, sq: u8 ) -> Vec<[Option<Piece>; 64]> {

        let mut board_states: Vec<[Option<Piece>; 64]>;


        if c == Color::Black {

        }

        else {

        }

        let mut g = game.clone();
        g.make_move(sq, 1);

        return board_states;


    }

    fn enumerate_knight(&self, c: Color, sq: u8) {

        /* let mut board_states: Vec<[Option<Piece>; 64]>;

        match sq {

            (sq > 16) => board_states.push(self.make_move_clone(sq, sq - 17)),
            _ => (),

        } */



    }



    pub fn print(&self) -> () {

        let mut count = 0;

        for p in self.board {

            if let Some(p) = p {

                match (&p.ptype, &p.color) {
                    //white pieces
                    (PieceType::Rook, Color::White) => print!("♜ "),
                    (PieceType::Knight, Color::White) => print!("♞ "),
                    (PieceType::Bishop, Color::White) => print!("♝ "),
                    (PieceType::King, Color::White) => print!("♚ "),
                    (PieceType::Queen, Color::White) => print!("♛ "),
                    (PieceType::Pawn, Color::White) => print!("♟ "),
                    //black pieces
                    (PieceType::Rook, Color::Black) => print!("♖ "),
                    (PieceType::Knight, Color::Black) => print!("♘  "),
                    (PieceType::Bishop, Color::Black) => print!("♗ "),
                    (PieceType::King, Color::Black) => print!("♔ "),
                    (PieceType::Queen, Color::Black) => print!("♕ "),
                    (PieceType::Pawn, Color::Black) => print!("♙ "),

                }
            }

            else {
                print!(" ");
            }

            match count {
                7|15|31|39|47|55|63 => println!(""),
                _ => (),
            }
            
            count += 1;

        }

    }

    //verify that a set of coordinates represent an actual square
/*     pub fn verify(src: (usize, usize), dest: (usize, usize) ) -> bool {

        match src {
            x > 63 => return false,
            x < 0 => return false,
            y > 63 => return false,
            y < 0 => return false,
            _ => (),

        }


    } */
}



impl Default for Game {
    fn default() -> Self {
        return Game {
            board: init_board(),
            moves: 0,
            bqc: true,
            bkc: true,
            wqc: true,
            wkc: true,

        }

    }
}


//return an initial chessboard state
//8x8 mailbox representation
//lower numbers are White
fn init_board() -> [Option<Piece>; 64] {

    let mut board: [Option<Piece>; 64];

    let mut count: usize = 0;

    for piece in board {

        match count {
            //set white pieces
            // 0 | 7 => *sq.unwrap() = Some(Piece::new(Color::White, PieceType::Rook)),
            0 | 7 => piece = Some(Piece { color: Color::White, ptype: PieceType::Rook}),
            1 | 6 => piece = Some(Piece { color: Color::White, ptype: PieceType::Knight }),
            2 | 5 => piece = Some(Piece { color: Color::White, ptype: PieceType::Bishop}),
            3 => piece = Some(Piece { color: Color::White, ptype: PieceType::Queen}),
            4 => piece = Some(Piece { color: Color::White, ptype: PieceType::King}),
            8..=15 => piece = Some(Piece { color: Color::White, ptype: PieceType::Pawn}),
            //set black pieces
            48..=55 => piece = Some(Piece { color: Color::Black, ptype: PieceType::Pawn}),
            56 | 63 => piece = Some(Piece { color: Color::Black, ptype: PieceType::Rook}),
            57 | 62 => piece = Some(Piece { color: Color::Black, ptype: PieceType::Knight}),
            58 | 61 => piece = Some(Piece { color: Color::Black, ptype: PieceType::Bishop}),
            59 => piece = Some(Piece { color: Color::Black, ptype: PieceType::Queen}),
            60 => piece = Some(Piece { color: Color::Black, ptype: PieceType::King}),
            _ => (),
        }

        count += 1;
    }

    return board;
}

