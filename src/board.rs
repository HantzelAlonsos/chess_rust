

pub enum Color{
    White,
    Black,
    None
}


pub enum PieceType{
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
    None
}

pub struct Piece{
    color: Color,
    pieceType: PieceType
}

pub struct Square{
    piecePtr: Piece
}
