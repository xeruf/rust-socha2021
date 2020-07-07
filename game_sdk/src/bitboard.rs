use std::ops::{
    BitXor, BitXorAssign, BitAnd, BitAndAssign,
    BitOr, BitOrAssign, Not, Shl, ShlAssign, Shr, ShrAssign
};

pub const VALID_FIELDS: Bitboard = Bitboard::from(
    34359721983,
    337623909661717427026139553986326233087,
    329648537884054317714434393650000297983,
    297747050773401880467613752304696557567
);
pub const RED_START_FIELD: Bitboard = Bitboard::from(0, 0, 0, 1);
pub const BLUE_START_FIELD: Bitboard = Bitboard::from(1 << 34, 0, 0, 0);

#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Direction {
    LEFT = 0,
    UP = 1,
    RIGHT = 2,
    DOWN = 3
}

impl Direction {
    pub fn to_string(&self) -> String {
        match self {
            Direction::LEFT => "LEFT".to_string(),
            Direction::UP => "UP".to_string(),
            Direction::RIGHT => "RIGHT".to_string(),
            Direction::DOWN => "DOWN".to_string()
        }
    }
    pub fn from_u16(n: u16) -> Direction {
        match n {
            0 => Direction::LEFT,
            1 => Direction::UP,
            2 => Direction::RIGHT,
            3 => Direction::DOWN,
            _ => panic!("Invalid direction")
        }
    }
    pub fn clockwise(&self) -> Direction {
        match self {
            Direction::LEFT => Direction::UP,
            Direction::UP => Direction::RIGHT,
            Direction::RIGHT => Direction::DOWN,
            Direction::DOWN => Direction::LEFT
        }
    }
    pub fn anticlockwise(&self) -> Direction {
        match self {
            Direction::LEFT => Direction::DOWN,
            Direction::UP => Direction::LEFT,
            Direction::RIGHT => Direction::UP,
            Direction::DOWN => Direction::RIGHT
        }
    }
    pub fn mirror(&self) -> Direction {
        match self {
            Direction::LEFT => Direction::RIGHT,
            Direction::RIGHT => Direction::LEFT,
            Direction::UP => Direction::DOWN,
            Direction::DOWN => Direction::UP,
        }
    }
}

pub const DIRECTIONS: [Direction; 4] = [
    Direction::LEFT,
    Direction::UP,
    Direction::RIGHT,
    Direction::DOWN,
];

pub const PIECE_SHAPES: [u128; 11] = [
    1, // Monomino
    3, // Domino horizontal
    2097153, // Domion vertical
    7, // I-Tromino horizontal
    4398048608257, // I-Tromino vertical
    15, // I-Tetromino horizontal
    9223376434903384065, // I-Tetromino vertical
    31, // I-Pentomino horizontal
    19342822337210501698682881, // I-Pentomino vertical
    6291459, // O-Tetromino
    4398053851137, // X-Pentomino
];

#[derive(Debug, Copy, Clone)]
pub struct Bitboard {
    pub one: u128,
    pub two: u128,
    pub three: u128,
    pub four: u128
}

impl Bitboard {
    pub fn new() -> Bitboard {
        Bitboard {
            one: 0,
            two: 0,
            three: 0,
            four: 0
        }
    }

    pub const fn from(one: u128, two: u128, three: u128, four: u128) -> Bitboard {
        Bitboard {
            one: one,
            two: two,
            three: three,
            four: four
        }
    }

    pub fn with_piece(action: u16) -> Bitboard {
        let piece_shape = PIECE_SHAPES[(action >> 9) as usize];
        let to = action & 511;
        if to == 0 {
            return Bitboard::from(0, 0, 0, piece_shape);
        }
        if to == 128 {
            return Bitboard::from(0, 0, piece_shape, 0);
        }
        if to == 256 {
            return Bitboard::from(0, piece_shape, 0, 0);
        }
        if to < 128 {
            let mut board = Bitboard::from(0, 0, piece_shape, 0);
            board >>= (128 - to) as u8;
            return board;
        }
        if to < 256 {
            let mut board = Bitboard::from(0, piece_shape, 0, 0);
            board >>= (256 - to) as u8;
            return board;
        }
        if to < 384 {
            let mut board = Bitboard::from(piece_shape, 0, 0, 0);
            board >>= (384 - to) as u8;
            return board;
        }
        if to == 384 {
            return Bitboard::from(piece_shape, 0, 0, 0);
        }
        Bitboard::from(piece_shape, 0, 0, 0) << (to - 384) as u8
    }

    pub fn bit(n: u16) -> Bitboard {
        if n < 128 {
            return Bitboard::from(0, 0, 0, 1 << n);
        }
        if n < 256 {
            return Bitboard::from(0, 0, 1 << (n - 128), 0);
        }
        if n < 384 {
            return Bitboard::from(0, 1 << (n - 256), 0, 0);
        }
        return Bitboard::from(1 << (n - 384), 0, 0, 0);
    }

    pub fn count_ones(&self) -> u32 {
        return self.one.count_ones() + self.two.count_ones()
            + self.three.count_ones() + self.four.count_ones();
    }

    pub fn trailing_zeros(&self) -> u16 {
        if self.one != 0 {
            return self.one.trailing_zeros() as u16 + 384;
        }
        if self.two != 0 {
            return self.two.trailing_zeros() as u16 + 256;
        }
        if self.three != 0 {
            return self.three.trailing_zeros() as u16 + 128;
        }
        if self.four != 0 {
            return self.four.trailing_zeros() as u16;
        }
        512
    }

    pub fn not_zero(&self) -> bool {
        self.one != 0 || self.two != 0 || self.three != 0 || self.four != 0
    }

    pub fn neighbours(&self) -> Bitboard {
        ((*self << 1) | (*self >> 1) | (*self >> 21) | (*self << 21)) & VALID_FIELDS
    }

    pub fn diagonal_neighbours(&self) -> Bitboard {
        ((*self << 22) | (*self >> 22) | (*self >> 20) | (*self << 20)) & VALID_FIELDS
    }

    pub fn neighbours_in_direction(&self, d: Direction) -> Bitboard {
        match d {
            Direction::LEFT => *self << 1,
            Direction::RIGHT => *self >> 1,
            Direction::UP => *self << 21,
            Direction::DOWN => *self >> 21,
        }
    }
}

impl BitXor for Bitboard {
    type Output = Self;

    fn bitxor(self, other: Self) -> Self::Output {
        Bitboard {
            one: self.one ^ other.one,
            two: self.two ^ other.two,
            three: self.three ^ other.three,
            four: self.four ^ other.four
        }
    }
}

impl BitXorAssign for Bitboard {
    fn bitxor_assign(&mut self, other: Self) {
        self.one ^= other.one;
        self.two ^= other.two;
        self.three ^= other.three;
        self.four ^= other.four;
    }
}

impl BitAnd for Bitboard {
    type Output = Self;

    fn bitand(self, other: Self) -> Self::Output {
        Bitboard {
            one: self.one & other.one,
            two: self.two & other.two,
            three: self.three & other.three,
            four: self.four & other.four
        }
    }
}

impl BitAndAssign for Bitboard {
    fn bitand_assign(&mut self, other: Self) {
        self.one &= other.one;
        self.two &= other.two;
        self.three &= other.three;
        self.four &= other.four;
    }
}

impl BitOr for Bitboard {
    type Output = Self;

    fn bitor(self, other: Self) -> Self::Output {
        Bitboard {
            one: self.one | other.one,
            two: self.two | other.two,
            three: self.three | other.three,
            four: self.four | other.four
        }
    }
}

impl BitOrAssign for Bitboard {
    fn bitor_assign(&mut self, other: Self) {
        self.one |= other.one;
        self.two |= other.two;
        self.three |= other.three;
        self.four |= other.four;
    }
}

impl Not for Bitboard {
    type Output = Self;

    fn not(self) -> Self::Output {
        Bitboard {
            one: !self.one,
            two: !self.two,
            three: !self.three,
            four: !self.four
        }
    }
}

impl Shl<u8> for Bitboard {
    type Output = Self;

    fn shl(self, n: u8) -> Self::Output {
        Bitboard {
            one: (self.one << n) | (self.two >> (128 - n)),
            two: (self.two << n) | (self.three >> (128 - n)),
            three: (self.three << n) | (self.four >> (128 - n)),
            four: self.four << n
        }
    }
}

impl ShlAssign<u8> for Bitboard {
    fn shl_assign(&mut self, n: u8) {
        self.one = (self.one << n) | (self.two >> (128 - n));
        self.two = (self.two << n) | (self.three >> (128 - n));
        self.three = (self.three << n) | (self.four >> (128 - n));
        self.four = self.four << n;
    }
}

impl Shr<u8> for Bitboard {
    type Output = Self;

    fn shr(self, n: u8) -> Self::Output {
        Bitboard {
            one: self.one >> n,
            two: (self.two >> n) | (self.one << (128 - n)),
            three: (self.three >> n) | (self.two << (128 - n)),
            four: (self.four >> n) | self.three << (128 - n)
        }
    }
}

impl ShrAssign<u8> for Bitboard {
    fn shr_assign(&mut self, n: u8) {
        self.four = (self.four >> n) | self.three << (128 - n);
        self.three = (self.three >> n) | (self.two << (128 - n));
        self.two = (self.two >> n) | (self.one << (128 - n));
        self.one >>= n;
    }
}

impl PartialEq for Bitboard {
    fn eq(&self, other: &Self) -> bool {
        return self.one == other.one && self.two == other.two
            && self.three == other.three && self.four == other.four
    }
}
