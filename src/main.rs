use std::{fmt, string};

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
    NotEmpty {
        piece: Piece,
        color: Color,
    },
    #[default]
    Empty,
}

impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Square::Empty => write!(f, "   "),
            Square::NotEmpty {
                piece: Piece::King,
                color: Color::White,
            } => write!(f, " ♔ "),
            Square::NotEmpty {
                piece: Piece::King,
                color: Color::Black,
            } => write!(f, " ♚ "),

            Square::NotEmpty {
                piece: Piece::Queen,
                color: Color::White,
            } => write!(f, " ♕ "),
            Square::NotEmpty {
                piece: Piece::Queen,
                color: Color::Black,
            } => write!(f, " ♛ "),

            Square::NotEmpty {
                piece: Piece::Bishop,
                color: Color::White,
            } => write!(f, " ♗ "),
            Square::NotEmpty {
                piece: Piece::Bishop,
                color: Color::Black,
            } => write!(f, " ♝ "),

            Square::NotEmpty {
                piece: Piece::Knight,
                color: Color::White,
            } => write!(f, " ♘ "),
            Square::NotEmpty {
                piece: Piece::Knight,
                color: Color::Black,
            } => write!(f, " ♞ "),

            Square::NotEmpty {
                piece: Piece::Rook,
                color: Color::White,
            } => write!(f, " ♖ "),
            Square::NotEmpty {
                piece: Piece::Rook,
                color: Color::Black,
            } => write!(f, " ♜ "),

            Square::NotEmpty {
                piece: Piece::Pawn,
                color: Color::White,
            } => write!(f, " ♙ "),
            Square::NotEmpty {
                piece: Piece::Pawn,
                color: Color::Black,
            } => write!(f, " ♟︎ "),
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
        self.board[self.GetPos("e1")] = Square::NotEmpty {
            piece: Piece::King,
            color: Color::White,
        };

        self.board[self.GetPos("e8")] = Square::NotEmpty {
            piece: Piece::King,
            color: Color::Black,
        };

        // Queen
        self.board[self.GetPos("d1")] = Square::NotEmpty {
            piece: Piece::Queen,
            color: Color::White,
        };

        self.board[self.GetPos("d8")] = Square::NotEmpty {
            piece: Piece::Queen,
            color: Color::Black,
        };

        // Rooks
        self.board[self.GetPos("a1")] = Square::NotEmpty {
            piece: Piece::Rook,
            color: Color::White,
        };

        self.board[self.GetPos("a8")] = Square::NotEmpty {
            piece: Piece::Rook,
            color: Color::Black,
        };

        self.board[self.GetPos("h1")] = Square::NotEmpty {
            piece: Piece::Rook,
            color: Color::White,
        };

        self.board[self.GetPos("h8")] = Square::NotEmpty {
            piece: Piece::Rook,
            color: Color::Black,
        };

        // Knights
        self.board[self.GetPos("b1")] = Square::NotEmpty {
            piece: Piece::Knight,
            color: Color::White,
        };

        self.board[self.GetPos("b8")] = Square::NotEmpty {
            piece: Piece::Knight,
            color: Color::Black,
        };

        self.board[self.GetPos("g1")] = Square::NotEmpty {
            piece: Piece::Knight,
            color: Color::White,
        };

        self.board[self.GetPos("g8")] = Square::NotEmpty {
            piece: Piece::Knight,
            color: Color::Black,
        };

        // Bishops
        self.board[self.GetPos("c1")] = Square::NotEmpty {
            piece: Piece::Bishop,
            color: Color::White,
        };

        self.board[self.GetPos("c8")] = Square::NotEmpty {
            piece: Piece::Bishop,
            color: Color::Black,
        };

        self.board[self.GetPos("f1")] = Square::NotEmpty {
            piece: Piece::Bishop,
            color: Color::White,
        };

        self.board[self.GetPos("f8")] = Square::NotEmpty {
            piece: Piece::Bishop,
            color: Color::Black,
        };

        for n in 0..8 {
            self.board[self.GetPos("a2") + n] = Square::NotEmpty {
                piece: Piece::Pawn,
                color: Color::White,
            };

            self.board[self.GetPos("a7") + n] = Square::NotEmpty {
                piece: Piece::Pawn,
                color: Color::Black,
            };
        }
    }

    fn isMoveValid(&mut self, origin1: Square, target1: Square) -> bool {

        match origin1 {
            Square::Empty => return false,
            _ => (),
        }
        
        let mut target_is_not_empty = false;
        match target1 {
            Square::Empty => target_is_not_empty = true,
            _ => (),
        }

        
        return true;
    }

    fn Move(&mut self, moveStr: &str) {
        if moveStr.len() != 4 {
            return; //Throw error
        }
        let (originStr, targetStr) = moveStr.split_at(2);

        let mut origin = self.board[self.GetPos(originStr)];
        let mut target = self.board[self.GetPos(targetStr)];
        
        if self.isMoveValid(origin, target) {
            println!("{}", originStr);
            println!("{}", targetStr);
            self.board[self.GetPos(targetStr)] = self.board[self.GetPos(originStr)];
            self.board[self.GetPos(originStr)] = Square::Empty;
        }
    }
}

fn main() {
    let mut game: Game = Game::default();

    game.Setup();

    game.Move("e2e4");
    game.Move("e7e5");

    for (i, square) in game.board.iter().enumerate() {
        print!("{}", square);
        if ((i + 1) % 8 == 0) {
            println!();
        }
    }

    println!("Hello Short ♔!");
}
