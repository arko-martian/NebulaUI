//! # Nebula Components - Building Blocks for Beautiful UIs! 🧱
//! 
//! Reactive, accessible, and works on ANY hardware!
//! 
//! ## Components:
//! - **Button**: Interactive buttons with click handlers
//! - **Text**: Reactive text display
//! 
//! ## Example Counter App:
//! ```rust,ignore
//! use nebula_components::{Button, Text};
//! use nebula_core::Signal;
//! 
//! // Create reactive counter
//! let count = Signal::new(0);
//! 
//! // Create text that displays count
//! let text = Text::from_signal(count.clone());
//! 
//! // Create increment button
//! let inc_button = Button::new("+")
//!     .on_click(move || count.update(|c| c + 1));
//! 
//! // Create decrement button
//! let dec_button = Button::new("-")
//!     .on_click(move || count.update(|c| c - 1));
//! ```

pub mod button;
pub mod text;
pub mod container;
pub mod spacer;
pub mod divider;
pub mod checkbox;
pub mod radio;
pub mod textfield;
pub mod grid;
pub mod image;
pub mod image_cache;
pub mod scroll;

pub use button::Button;
pub use text::Text;
pub use container::{VStack, HStack, ZStack, Alignment};
pub use spacer::{Spacer, SpacerType};
pub use divider::{Divider, DividerOrientation, DividerColor};
pub use checkbox::Checkbox;
pub use radio::{Radio, RadioGroup};
pub use textfield::TextField;
pub use grid::Grid;
pub use image::{Image, ImageSource, ImageState, ImageFit};
pub use image_cache::{ImageCache, CachedImage};
pub use scroll::{ScrollView, ScrollDirection};
