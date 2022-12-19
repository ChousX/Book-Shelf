use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Default)]
pub enum View {
    #[default]
    Shown,
    Hidden,
}

impl View {
    pub fn is_visible(&self) -> bool {
        match self {
            Self::Hidden => false,
            Self::Shown => true,
        }
    }

    pub fn toggle(&mut self) {
        let t = match self {
            Self::Hidden => Self::Shown,
            Self::Shown => Self::Hidden,
        };
        *self = t;
    }
}
