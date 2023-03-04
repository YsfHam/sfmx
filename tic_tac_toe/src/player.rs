use sfmx::prelude::*;

use crate::grid::{DrawableGrid, Symbol, GameStatus};
use std::fmt::Display;


#[repr(u8)]
pub enum PlayerType {
    Human = 0,
    AI
}
impl Display for PlayerType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PlayerType::Human => write!(f, "Human"),
            PlayerType::AI => write!(f, "AI"),
        }
    }
}

pub trait Player {
    fn play(&self, grid: &mut DrawableGrid) -> bool;
    fn allow(&mut self);
    fn forbid(&mut self);
    fn on_event(&mut self, event: Event);
    fn set_symbol(&mut self, symbol: Symbol);
}

struct PlayerProps {
    can_play: bool,
    symbol: Symbol,
}

impl PlayerProps {
    pub fn new() -> Self {
        Self {
            can_play: false,
            symbol: Symbol::Empty
        }
    }
}

fn default_player_forbid<T: DefaultPlayer + ?Sized>(obj: &mut T) {
    obj.get_player_props_mut().can_play = false;
}

trait DefaultPlayer {
    fn play(&self, grid: &mut DrawableGrid) -> bool;

    fn allow(&mut self) {
        self.get_player_props_mut().can_play = true;
    }
    fn forbid(&mut self) {
        default_player_forbid(self);
    }

    fn on_event(&mut self, event: Event) {

    }

    fn get_player_props(&self) -> &PlayerProps;
    fn get_player_props_mut(&mut self) -> &mut PlayerProps;

    fn set_symbol(&mut self, symbol: Symbol) {
        self.get_player_props_mut().symbol = symbol;
    }
}

impl<T: DefaultPlayer> Player for T {
    fn allow(&mut self) {
        self.allow();
    }

    fn forbid(&mut self) {
        self.forbid();
    }

    fn on_event(&mut self, event: Event) {
        self.on_event(event);
    }

    fn play(&self, grid: &mut DrawableGrid) -> bool {
        if self.get_player_props().can_play {
            return self.play(grid)
        }
        false
    }

    fn set_symbol(&mut self, symbol: Symbol) {
        self.set_symbol(symbol);
    }
}

pub struct HumanPlayer {
    mouse_position: Vector2f,
    mouse_pressed: bool,
    props: PlayerProps,
}

impl HumanPlayer {
    pub fn new() -> Self {
        Self {
            mouse_position: Default::default(),
            mouse_pressed: false,
            props: PlayerProps::new(),
        }
    }
}

impl DefaultPlayer for HumanPlayer {

    fn play(&self, grid: &mut DrawableGrid) -> bool {
        if self.mouse_pressed {
            return grid.on_mouse_click(self.mouse_position.x, self.mouse_position.y, self.props.symbol);
        }
        false
    }

    fn forbid(&mut self) {
        default_player_forbid(self);
        self.mouse_pressed = false;
    }

    fn on_event(&mut self, event: Event) {
        if let Event::MouseButtonReleased { button, x, y } = event {
            if button == mouse::Button::Left {
                self.mouse_position = Vector2::from((x, y)).as_other();
                self.mouse_pressed = true;
            }
        }
        else {
            self.mouse_pressed = false;
        }
    }

    fn get_player_props_mut(&mut self) -> &mut PlayerProps {
        &mut self.props
    }

    fn get_player_props(&self) -> &PlayerProps {
        &self.props
    }
}

pub struct MiniMaxAI {
    props: PlayerProps,

    minimax_max_depth: i32,
    max_cells_test: u32
}

impl MiniMaxAI {
    pub fn new() -> Self {
        Self {
            props: PlayerProps::new(),
            minimax_max_depth: 5,
            max_cells_test: 16
        }
    }

    fn minimax(&self, opp_sym: Symbol, winning_score: i32, loosing_score: i32, depth: i32, grid: &mut DrawableGrid, is_max: bool, mut alpha: i32, mut beta: i32) -> i32 {
        let status = grid.get_winner();
        if let GameStatus::Winner(sym) = status {
            if sym == self.props.symbol {
                return winning_score - depth;
            }
            else {
                return loosing_score + depth;
            }
        }
        if status == GameStatus::Draw {
            return 0;
        }

        if depth >= self.minimax_max_depth {
            return (winning_score - loosing_score) / 2;
        }

        if is_max {
            let mut best = -100;
            let mut tested_cells = 0;
            for x in 0..grid.get_grid_size() as usize {
                for y in 0..grid.get_grid_size() as usize {
                    if grid.is_empty(x, y) {
                        grid.put_symbol(x, y, self.props.symbol);
                        
                        best = std::cmp::max(best, self.minimax(opp_sym, winning_score, loosing_score, depth+1, grid, !is_max, alpha, beta));
                        grid.clear_cell(x, y);
                        
                        tested_cells += 1;
                        if tested_cells > self.max_cells_test {
                            return best;
                        }

                        alpha = std::cmp::max(alpha, best);
                        if beta <= alpha {
                            return best;
                        }
                    }
                }
            }

            return best;
        }
        else {
            let mut best = 100;
            let mut tested_cells = 0;
            for x in 0..grid.get_grid_size() as usize {
                for y in 0..grid.get_grid_size() as usize {
                    if grid.is_empty(x, y) {
                        
                        grid.put_symbol(x, y, opp_sym);
                        best = std::cmp::min(best, self.minimax(opp_sym, winning_score, loosing_score, depth+1, grid, !is_max, alpha, beta));
                        grid.clear_cell(x, y);

                        tested_cells += 1;
                        if tested_cells > self.max_cells_test {
                            return best;
                        }

                        beta = std::cmp::min(beta, best);
                        if beta <= alpha {
                            return best;
                        }
                    }
                }
            }
            return best;
        }
    }

    fn get_best_move(&self, grid: &mut DrawableGrid) -> (usize, usize) {
        let mut best = i32::MIN;
        let mut res = (0, 0);

        let opp_sym = match self.props.symbol {
            Symbol::X => Symbol::O,
            Symbol::O => Symbol::X,
            _ => panic!("Error symbole {:?} is not allowed", self.props.symbol)
        };

        for x in 0..grid.get_grid_size() as usize {
            for y in 0..grid.get_grid_size() as usize {
                if grid.is_empty(x, y) {
                    grid.put_symbol(x, y, self.props.symbol);
                    let score = self.minimax(opp_sym, 10, -10, 0, grid, false, -100, 100);
                    grid.clear_cell(x, y);
    
                    if score > best {
                        best = score;
                        res = (x, y);
                    }
                }
            }
        }

        println!("best move score {}", best);
        res
    }
}

impl DefaultPlayer for MiniMaxAI {
    fn get_player_props_mut(&mut self) -> &mut PlayerProps {
        &mut self.props
    }

    fn get_player_props(&self) -> &PlayerProps {
        &self.props
    }

    fn play(&self, grid: &mut DrawableGrid) -> bool {

        let (x, y) = self.get_best_move(grid);
        grid.put_symbol(x, y, self.props.symbol);
        true
    }
}