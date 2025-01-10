#![feature(portable_simd)]
use std::{array::IntoIter, fmt::Display, simd::u64x16};

use boards::{BLACK_PAWN_MOVES, BLACK_ROOK_MOVES, WHITE_KNIGHT_MOVES, WHITE_PAWN_MOVES, WHITE_ROOK_MOVES};

pub mod boards;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + 'static>>;

#[derive(Debug, Clone, Copy)]
pub enum Piece {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King
}

#[derive(Debug, Clone)]
pub enum PlayerColor {
    Black,
    White,
}

#[derive(Debug)]
pub struct Player {
    pub pawns: u64,
    pub rooks: u64,
    pub knights: u64,
    pub bishops: u64,
    pub queen: u64,
    pub king: u64,
    color: PlayerColor,
}

impl Player {
    /// create boards with the starting positions of each player's pieces
    pub fn new(color: PlayerColor) -> Self {
        let (pawns, rooks, knights, bishops, queen, king) = match color {
            PlayerColor::Black => (
                0xff << 48,
                0x81 << 56,
                0x42 << 56,
                0x24 << 56,
                0x8 << 56,
                0x10 << 56,
            ),
            PlayerColor::White => (0xff << 8, 0x81, 0x42, 0x24, 0x8, 0x10),
        };
        Self {
            pawns,
            rooks,
            knights,
            bishops,
            queen,
            king,
            color,
        }
    }
    pub fn boards(&self) -> IntoIter<u64, 6> {
        [
            self.pawns,
            self.rooks,
            self.knights,
            self.bishops,
            self.queen,
            self.king,
        ]
        .into_iter()
    }
}
#[derive(Debug)]
pub struct Game {
    pub white: Player,
    pub black: Player,
    pub turn: PlayerColor,
}
impl Default for Game {
    fn default() -> Self {
        Self {
            white: Player::new(PlayerColor::White),
            black: Player::new(PlayerColor::Black),
            turn: PlayerColor::White,
        }
    }
}
impl Game {
    pub fn boards(&self) -> Vec<u64> {
        self.white
            .boards()
            .chain(self.black.boards())
            .collect::<Vec<u64>>()
    }

    pub fn advance(&self, start: u64, end: u64) -> Option<Piece> {
        None
    }

    pub fn attack(&self, boards: &[u64]) -> Option<Piece> {
        None
    }

    // TODO: check active piece board of each player?
    // TODO: nuke this
    pub fn query(&self, position: u64) -> Option<Piece> {
        let mut slice = [0u64; 16];
        slice[..self.boards().len()].copy_from_slice(self.boards().as_slice());
        let boards = u64x16::from_slice(&slice);
        let mask = u64x16::splat(position);
        let shift = boards >> mask;
        // let and = shift & 1;
        for (index, res) in shift.as_array().iter().enumerate() {
            let x = res & 1;
            if x == 1 {
                let piece = match index {
                    0|6 => Some(Piece::Pawn),
                    1|7 => Some(Piece::Rook),
                    2|8 => Some(Piece::Knight),
                    3|9 => Some(Piece::Bishop),
                    4|10 => Some(Piece::Queen),
                    5|11 => Some(Piece::King),
                    _ => None
                };
                match (piece, self.turn.clone()) {
                    (Some(Piece::Pawn), PlayerColor::Black) => {
                        println!("black pawns");
                        print_moves(self.black.pawns);
                        println!("moves from {position}");
                        let moves = BLACK_PAWN_MOVES[position as usize];
                        print_moves(moves);
                    },
                    (Some(Piece::Pawn), PlayerColor::White) => {
                        println!("white pawns");
                        print_moves(self.white.pawns);
                        println!("moves from {position}");
                        let moves = WHITE_PAWN_MOVES[position as usize];
                        print_moves(moves);
                    },
                    (Some(Piece::Rook), PlayerColor::Black) => {
                        println!("black rook");
                        print_moves(self.black.rooks);
                        println!("moves from {position}");
                        let moves = BLACK_ROOK_MOVES[position as usize];
                        print_moves(moves);
                    },
                    (Some(Piece::Rook), PlayerColor::White) => {
                        println!("white rook");
                        print_moves(self.white.rooks);
                        println!("moves from {position}");
                        let moves = WHITE_ROOK_MOVES[position as usize];
                        print_moves(moves);
                    },
                    (Some(Piece::Knight), PlayerColor::White) => {
                        let moves = WHITE_KNIGHT_MOVES[position as usize];
                        print_moves(moves);
                    },
                    _ => ()

                }
            }
        }
        
        return None;
    }
}

pub fn print_moves(moves: u64) {
    let binary_string = format!("{:064b}", moves);
    for chunk in binary_string.as_bytes().chunks(8) {
        println!("{}", std::str::from_utf8(chunk).unwrap());
    }
}

impl Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for index in 0..64 {
            let j = index % 8;
            if let Some(piece) = self.query(index as u64) {
                let piece = match piece {
                    Piece::Pawn => "♙",
                    Piece::Rook => "♖",
                    Piece::Knight => "♘",
                    Piece::Bishop => "♗",
                    Piece::Queen => "♕",
                    Piece::King => "♔"
                };
                write!(f, " {piece} ")?;
            } else {
                write!(f, " x ")?;
            }
            if j == 7 {
                write!(f, "\n")?;
            }
        }
        Ok(())
    }
}

