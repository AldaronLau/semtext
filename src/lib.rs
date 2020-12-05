// lib.rs      semtext crate.
//
// Copyright (c) 2020  Douglas Lau
//
//! Semtext is a Rust library for building text user interfaces, or **TUI**s.
//! It depends on the excellent [crossterm] library, which allows it to work
//! seamlessly on Linux, Mac or Windows.
//!
//! [crossterm]: https://github.com/crossterm-rs/crossterm

#![forbid(unsafe_code)]

mod bbox;
mod border;
mod bounds;
mod error;
mod label;
mod layout;
mod screen;
mod spacer;
mod widget;

use crate::bbox::Dim;
pub use crate::bbox::{BBox, Edge};
pub use crate::border::{Border, LineStyle};
pub use crate::bounds::AreaBound;
use crate::bounds::LengthBound;
pub use crate::error::{Error, Result};
pub use crate::label::Label;
pub use crate::layout::Layout;
pub use crate::screen::{Cells, Glyph, IntoGlyph, Screen};
pub use crate::spacer::Spacer;
pub use crate::widget::Widget;
