use raylib::prelude::*;

mod board;
use board::*;
mod theme;
use theme::*;

fn main() {
    let window_width = 1280;
    let window_height = 720;
    let (mut rl, thread) = raylib::init()
        .size(window_width, window_height)
        .title("Chess")
        .build();
    rl.set_target_fps(60);

    let board = Board{
        num_cols: 8,
        num_rows: 8,
        cell_width: 32,
        cell_height: 32,
    };

    let theme = Theme{
        tile_black:  Color::new(236, 167,  95, 255),
        tile_white:  Color::new(252, 219, 166, 255),
        piece_black: PieceTheme{
            fill:    Color::new( 80,  80,  90, 255),
            shade:   Color::new( 50,  55,  60, 255),
            outline: Color::new( 40,  30,  50, 255),
            shine:   Color::new(100, 100, 110, 255),
        },
        piece_white: PieceTheme{
            fill:    Color::new(240, 240, 230, 255),
            shade:   Color::new(180, 150, 120, 255),
            outline: Color::new(100, 100,  90, 255),
            shine:   Color::new(255, 255, 255, 255),
        },
        hover:       Color::new(127, 127, 255, 255),
        select:      Color::new( 44, 200,  37, 255),
        available:   Color::new(155, 235, 135, 255),
        capturing:   Color::new(255,  80,  80, 255),
        illegal:     Color::new(255, 200,  80, 255),
        blocking:    Color::new(127, 127, 127, 255),
    };

    while !rl.window_should_close() {
        // -------------------------------------------- Tick -------------------------------------------- //
        let mouse_pos = rl.get_mouse_position();
        let hovered_cell = board.local_to_map(&mouse_pos);

        // -------------------------------------------- Draw -------------------------------------------- //
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color{ r: 0, g: 0, b: 0, a: 0 });

        let cell_size = board.cell_size();

        d.draw_rectangle_v(
            board.local_from_map(&Cell{ col: 0, row: 0 }),
            board.local_from_map(&Cell{ col: board.num_cols as i32, row: board.num_rows as i32 }),
            theme.tile_black);

        for row in 0..board.num_rows as i32 {
            for col in ((row & 1)..board.num_cols as i32).step_by(2) {
                let c = Cell{ col, row };
                d.draw_rectangle_v(board.local_from_map(&c), cell_size, theme.tile_white);
            }
        }

        d.draw_rectangle_v(board.local_from_map(&hovered_cell), cell_size, theme.hover);
    }
}
