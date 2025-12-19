use std::fmt::Display;

use crate::{piece::PieceType, tile::Tile};

pub struct SimpleMove {
    pub piece_type: PieceType,
    pub target_tile: Tile,
    pub source_file: Option<char>,
    pub source_rank: Option<usize>,
    pub takes: bool,
    pub check: bool,
    pub checkmate: bool,
    pub promotion_piece_type: Option<PieceType>,
}

impl SimpleMove {
    pub fn piece_moved(piece_type: PieceType, target_tile: Tile) -> Self {
        Self {
            piece_type,
            target_tile,
            source_file: None,
            source_rank: None,
            takes: false,
            check: false,
            checkmate: false,
            promotion_piece_type: None,
        }
    }

    pub fn takes(piece_type: PieceType, target_tile: Tile) -> Self {
        Self {
            piece_type,
            target_tile,
            source_file: None,
            source_rank: None,
            takes: true,
            check: false,
            checkmate: false,
            promotion_piece_type: None,
        }
    }
}

impl Display for SimpleMove {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.piece_type)?;

        if let Some(ref file) = self.source_file {
            write!(f, "{file}")?;
        }
        if let Some(ref rank) = self.source_rank {
            write!(f, "{rank}")?;
        }
        if self.takes {
            write!(f, "x")?;
        }
        if let Some(ref prom) = self.promotion_piece_type {
            write!(f, "{prom}")?;
        }
        if self.check {
            write!(f, "+")?;
        } else if self.checkmate {
            write!(f, "#")?;
        }
        Ok(())
    }
}

pub enum Castling {
    KingSide,
    QueenSide,
}

impl Display for Castling {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::KingSide => write!(f, "0-0"),
            Self::QueenSide => write!(f, "0-0-0"),
        }
    }
}

pub enum Move {
    SimpleMove(SimpleMove),
    Castling(Castling),
}

impl Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SimpleMove(mv) => write!(f, "{mv}"),
            Self::Castling(ca) => write!(f, "{ca}"),
        }
    }
}
