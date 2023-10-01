/// Empty = 0 \
/// Pawn = 1 \
/// Rook = 2 \
/// Knight = 3 \
/// Bishop = 4 \
/// Queen = 5 \
/// King = 6 \
/// Black variants += 6
#[derive(Copy, Clone, Debug)]
pub struct Piece(pub i8);

pub struct Board {
    pub(super) pieces: [Piece; 8 * 8],
}

impl Board {
    pub fn new_empty() -> Self {
        Self {
            pieces: [Piece(0); 8 * 8],
        }
    }

    pub fn new_normal() -> Self {
        const P: Piece = Piece(0);
        let mut pieces = [P; 8 * 8];
        let mut pos = 0;
        // Generate white pieces
        for p in &mut pieces[0..16] {
            let piece_val = match pos {
                0 | 7 => 2,
                1 | 6 => 3,
                2 | 5 => 4,
                3 => 5,
                4 => 6,
                _pawn => {
                    p.0 = 1;
                    continue;
                }
            };
            p.0 = piece_val;
            pos += 1;
        }
        pieces.copy_within(0..8, 56);
        pieces.copy_within(8..16, 48);
        for p in &mut pieces[48..64] {
            p.0 += 6;
        }
        Self { pieces }
    }

    pub fn pretty_print(&self) {
        for p in self.pieces.iter().enumerate() {
            let padding = if p.1 .0 < 10 { " " } else { "" };
            if (p.0 + 1) % 8 == 0 {
                println!("{}{}", padding, p.1 .0);
            } else {
                print!("{}{}, ", padding, p.1 .0);
            }
        }
    }

    pub fn get_pieces(&self) -> &[Piece; 64] {
        return &self.pieces;
    }
}

pub struct Move {
    pub(super) piece: Piece,
    pub(super) capture: Option<Piece>,
    pub(super) from: usize,
    pub(super) to: usize,
}

pub struct Game {
    pub(super) board: Board,
    pub(super) moves: Vec<Move>,
    pub(super) castling: ((bool, bool), (bool, bool)),
}


impl Game {
    pub fn new(board: Board) -> Self {
        Self {
            board,
            moves: Vec::new(),
            castling: ((true, true), (true, true))
        }
    }
}

mod tests {
    use super::Board;
    #[test]
    pub fn tst1() {
        let b = Board::new_normal();
        b.pretty_print();
    }
}
