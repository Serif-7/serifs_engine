use grid::Grid;


fn main() {
    // print!("Hello");

    let mut game = Game::default();

    let board = &game.board;

    game.print();

}


enum Color {
    Black,
    White,
}

enum PieceType {
    Pawn,
    Rook,
    Bishop,
    Knight,
    Queen,
    King,
}

struct Piece {
    color: Color,
    ptype: PieceType,
}

//full game state
struct Game {
    board: grid::Grid<Option<Piece>>,
    moves: i32,
    bqc: bool,
    bkc: bool,
    wqc: bool,
    wkc: bool,

}


impl Game {

    pub fn make_move(&mut self, src: (usize, usize), dest: (usize, usize)) {

        let (x, y) = src;
        let (a, b) = dest;

        let attacker = self.board.get(x, y).take();
        self.board.get(a, b).replace(attacker.unwrap());
    }


    //coords might be wrong, check later
    pub fn castle(&mut self, c: Color, kingside: bool) {
        
        match (c, kingside) {

            (Color::White, true) => {
                //move king
                self.make_move((0, 4), (0, 6));
                //move rook
                self.make_move((0, 0), (0, 2));
            }
            (Color::White, false) => {
                self.make_move((0, 4), (0, 2));
                self.make_move((0, 7), (0, 5));
            }
            (Color::Black, true) => {
                self.make_move((7, 4), (7, 2));
                self.make_move((7, 0), (7, 3));
            }
            (Color::Black, false) => {
                self.make_move((7, 4), (7, 6));
                self.make_move((7, 7), (7, 5));
            }

        }
    }

    //enumerate possible board states for a given side
/*     pub fn enumerate_moves(&self, c: Color) -> Vec<grid::Grid<Option<Piece>>> {
        let mut iter = self.board.iter();

        let mut count = 0;

        while count < 64 {
        }



    } */

    //TODO
    pub fn print(&self) -> () {

        let board = self.board.flatten();

        let mut count = 0;

        for sq in board {

            //sq is a double reference
            if let Some(sq) = sq {

                match (&sq.ptype, &sq.color) {
                    //white pieces
                    (PieceType::Rook, Color::White) => print!("♖ "),
                    (PieceType::Knight, Color::White) => print!("♘ "),
                    (PieceType::Bishop, Color::White) => print!("♗ "),
                    (PieceType::King, Color::White) => print!("♔ "),
                    (PieceType::Queen, Color::White) => print!("♕ "),
                    (PieceType::Pawn, Color::White) => print!("♙ "),
                    //black pieces
                    (PieceType::Rook, Color::Black) => print!("♜ "),
                    (PieceType::Knight, Color::Black) => print!("♞ "),
                    (PieceType::Bishop, Color::Black) => print!("♝ "),
                    (PieceType::King, Color::Black) => print!("♚ "),
                    (PieceType::Queen, Color::Black) => print!("♛ "),
                    (PieceType::Pawn, Color::Black) => print!("♟ "),

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
fn init_board() -> grid::Grid<Option<Piece>> {

    let mut board: grid::Grid<Option<Piece>> = Grid::new(8, 8);

    let mut count: usize = 0;

    let mut iter = board.iter_mut();

    while count < 64 {
        let sq = iter.next();

        match count {
            //set white pieces
            // 0 | 7 => *sq.unwrap() = Some(Piece::new(Color::White, PieceType::Rook)),
            0 | 7 => *sq.unwrap() = Some(Piece { color: Color::White, ptype: PieceType::Rook}),
            1 | 6 => *sq.unwrap() = Some(Piece { color: Color::White, ptype: PieceType::Knight }),
            2 | 5 => *sq.unwrap() = Some(Piece { color: Color::White, ptype: PieceType::Bishop}),
            3 => *sq.unwrap() = Some(Piece { color: Color::White, ptype: PieceType::Queen}),
            4 => *sq.unwrap() = Some(Piece { color: Color::White, ptype: PieceType::King}),
            8..=15 => *sq.unwrap() = Some(Piece { color: Color::White, ptype: PieceType::Pawn}),
            //set black pieces
            48..=55 => *sq.unwrap() = Some(Piece { color: Color::Black, ptype: PieceType::Pawn}),
            56 | 63 => *sq.unwrap() = Some(Piece { color: Color::Black, ptype: PieceType::Rook}),
            57 | 62 => *sq.unwrap() = Some(Piece { color: Color::Black, ptype: PieceType::Knight}),
            58 | 61 => *sq.unwrap() = Some(Piece { color: Color::Black, ptype: PieceType::Bishop}),
            59 => *sq.unwrap() = Some(Piece { color: Color::Black, ptype: PieceType::Queen}),
            60 => *sq.unwrap() = Some(Piece { color: Color::Black, ptype: PieceType::King}),
            _ => (),
        }

        count += 1;
    }

    return board;
}
