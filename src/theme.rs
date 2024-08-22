use raylib::prelude::*;

pub struct PieceTheme {
    pub fill:    Color,
    pub shade:   Color,
    pub outline: Color,
    pub shine:   Color,
}

pub struct Theme {
    /// Color of black tiles on the board
    pub tile_black: Color,
    /// Color of white tiles on the board
    pub tile_white: Color,

    /// Colors used by chess pieces on the black team
    pub piece_black: PieceTheme,
    /// Colors used by chess pieces on the white team
    pub piece_white: PieceTheme,

    pub hover: Color,

    /// Unit player is moving
    pub select: Color,

    /// Available space player can move to
    pub available: Color,

    /// Available space with a piece the player can take
    pub capturing: Color,

    /// King would be put in check
    pub illegal: Color,

    /// A teammate is already at that space
    pub blocking: Color,
}
