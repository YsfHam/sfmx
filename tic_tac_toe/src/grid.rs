use sfmx::prelude::*;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Symbol {
    Empty = 0,
    X,
    O,

    Count
}

#[derive(Clone, Debug, PartialEq, Copy)]
pub enum GameStatus {
    Winner(Symbol),
    Draw,
    NotFinished
}

#[derive(Clone)]
struct Grid {
    symbols: Vec<Symbol>,
    grid_size: usize,
    sym_occs_win: u32,
    last_move: (usize, usize)
}

impl Grid {
    fn new(grid_size: usize, sym_occs_win: u32) -> Self {
        let mut symbols = Vec::with_capacity(grid_size * grid_size);
        symbols.resize(grid_size * grid_size, Symbol::Empty);

        Self {
            symbols,
            grid_size,
            sym_occs_win,
            last_move: (0, 0)
        }
    }

    fn get_symbol(&self, x: usize, y: usize) -> Symbol {
        if x >= self.grid_size || y >= self.grid_size {
            return Symbol::Empty;
        }
        self.symbols[x * self.grid_size + y]
    }

    fn set_symbol(&mut self, x: usize, y: usize, symbol: Symbol) {
        self.symbols[x * self.grid_size + y] = symbol;
        self.last_move = (x, y);
    }

    fn is_empty(&self, x: usize, y: usize) -> bool {
        self.get_symbol(x, y) == Symbol::Empty
    }

    fn check_line(&self) -> Option<Vec<(usize, usize)>> {
        let (x, y) = self.last_move;
        
        let mut winning_indices = Vec::with_capacity(self.grid_size);

        let start = x as i32 - self.sym_occs_win as i32 + 1;
        for i in 0..self.grid_size {
            let start = start + i as i32;
            let end = x as i32 + i as i32 + 1;
            winning_indices.clear();
            for j in start..end {
                if j >= 0 && self.get_symbol(j as usize, y) == self.get_symbol(x, y) {
                    winning_indices.push((j as usize, y));
                }
            }
            if winning_indices.len() == self.sym_occs_win as usize {
                return Some(winning_indices);
            }
        }
        None
    }

    fn check_cols(&self) -> Option<Vec<(usize, usize)>> {
        let (x, y) = self.last_move;

        let mut winning_indices = Vec::with_capacity(self.grid_size);

        let start = y as i32 - self.sym_occs_win as i32 + 1;
        for i in 0..self.grid_size {
            let start = start + i as i32;
            let end = y as i32 + i as i32 + 1;
            winning_indices.clear();
            for j in start..end {
                if j >= 0 && self.get_symbol(x, j as usize) == self.get_symbol(x, y) {
                    winning_indices.push((x, j as usize));
                }
            }
            if winning_indices.len() == self.sym_occs_win as usize {
                return Some(winning_indices);
            }
        }
        None    
    }

    fn check_diag(&self) -> Option<Vec<(usize, usize)>> {
        let (x, y) = self.last_move;

        let mut winning_indices = Vec::with_capacity(self.grid_size);

        let startx = x as i32 - self.sym_occs_win as i32 + 1;
        let starty = y as i32 - self.sym_occs_win as i32 + 1;
        for i in 0..self.grid_size {
            let startx = startx + i as i32;
            let starty = starty + i as i32;
            winning_indices.clear();
            for j in 0..self.grid_size {
                let tx = startx + j as i32;
                let ty = starty + j as i32;
                if tx >= 0 && ty >= 0 && self.get_symbol(tx as usize, ty as usize) == self.get_symbol(x, y) {
                    winning_indices.push((tx as usize, ty as usize));
                }
            }
            if winning_indices.len() == self.sym_occs_win as usize {
                return Some(winning_indices);
            }
        }

        None
    }

    fn check_diag_rev(&self) -> Option<Vec<(usize, usize)>> {
        let (x, y) = self.last_move;

        let mut winning_indices = Vec::with_capacity(self.grid_size);

        let startx = x as i32 + self.sym_occs_win as i32 - 1;
        let starty = y as i32 - self.sym_occs_win as i32 + 1;
        for i in 0..self.grid_size {
            let startx = startx - i as i32;
            let starty = starty + i as i32;
            winning_indices.clear();
            for j in 0..self.grid_size {
                let tx = startx - j as i32;
                let ty = starty + j as i32;
                if tx >= 0 && ty >= 0 && self.get_symbol(tx as usize, ty as usize) == self.get_symbol(x, y) {
                    winning_indices.push((tx as usize, ty as usize));
                }
            }
            if winning_indices.len() == self.sym_occs_win as usize {
                return Some(winning_indices);
            }
        }
        None
    }

    fn get_winner(&self) -> (GameStatus, Option<Vec<(usize, usize)>>) {
        let (x, y) = self.last_move;
        let sym = self.get_symbol(x, y);
        let funcs = [Self::check_cols, Self::check_diag, Self::check_diag_rev, Self::check_line];
        for f in funcs {
            let v = f(self);
            if v.is_some() {
                return (GameStatus::Winner(sym), v);
            }
        }

        for sym in self.symbols.iter() {
            if *sym == Symbol::Empty {
                return (GameStatus::NotFinished, None);
            }
        }
        (GameStatus::Draw, None)

    }
}

#[derive(Default, Debug)]
struct Line {
    start_point: Vector2f,
    length: f32,
    rotation_from_x_axis: f32,
}

impl Line {

    fn new(start_point: Vector2f, end_point: Vector2f) -> Self  {
        let diff_vector = end_point - start_point;
        let length = diff_vector.length_sq().sqrt();
        let line_dir = diff_vector / length;

        let e1 = Vector2f::new(1.0, 0.0);
        let rotation_angle = e1.dot(line_dir).acos().to_degrees();
        Self {
            start_point,
            length,
            rotation_from_x_axis: rotation_angle
        }
    }
}
pub struct DrawableGrid {
    grid: Grid,
    grid_sprites: [RcSprite; 3],
    cells_offset: f32,
    cell_size: Vector2f,
    players_symbols: [Symbol; 2],
    current_symbol: usize,
    should_draw_sprites: bool,
    dimensions: Vector2f,
    position: Vector2f,

    winnign_patters: Vec<(usize, usize)>,
    game_over_line: Line,
    draw_line: bool
}

impl DrawableGrid {
    pub fn new(grid_textures: [&RcTexture; Symbol::Count as usize],
        cells_offset: f32,
        grid_size: usize,
        sym_occs_win: u32,
        dimensions: impl Into<Vector2f>) -> DrawableGrid {

        let mut grid_sprites: [RcSprite; 3] = Default::default();

        for i in 0..Symbol::Count as usize {
            grid_sprites[i].set_texture(grid_textures[i], true);
            grid_sprites[i].set_color(Color::rgb(92,192,192))
        }
        Self {
            grid: Grid::new(grid_size, sym_occs_win),
            grid_sprites,
            cells_offset,
            players_symbols: [Symbol::X, Symbol::O],
            current_symbol: 0,
            cell_size: Vector2f::default(),
            should_draw_sprites: true,
            dimensions: dimensions.into(),
            winnign_patters: Vec::new(),
            game_over_line: Default::default(),
            draw_line: false,
            position: Default::default()
        }
    }

    pub fn get_winner(&mut self) -> GameStatus {
        let (s, v) = self.grid.get_winner();
        if v.is_some() {
            self.winnign_patters = v.unwrap();
        }
        s
    }

    pub fn get_grid_size(&self) -> usize {
        self.grid.grid_size
    }

    pub fn put_symbol(&mut self, x: usize, y: usize) -> bool {
        if self.grid.is_empty(x, y) {
            self.grid.set_symbol(x, y, self.players_symbols[self.current_symbol]);
            self.current_symbol = (self.current_symbol + 1) % 2;
            self.should_draw_sprites = true;

            return true;
        }
        return false;
    }

    pub fn set_position(&mut self, pos: Vector2f) {
        self.position = pos;
    }

    pub fn init_winning_line(&mut self) {
        let index1 = self.winnign_patters[0];
        let index2 = self.winnign_patters.last().unwrap();

        let mut offset = 0.0;
        if index1.0 > index2.0 {
            offset = self.cell_size.x;
        }

        let p1 = Vector2f::from((
            index1.0 as f32  * (self.cell_size.x + self.cells_offset) + self.cells_offset / 2.0 + offset,
            index1.1 as f32  * (self.cell_size.y + self.cells_offset) + self.cells_offset / 2.0
        ));

        let p2 = Vector2f::from((
            index2.0 as f32  * (self.cell_size.x + self.cells_offset) + self.cells_offset / 2.0 - offset,
            index2.1 as f32  * (self.cell_size.y + self.cells_offset) + self.cells_offset / 2.0
        ));

        let factor = (
            (p1.x - p2.x).abs().clamp(0.0, 1.0),
            (p1.y - p2.y).abs().clamp(0.0, 1.0),
        );

        let p2 = p2 + Vector2f::from((factor.0 * self.cell_size.x, factor.1 * self.cell_size.y));

        self.game_over_line = Line::new(p1, p2);

        self.draw_line = true;
    }

    pub fn on_mouse_click(&mut self, x: f32, y: f32) -> bool {
        let pos = Vector2f::new(x, y);
        let half_offset = self.cells_offset / 2.0;

        let num = pos - Vector2f::new(half_offset, half_offset) - self.position;
        let denom = self.cell_size + Vector2::new(self.cells_offset, self.cells_offset);

        let mut grid_pos =  Vector2f::default();
        grid_pos.x = num.x / denom.x;
        grid_pos.y = num.y / denom.y;

        if grid_pos.x < 0.0 || grid_pos.x >= self.dimensions.x ||
            grid_pos.y < 0.0 || grid_pos.y >= self.dimensions.y {
            return false;
        }

        let grid_pos = grid_pos.as_other::<usize>();

        self.put_symbol(grid_pos.x, grid_pos.y)
    }

    pub fn draw(&mut self, target: &mut impl RenderTarget) {
        let half_offset = self.cells_offset / 2.0;

        let cell_size = self.dimensions / self.grid.grid_size as f32 - Vector2::new(self.cells_offset , self.cells_offset );
        self.cell_size = cell_size;
        for sprite in &mut self.grid_sprites {
            let texture_size = sprite.texture().unwrap().size().as_other::<f32>();
            sprite.set_scale((
                cell_size.x / texture_size.x,
                cell_size.y / texture_size.y
            ));
        }

        for x in 0..self.grid.grid_size {
            for y in 0..self.grid.grid_size {
                let sprite = &mut self.grid_sprites[self.grid.get_symbol(x, y) as usize];
                let sprite_pos = Vector2f::new(
                    x as f32 * (cell_size.x + self.cells_offset) + half_offset,
                    y as f32 * (cell_size.y + self.cells_offset) + half_offset
                );
                sprite.set_position(sprite_pos + self.position);

                target.draw(sprite);

            }
        }

        if self.draw_line {
            let p = self.game_over_line.start_point;
            let length = self.game_over_line.length;
            let thickness = 10.0;
            let rect_size = Vector2f::new(length, thickness);
            let mut rect = RectangleShape::with_size(rect_size);
            rect.set_fill_color(Color::RED);
            let mut rect_pos = p;
            if self.game_over_line.rotation_from_x_axis == 0.0 {
                rect_pos.y += (self.cell_size.y - thickness) / 2.0;
            }
            else {
                rect.rotate(self.game_over_line.rotation_from_x_axis);
                if self.game_over_line.rotation_from_x_axis == 90.0 {
                    rect_pos.x += (self.cell_size.y - thickness) / 2.0;
                }
            }
            rect.set_position(rect_pos + self.position);
            target.draw(&rect);
        }

        self.should_draw_sprites = false;
    }

    pub fn can_draw_sprites(&self) -> bool {
        self.should_draw_sprites
    }
}



