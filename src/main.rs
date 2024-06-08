use std::fmt;

#[derive(Copy, Clone)]
enum Color {
    White,
    Black,
}

#[derive(Copy, Clone)]
enum Piece {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}

#[derive(Copy, Clone, Default)]
enum Square {
    #[default]
    Empty,
    Value(Piece, Color),
}

impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Square::Empty => write!(f, "   "),
            Square::Value(Piece::King, Color::White) => write!(f, " ♔ "),
            Square::Value(Piece::King, Color::Black) => write!(f, " ♚ "),

            Square::Value(Piece::Queen, Color::White) => write!(f, " ♕ "),
            Square::Value(Piece::Queen, Color::Black) => write!(f, " ♛ "),

            Square::Value(Piece::Bishop, Color::White) => write!(f, " ♗ "),
            Square::Value(Piece::Bishop, Color::Black) => write!(f, " ♝ "),

            Square::Value(Piece::Knight, Color::White) => write!(f, " ♘ "),
            Square::Value(Piece::Knight, Color::Black) => write!(f, " ♞ "),

            Square::Value(Piece::Rook, Color::White) => write!(f, " ♖ "),
            Square::Value(Piece::Rook, Color::Black) => write!(f, " ♜ "),

            Square::Value(Piece::Pawn, Color::White) => write!(f, " ♙ "),
            Square::Value(Piece::Pawn, Color::Black) => write!(f, " ♟︎ "),
        }
    }
}

#[derive(Copy, Clone)]
struct Game {
    board: [Square; 64],
}

//board : [Square::Empty; 64]
impl Default for Game {
    fn default() -> Self {
        Game {
            board: [Square::Empty; 64],
        }
    }
}

impl Game {
    fn GetPos(&self, a: &str) -> usize {
        if a.len() != 2 {
            return 0; // throw ?
        }
        let (b, c) = a.split_at(a.len() / 2);
        let row = (c.parse::<u16>().unwrap() - 1) * 8; // difference between 0 index and 1 index
        let mut col: u16 = 0;
        match b {
            "a" => col = 0,
            "b" => col = 1,
            "c" => col = 2,
            "d" => col = 3,
            "e" => col = 4,
            "f" => col = 5,
            "g" => col = 6,
            "h" => col = 7,
            _ => col = 8, // Error throwing?
        }

        return usize::from(row + col);
    }

    fn Setup(&mut self) {
        // King
        self.board[self.GetPos("e1")] = Square::Value(
            Piece::King,
            Color::White,
        );

        self.board[self.GetPos("e8")] = Square::Value(
            Piece::King,
            Color::Black,
        );

        // Queen
        self.board[self.GetPos("d1")] = Square::Value(
            Piece::Queen,
            Color::White,
        );

        self.board[self.GetPos("d8")] = Square::Value(
            Piece::Queen,
            Color::Black,
        );

        // Rooks
        self.board[self.GetPos("a1")] = Square::Value(
            Piece::Rook,
            Color::White,
        );

        self.board[self.GetPos("a8")] = Square::Value(
            Piece::Rook,
            Color::Black,
        );

        self.board[self.GetPos("h1")] = Square::Value(
            Piece::Rook,
            Color::White,
        );

        self.board[self.GetPos("h8")] = Square::Value(
            Piece::Rook,
            Color::Black,
        );

        // Knights
        self.board[self.GetPos("b1")] = Square::Value(
            Piece::Knight,
            Color::White,
        );

        self.board[self.GetPos("b8")] = Square::Value(
            Piece::Knight,
            Color::Black,
        );

        self.board[self.GetPos("g1")] = Square::Value(
            Piece::Knight,
            Color::White,
        );

        self.board[self.GetPos("g8")] = Square::Value(
            Piece::Knight,
            Color::Black,
        );

        // Bishops
        self.board[self.GetPos("c1")] = Square::Value(
            Piece::Bishop,
            Color::White,
        );

        self.board[self.GetPos("c8")] = Square::Value(
            Piece::Bishop,
            Color::Black,
        );

        self.board[self.GetPos("f1")] = Square::Value(
            Piece::Bishop,
            Color::White,
        );

        self.board[self.GetPos("f8")] = Square::Value(
            Piece::Bishop,
            Color::Black,
        );

        for n in 0..8 {
            self.board[self.GetPos("a2") + n] = Square::Value(
                Piece::Pawn,
                Color::White,
            );

            self.board[self.GetPos("a7") + n] = Square::Value(
                Piece::Pawn,
                Color::Black,
            );
        }
    }

    fn isMoveValid(&mut self, origin: Square, target1: Square) -> bool {
        let mut target_is_not_empty = false;
        match target1 {
            Square::Empty => target_is_not_empty = true,
            _ => (),
        }

        match origin {
            Square::Empty => return false,
            Square::Value(
                Piece::King,
                _,
            ) => (),
            Square::Value(
                Piece::Queen,
                _,
            ) => (),
            Square::Value(
                Piece::Bishop,
                _,
            ) => (),
            Square::Value(
                Piece::Knight,
                _,
            ) => (),
            Square::Value(
                Piece::Rook,
                _,
            ) => (),
            Square::Value(
                Piece::Pawn,
                Color::White,
            ) => (),
            Square::Value(
                Piece::Pawn,
                Color::Black,
            ) => (),
        }
        return true;
    }

    fn Move(&mut self, moveStr: &str) {
        if moveStr.len() != 4 {
            return; //Throw error
        }
        let (origin_str, target_str) = moveStr.split_at(2);

        let mut origin = self.board[self.GetPos(origin_str)];
        let mut target = self.board[self.GetPos(target_str)];

        if self.isMoveValid(origin, target) {
            self.board[self.GetPos(target_str)]  = self.board[self.GetPos(origin_str)];
            self.board[self.GetPos(origin_str)] = Square::Empty;
        }
    }
}

fn main() {
    let mut game: Game = Game::default();

    game.Setup();

    game.Move("e2e4");
    game.Move("e7e5");

    game.Move("e1d1");

    for (i, square) in game.board.iter().enumerate() {
        print!("{}", square);
        if (i + 1) % 8 == 0 {
            println!();
        }
    }

    println!("Hello Short ♔!");
}
