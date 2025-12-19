mod board;
mod chess_move;
mod piece;
mod tile;

use static_sdd::*;

#[grammar]
mod chess {
    use super::*;

    #[context]
    use crate::board::Board;

    #[non_terminal]
    #[start_symbol]
    pub enum Game {
        WhiteWon,
        BlackWon,
        StaleMate,
    }

    #[non_terminal]
    pub type SetupString = ();

    #[non_terminal]
    pub type Moves = ();

    #[non_terminal]
    use crate::chess_move::SimpleMove;

    #[non_terminal]
    use crate::chess_move::Castling;

    #[non_terminal]
    pub use chess_move::Move;

    #[non_terminal]
    pub use piece::PieceType;

    #[non_terminal]
    pub use tile::Tile;

    #[token = "K"]
    pub struct KingPiece;

    #[token = "Q"]
    pub struct QueenPiece;

    #[token = "R"]
    pub struct RookPiece;

    #[token = "B"]
    pub struct BishopPiece;

    #[token = "N"]
    pub struct KnightPiece;

    #[token = "x"]
    pub struct Takes;

    #[token = "0-0|O-O"]
    pub struct KingSideCastling;

    #[token = "0-0-0|O-O-O"]
    pub struct QueenSideCastling;

    #[token = r"[a-h]"]
    pub type File = char;

    #[token = r"[1-8]"]
    pub type Rank = usize;

    // GAME
    production!(G, Game -> (SetupString, Moves), |board, _| {
        todo!()
    });

    // SETUP STRING
    production!(S0, SetupString -> (), |board, _| *board = Board::starting_board());

    // MOVES
    production!(M0, Game -> Moves, |_| todo!());
    production!(M1, Moves -> (Moves, Move), |board, (_, mv)| {
        board.do_move(mv);
        let response = board.best_move();
        println!("{response}");
        board.do_move(response);
    });
    production!(M2, Moves -> ());
    production!(M3, Move -> SimpleMove, |sm| Move::SimpleMove(sm));
    production!(M4, Move -> Castling, |c| Move::Castling(c));

    // MOVE
    production!(M5, SimpleMove -> (PieceType, Tile), |(ty, pos)| SimpleMove::piece_moved(ty, pos));
    production!(M6, SimpleMove -> Tile, |pos| SimpleMove::piece_moved(PieceType::Pawn, pos));
    production!(M7, SimpleMove -> (PieceType, Takes, Tile), |(ty, _, pos)| SimpleMove::takes(ty, pos));
    production!(M8, SimpleMove -> (PieceType, File, Tile), |(ty, sf, pos)| todo!());

    // PIECE TYPES
    production!(P0, PieceType -> KingPiece, |_| PieceType::King);
    production!(P1, PieceType -> QueenPiece, |_| PieceType::Queen);
    production!(P2, PieceType -> RookPiece, |_| PieceType::Rook);
    production!(P3, PieceType -> BishopPiece, |_| PieceType::Bishop);
    production!(P4, PieceType -> KnightPiece, |_| PieceType::Knight);
}

fn main() {}
