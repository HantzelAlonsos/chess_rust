use core::panic;
use std::{collections::btree_map::Range, fmt};


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
    en_passant_square : i32,
    en_passant_turn : i32,
    current_turn : i32,
}

impl Default for Game {
    fn default() -> Self {
        Game {
            board: [Square::Empty; 64],
            en_passant_square : -1,
            en_passant_turn : -1,
            current_turn : 0,
        }
    }
}

impl Game {
    fn Get_pos(&self, a: &str) -> usize {
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
        if row+col > 63 {
            println!("Alrighjt so the value of the array larger than the size of the board so that isnt okay");
            return 0;

        }
        return usize::from(row + col);
    }

    fn Setup(&mut self) {
        // King
        self.board[self.Get_pos("e1")] = Square::Value(
            Piece::King,
            Color::White,
        );

        self.board[self.Get_pos("e8")] = Square::Value(
            Piece::King,
            Color::Black,
        );

        // Queen
        self.board[self.Get_pos("d1")] = Square::Value(
            Piece::Queen,
            Color::White,
        );

        self.board[self.Get_pos("d8")] = Square::Value(
            Piece::Queen,
            Color::Black,
        );

        // Rooks
        self.board[self.Get_pos("a1")] = Square::Value(
            Piece::Rook,
            Color::White,
        );

        self.board[self.Get_pos("a8")] = Square::Value(
            Piece::Rook,
            Color::Black,
        );

        self.board[self.Get_pos("h1")] = Square::Value(
            Piece::Rook,
            Color::White,
        );

        self.board[self.Get_pos("h8")] = Square::Value(
            Piece::Rook,
            Color::Black,
        );

        // Knights
        self.board[self.Get_pos("b1")] = Square::Value(
            Piece::Knight,
            Color::White,
        );

        self.board[self.Get_pos("b8")] = Square::Value(
            Piece::Knight,
            Color::Black,
        );

        self.board[self.Get_pos("g1")] = Square::Value(
            Piece::Knight,
            Color::White,
        );

        self.board[self.Get_pos("g8")] = Square::Value(
            Piece::Knight,
            Color::Black,
        );

        // Bishops
        self.board[self.Get_pos("c1")] = Square::Value(
            Piece::Bishop,
            Color::White,
        );

        self.board[self.Get_pos("c8")] = Square::Value(
            Piece::Bishop,
            Color::Black,
        );

        self.board[self.Get_pos("f1")] = Square::Value(
            Piece::Bishop,
            Color::White,
        );

        self.board[self.Get_pos("f8")] = Square::Value(
            Piece::Bishop,
            Color::Black,
        );

        for n in 0..8 {
            self.board[self.Get_pos("a2") + n] = Square::Value(
                Piece::Pawn,
                Color::White,
            );

            self.board[self.Get_pos("a7") + n] = Square::Value(
                Piece::Pawn,
                Color::Black,
            );
        }
    }

    fn isMoveValid(&mut self, origin: Square, target: Square, origin_str : &str, target_str: &str) -> bool {
        let mut is_black = false;
        let mut target_is_black = false;
        let mut target_is_emtpy = false;
        self.current_turn = self.current_turn + 1;
        match origin {
            Square::Value(_, Color::Black)=> is_black = true,
            Square::Value(_, Color::White)=> is_black = false,
            Square::Empty => return false
        }

        match target{
            Square::Empty => target_is_emtpy = true,
            Square::Value(_, Color::Black)=> target_is_black = true,
            Square::Value(_, Color::White)=> target_is_black = false,
        }
        if !target_is_emtpy{
            if (is_black == target_is_black){
                // Fuck
                return false;
            }    
        }

        match origin {
            Square::Empty => return false,
            Square::Value(
                Piece::King,
                _,
            ) => return  self.is_lateral_move_valid(origin, target, origin_str, target_str, 1) ||  self.is_diagonal_move_valid(origin, target, origin_str, target_str, 1),
            Square::Value(
                Piece::Queen,
                _,
            ) => return self.is_lateral_move_valid(origin, target, origin_str, target_str, 8) ||  self.is_diagonal_move_valid(origin, target, origin_str, target_str, 8),
            Square::Value(
                Piece::Bishop,
                _,
            ) => return self.is_diagonal_move_valid(origin, target, origin_str, target_str, 8),
            Square::Value(
                Piece::Knight,
                _,
            ) => return self.is_knight_move_valid(origin, target, origin_str, target_str),
            Square::Value(
                Piece::Rook,
                _,
            ) => return self.is_lateral_move_valid(origin, target, origin_str, target_str, 8),
            Square::Value(
                Piece::Pawn,
                Color::White,
            ) => return self.is_pawn_move_valid(origin, target, origin_str, target_str),
            Square::Value(
                Piece::Pawn,
                Color::Black,
            ) => return self.is_pawn_move_valid(origin, target, origin_str, target_str), // Pawns need to be dealt with seperately, specifically the enpassant
        }
        return false;
    }

    fn is_pawn_move_valid(&mut self, origin: Square, target: Square, origin_str : &str, target_str: &str) -> bool{
    
        match origin{
            Square::Value(Piece::Pawn, _)=>(),
            _ => return false, // Thius will never happen but you never know
        }
        
        let origin_val = self.Get_pos(origin_str);
        let target_val = self.Get_pos(target_str); 

        let delta = origin_val as i32 - target_val as i32;
        
        if delta.abs() == 8 {
            return self.is_lateral_move_valid(origin, target, origin_str, target_str, 1)
        }

        match (delta.abs(), origin, target){
            (7, Square::Value(_, Color::Black), Square::Value(_, Color::White)) => return true,
            (9, Square::Value(_, Color::Black), Square::Value(_, Color::White)) => return true,
            (7, Square::Value(_, Color::White), Square::Value(_, Color::Black)) => return true,
            (9, Square::Value(_, Color::White), Square::Value(_, Color::Black)) => return true,
            _ => (),
        }

        let target_is_en_passant = if target_val as i32 == self.en_passant_square  {true} else {false};
        let en_passant_was_previous_turn = if  self.current_turn - self.en_passant_turn  == 1  {true} else {false};
        
        match (target_is_en_passant, en_passant_was_previous_turn,  origin, target){
            (true, true ,Square::Value(_, Color::Black), Square::Empty) => return true,
            (true, true ,Square::Value(_, Color::White), Square::Empty) => return true,
            _ => (),
        }
        
        println!("{}", target_val);
        println!("{}", self.current_turn);
        

        println!("{}", self.en_passant_square);
        println!("{}", self.en_passant_turn);
        
        println!("{}", en_passant_was_previous_turn);
        println!("{}", target_is_en_passant);

        if delta.abs() == 16{
            if self.is_lateral_move_valid(origin, target, origin_str, target_str, 2){
                self.en_passant_square = origin_val as i32 - delta/2;
                self.en_passant_turn = self.current_turn;
                return true;
            }
        }


        return false;
    }

    fn is_knight_move_valid(&mut self, origin: Square, target1: Square, origin_str : &str, target_str: &str) -> bool{
        
        let origin_val = self.Get_pos(origin_str);
        let target_val = self.Get_pos(target_str);

        let delta = origin_val as i32 - target_val as i32;

        match delta {
            15=>(),
            17=>(),
            10=>(),
            -6=>(), 
            -15=>(),
            -17=>(),
            -10=>(),
            6=>(),
            _=> return false,
        }  
        return true;
    }

       
    fn is_lateral_move_valid(&mut self, origin: Square, target1: Square, origin_str : &str, target_str: &str, max_distance : i32) -> bool{
        let origin_val = self.Get_pos(origin_str);
        let target_val = self.Get_pos(target_str);

        let move_row = (origin_val as i32 - target_val as i32)%8;
        let distance = ((origin_val as i32 - target_val as i32));
        
        let mut delta;
        if move_row == 0 {
           delta = if ((origin_val as i32 - target_val as i32) > 0) {-8} else {8};
        }
        else if distance.abs() < 8  {
            delta = if ((origin_val as i32 - target_val as i32) > 0) {-1} else {1};
        }
        else{
            return false;
        }

        for jump in 1..=max_distance {
            let square = origin_val as i32 + jump * delta;
            if square == target_val as i32{
                return true;
            }
            
            match self.board[square as usize]{
                Square::Empty => (),
                _ => return false,
            }
        }

        return false;
    }

    fn is_diagonal_move_valid(&mut self, origin: Square, target1: Square, origin_str : &str, target_str: &str, max_distance : i32) -> bool{
        let origin_val = self.Get_pos(origin_str);
        let target_val = self.Get_pos(target_str);

        let move_left_delta = (origin_val as i32 - target_val as i32)%7;
        let move_right_delta = (origin_val as i32 - target_val as i32)%9;

        let mut delta = 0;
        if move_left_delta == 0{
            delta = if ((origin_val as i32 - target_val as i32) > 0) {-7} else {7};
        }
        else if move_right_delta == 0 {
            delta = if ((origin_val as i32 - target_val as i32) > 0) {-9} else {9};
        }
        else{
            return false;
        }
        
        for jump in 1..max_distance {
            let square = origin_val as i32 + jump * delta;
            if square == target_val as i32{
                return true;
            }
            
            match self.board[square as usize]{
                Square::Empty => (),
                _ => return false,
            }
        }
        return false;
    }

    fn Move(&mut self, moveStr: &str) {
        if moveStr.len() != 4 {
            return; //Throw error
        }
        let (origin_str, target_str) = moveStr.split_at(2);

        let mut origin = self.board[self.Get_pos(origin_str)];
        let mut target = self.board[self.Get_pos(target_str)];

        if self.isMoveValid(origin, target, origin_str, target_str) {
            println!("{} is valid", moveStr);
            self.board[self.Get_pos(target_str)]  = self.board[self.Get_pos(origin_str)];
            self.board[self.Get_pos(origin_str)] = Square::Empty;
        }
        else{
            println!("{} is not valid", moveStr);
        }
    }

    fn Render(&self) {

        println!("    a  b  c  d  e  f  g  h    ");
        print!(" 1 ");
        for (i, square) in self.board.iter().enumerate() {
            if (i) % 8 == 0 {
                if i != 0{
                    println!(" {} ", (i/8));
                    print!(" {} ", (i/8)+1);
                }
                
            }
            print!("{}", square);
            
        }
        println!("");
        println!("    a  b  c  d  e  f  g  h    ");

    }
}

fn main() {
    let mut game: Game = Game::default();

    game.Setup();

    
    game.Move("e2e3");
    game.Render();

    game.Move("d2d4");
    game.Render();

    game.Move("d7d6");
    game.Render();

    
    game.Move("a2a4");
    game.Render();

    game.Move("a4a5");
    game.Render();

    game.Move("b7b5");
    game.Render();

    game.Move("a5b6");
    game.Render();

    game.Move("h7h5");
    game.Move("h5h4");
    game.Move("g2g4");
    game.Move("h4g3");
    game.Render();

    println!("Hello Short ♔!");
}
