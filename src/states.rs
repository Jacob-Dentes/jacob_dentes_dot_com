use dioxus::prelude::*;

#[derive(PartialEq, Default, Clone)]
pub enum NavigationOptions {
    #[default]
    Home,
    Publications,
    CV,
}

#[derive(Clone)]
pub struct NavigationState {
    state: Signal<NavigationOptions>,
}

impl NavigationState {
    pub fn new(state: Signal<NavigationOptions>) -> Self {
        Self { state }
    }

    pub fn state(&self) -> &Signal<NavigationOptions> {
        &self.state
    }
    
    pub fn state_mut(&mut self) -> &mut Signal<NavigationOptions> {
        &mut self.state
    }
}