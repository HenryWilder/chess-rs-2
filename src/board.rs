use raylib::prelude::*;

pub enum CellColor {
    Black,
    White,
}

pub struct Cell {
    pub col: i32,
    pub row: i32,
}

impl Cell {
    pub fn color(&self) -> CellColor {
        if (self.col & 1) ^ (self.row & 1) == 1 {
            CellColor::Black
        } else {
            CellColor::White
        }
    }
}

pub struct Board {
    pub num_cols: u16,
    pub num_rows: u16,
    pub cell_width: u16,
    pub cell_height: u16,
}

impl Board {
    /// Gives the position of the top-left pixel in the cell, relative to the board.
    pub fn local_from_map(&self, cell: &Cell) -> Vector2 {
        Vector2 {
            x: cell.col.wrapping_mul(i32::try_from(self.cell_width).unwrap()) as f32,
            y: cell.row.wrapping_mul(i32::try_from(self.cell_height).unwrap()) as f32,
        }
    }

    /// Gives the cell containing the local position.
    pub fn local_to_map(&self, local_pos: &Vector2) -> Cell {
        Cell {
            col: local_pos.x as i32 / i32::try_from(self.cell_width).unwrap(),
            row: local_pos.y as i32 / i32::try_from(self.cell_height).unwrap(),
        }
    }

    pub fn cell_size(&self) -> Vector2 {
        Vector2 {
            x: self.cell_width as f32,
            y: self.cell_height as f32,
        }
    }
}
