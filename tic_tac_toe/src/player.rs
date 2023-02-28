use sfmx::prelude::*;

use crate::grid::DrawableGrid;
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
}

struct PlayerProps {
    can_play: bool
}

impl PlayerProps {
    pub fn new() -> Self {
        Self {
            can_play: false
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
}

pub struct HumanPlayer {
    mouse_position: Vector2f,
    mouse_pressed: bool,
    props: PlayerProps
}

impl HumanPlayer {
    pub fn new() -> Self {
        Self {
            mouse_position: Default::default(),
            mouse_pressed: false,
            props: PlayerProps::new()
        }
    }
}

impl DefaultPlayer for HumanPlayer {

    fn play(&self, grid: &mut DrawableGrid) -> bool {
        if self.mouse_pressed {
            return grid.on_mouse_click(self.mouse_position.x, self.mouse_position.y);
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


pub struct DumpPlayer {
    props: PlayerProps
}

impl DumpPlayer {
    pub fn new() -> DumpPlayer {
        Self {
            props: PlayerProps::new()
        }
    }
}

impl DefaultPlayer for DumpPlayer {
    fn get_player_props_mut(&mut self) -> &mut PlayerProps {
        &mut self.props
    }

    fn get_player_props(&self) -> &PlayerProps {
        &self.props
    }

    fn play(&self, grid: &mut DrawableGrid) -> bool {
        let mut x = 0usize;
        let mut y = 0usize;
        while !grid.put_symbol(x, y) {
            x += 1;
            if x >= grid.get_grid_size() as usize{
                y += 1;
                x = 0;
            }

            if y >= grid.get_grid_size() as usize{
                return false;
            }
        }
        
        true
    }
}