// style.rs
//
// Copyright (c) 2020  Douglas P Lau
//
use crate::text::{Color, Intensity};
use crossterm::style::{Attribute, Attributes};

/// Font weight
///
/// NOTE: Some terminals may treat this as intensity, altering the color rather
///       than font weight.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Weight {
    /// Normal weight (or intensity)
    Normal,
    /// Bold weight (or increased intensity)
    Bold,
    /// Thin weight (or faint / dim / decreased intensity)
    Thin,
}

/// Text appearance
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Appearance {
    /// Crossterm text attributes
    attributes: Attributes,
    /// Font weight
    weight: Weight,
}

/// Text style
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TextStyle {
    /// Background color
    background: Color,
    /// Foreground text color
    foreground: Color,
    /// Text appearance
    appearance: Appearance,
}

impl Default for Weight {
    fn default() -> Self {
        Weight::Normal
    }
}

impl Weight {
    /// Get weight attribute
    fn attribute(self) -> Option<Attribute> {
        match self {
            Weight::Bold => Some(Attribute::Bold),
            Weight::Thin => Some(Attribute::Dim),
            _ => None,
        }
    }
}

impl Appearance {
    /// Set font weight
    pub fn with_weight(mut self, weight: Weight) -> Self {
        self.weight = weight;
        self
    }

    /// Set `italic` text appearance
    pub fn with_italic(mut self, enable: bool) -> Self {
        if enable {
            self.attributes.set(Attribute::Italic);
        } else {
            self.attributes.unset(Attribute::Italic);
        }
        self
    }

    /// Set `strikethrough` text appearance
    pub fn with_strikethrough(mut self, enable: bool) -> Self {
        if enable {
            self.attributes.set(Attribute::CrossedOut);
        } else {
            self.attributes.unset(Attribute::CrossedOut);
        }
        self
    }

    /// Set `underline` text appearance
    pub fn with_underline(mut self, enable: bool) -> Self {
        if enable {
            self.attributes.set(Attribute::Underlined);
        } else {
            self.attributes.unset(Attribute::Underlined);
        }
        self
    }

    /// Set `reverse` text appearance
    pub fn with_reverse(mut self, enable: bool) -> Self {
        if enable {
            self.attributes.set(Attribute::Reverse);
        } else {
            self.attributes.unset(Attribute::Reverse);
        }
        self
    }

    /// Get changed attributes
    pub(crate) fn changed(&self, before: Self) -> Attributes {
        let mut attr = self.attributes;
        if self.weight != before.weight {
            match self.weight.attribute() {
                Some(w) => attr.set(w),
                None => attr.set(Attribute::NormalIntensity),
            }
        }
        if before.attributes.has(Attribute::Italic) {
            attr.set(Attribute::NoItalic);
        }
        if before.attributes.has(Attribute::CrossedOut) {
            attr.set(Attribute::NotCrossedOut);
        }
        if before.attributes.has(Attribute::Underlined) {
            attr.set(Attribute::NoUnderline);
        }
        if before.attributes.has(Attribute::Reverse) {
            attr.set(Attribute::NoReverse);
        }
        attr
    }
}

impl Default for TextStyle {
    fn default() -> Self {
        let background = Color::Black(Intensity::Normal);
        let foreground = Color::White(Intensity::Bright);
        let appearance = Appearance::default();
        Self {
            background,
            foreground,
            appearance,
        }
    }
}

impl TextStyle {
    /// Set the background color
    pub fn with_background(mut self, clr: Color) -> Self {
        self.background = clr;
        self
    }

    /// Set the foreground color
    pub fn with_foreground(mut self, clr: Color) -> Self {
        self.foreground = clr;
        self
    }

    /// Set the text appearance
    pub fn with_appearance(mut self, app: Appearance) -> Self {
        self.appearance = app;
        self
    }

    /// Get the background color
    pub fn background(&self) -> Color {
        self.background
    }

    /// Get the foreground color
    pub fn foreground(&self) -> Color {
        self.foreground
    }

    /// Get the text appearance
    pub fn appearance(&self) -> Appearance {
        self.appearance
    }
}
