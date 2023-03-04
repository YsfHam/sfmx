use super::widgets::Widget;
use std::{collections::HashMap, hash::Hash, any::Any};
use crate::sfml_export::*;

pub trait UIWidget: Widget + Any {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

impl<T: Widget + Any> UIWidget for T {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

pub struct UiManager<IDType> {
    widgets: HashMap<IDType, Box<dyn UIWidget>>
}

impl<IDType: Eq + Hash> UiManager<IDType> {
    pub fn new() -> Self {
        Self {
            widgets: HashMap::new()
        }
    }

    pub fn add_widget(&mut self, id: IDType, widget: Box<dyn UIWidget>) {
        assert!(!self.widgets.contains_key(&id), "widget already exists");

        self.widgets.insert(id, widget);
    }

    pub fn get_widget(&mut self, id: &IDType) -> Option<&mut dyn UIWidget> {
        let w = self.widgets.get_mut(&id)?;
        Some(w.as_mut())
    }

    pub fn with_widget<F, Args>(&mut self, id: &IDType, func: F, args: Args)
    where
        F: Fn(&mut dyn UIWidget, Args)
    {
        let widget = self.get_widget(id).unwrap();
        func(widget, args);
    }

    pub fn get_widget_as<W: Widget + 'static>(&mut self, id: &IDType) -> Option<&mut W> {
        let widget = self.widgets.get_mut(&id)?;
        match widget.as_any_mut().downcast_mut::<W>() {
            Some(w) => Some(w),
            None => panic!("Incompatible type for widget id")
        }
    }

    pub fn with_widget_as<W, F, Args>(&mut self, id: &IDType, func: F, args: Args)
    where
        W: Widget + 'static,
        F: Fn(&mut W, Args)
    {
        let widget = self.get_widget_as(id).unwrap();
        func(widget, args);
    }

    pub fn on_event(&mut self, event: Event) {
        for widget in self.widgets.values_mut() {
            widget.on_event(event);
        }
    }

    pub fn reset(&mut self) {
        for widget in self.widgets.values_mut() {
            widget.reset();
        }
    }

    pub fn draw(&self, target: &mut dyn RenderTarget) {
        for widget in self.widgets.values() {
            widget.draw(target);
        }
    }
}