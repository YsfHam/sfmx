#![allow(unused_variables)]

use std::collections::{VecDeque, vec_deque::IterMut};
use crate::application::AppSignal;
use crate::sfml_export::*;
use crate::assets_manager::{DefaultAssetsManager, AssetsManager};

pub struct StateData<T> {
    pub data: T,
    pub delta_time: f32,
    pub assets_manager: DefaultAssetsManager
}

impl<T> StateData<T> {
    pub(crate) fn new(data: T) -> StateData<T> {
        Self {
            data,
            delta_time: 0.0,
            assets_manager: AssetsManager::default()
        }
    }
}

type StateRef<T> = Box<dyn State<T>>;

pub enum Transition<Data> {
    None,
    Add(StateRef<Data>),
    Replace(StateRef<Data>),
    Remove,
    Quit
}

pub trait State<Data> {
    fn on_init(&mut self, state_data: &mut StateData<Data>) {}
    fn on_end(&mut self, state_data: &mut StateData<Data>) {}
    
    fn on_event(&mut self, event: Event, state_data: &mut StateData<Data>) -> Transition<Data> { Transition::None }
    fn on_update(&mut self, state_data: &mut StateData<Data>) -> Transition<Data> { Transition::None }
    fn on_pause(&mut self, state_data: &mut StateData<Data>) {}
    fn on_resume(&mut self, state_data: &mut StateData<Data>) {}
    fn on_render(&mut self, state_data: &mut StateData<Data>, window: &mut RenderWindow) {}
}


struct StatesStack<Data> {
    states: VecDeque<StateRef<Data>>,
}

impl<Data> StatesStack<Data> {
    fn new() -> Self {
        Self {
            states: VecDeque::new(),
        }
    }

    fn top(&mut self) -> &mut StateRef<Data> {
        self.states.back_mut().unwrap()
    }

    fn pop(&mut self) -> StateRef<Data> {
        self.states.pop_back().unwrap()
    }

    fn push(&mut self, state: StateRef<Data>) {
        self.states.push_back(state);
    }

    fn has_state(&self) -> bool {
        self.states.len() > 0
    }

    fn iter_mut(&mut self) -> IterMut<'_, StateRef<Data>> {
        self.states.iter_mut()
    }
}

pub(crate) struct StateMachine<Data> {
    states_stack: StatesStack<Data>,
    pub(crate) states_data: StateData<Data>
}

impl<Data> StateMachine<Data> {
    pub(crate) fn new(initial_state: StateRef<Data>, states_data: StateData<Data>) -> Self {
        let mut res = Self {
            states_stack: StatesStack::new(),
            states_data
        };

        res.add_state(initial_state, false);

        res

    }

    pub(crate) fn on_update(&mut self) -> AppSignal {
        let trans = self.states_stack.top().on_update(&mut self.states_data);

        self.transition(trans)
    }

    pub(crate) fn on_event(&mut self, event: Event) -> AppSignal {
        let trans = self.states_stack.top().on_event(event, &mut self.states_data);

        self.transition(trans)
    }

    pub(crate) fn on_render(&mut self, window: &mut RenderWindow) {
        self.states_stack.top().on_render(&mut self.states_data, window);
    }

    pub(crate) fn terminate(&mut self) {
        for state in self.states_stack.iter_mut() {
            state.on_end(&mut self.states_data);
        }
    }

    fn transition(&mut self, trans: Transition<Data>) -> AppSignal {
        match trans {
            Transition::Add(new_state) => {
                self.add_state(new_state, true);
                AppSignal::None
            },
            Transition::Remove => {
                self.remove_state(true);
                AppSignal::None
            },
            Transition::Replace(new_state) => {
                self.remove_state(false);
                self.add_state(new_state, false);
                AppSignal::None
            },
            Transition::None => AppSignal::None,
            Transition::Quit => AppSignal::Quit,
        }
    }

    fn add_state(&mut self, new_state: StateRef<Data>, pause: bool) {
        if self.states_stack.has_state() && pause {
            self.states_stack.top().on_pause(&mut self.states_data);
        }

        self.states_stack.push(new_state);

        let top_state = self.states_stack.top();
        top_state.on_init(&mut self.states_data);
    }

    fn remove_state(&mut self, resume: bool) {
        let mut removed_state = self.states_stack.pop();

        removed_state.on_end(&mut self.states_data);

        if self.states_stack.has_state() && resume {
            self.states_stack.top().on_resume(&mut self.states_data);
        }
    }
}