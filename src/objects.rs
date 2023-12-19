use pyo3::prelude::*;

#[derive(FromPyObject)]
pub struct Color(pub [u8; 4]);

#[pyclass]
pub enum TextAlign {
    Left,
    Center,
    Right,
}

impl TextAlign {
    pub fn to_align(&self) -> imagetext::outliner::TextAlign {
        match self {
            TextAlign::Left => imagetext::outliner::TextAlign::Left,
            TextAlign::Center => imagetext::outliner::TextAlign::Center,
            TextAlign::Right => imagetext::outliner::TextAlign::Right,
        }
    }
}

#[derive(Clone)]
#[pyclass]
pub struct EmojiSource(pub imagetext::emoji::source::EmojiSource);

#[allow(non_snake_case)]
#[pymethods]
impl EmojiSource {
    #[staticmethod]
    pub fn Twitter() -> Self {
        Self(imagetext::emoji::source::EmojiSource::Twitter)
    }

    #[staticmethod]
    pub fn Apple() -> Self {
        Self(imagetext::emoji::source::EmojiSource::Apple)
    }

    #[staticmethod]
    pub fn Google() -> Self {
        Self(imagetext::emoji::source::EmojiSource::Google)
    }

    #[staticmethod]
    pub fn Microsoft() -> Self {
        Self(imagetext::emoji::source::EmojiSource::Microsoft)
    }

    #[staticmethod]
    pub fn Samsung() -> Self {
        Self(imagetext::emoji::source::EmojiSource::Samsung)
    }

    #[staticmethod]
    pub fn WhatsApp() -> Self {
        Self(imagetext::emoji::source::EmojiSource::WhatsApp)
    }

    #[staticmethod]
    pub fn JoyPixels() -> Self {
        Self(imagetext::emoji::source::EmojiSource::JoyPixels)
    }

    #[staticmethod]
    pub fn OpenMoji() -> Self {
        Self(imagetext::emoji::source::EmojiSource::OpenMoji)
    }

    #[staticmethod]
    pub fn Emojidex() -> Self {
        Self(imagetext::emoji::source::EmojiSource::Emojidex)
    }

    #[staticmethod]
    pub fn Messenger() -> Self {
        Self(imagetext::emoji::source::EmojiSource::Messenger)
    }

    #[staticmethod]
    pub fn Mozilla() -> Self {
        Self(imagetext::emoji::source::EmojiSource::Mozilla)
    }

    #[staticmethod]
    pub fn Lg() -> Self {
        Self(imagetext::emoji::source::EmojiSource::Lg)
    }

    #[staticmethod]
    pub fn Htc() -> Self {
        Self(imagetext::emoji::source::EmojiSource::Htc)
    }

    #[staticmethod]
    pub fn Twemoji() -> Self {
        Self(imagetext::emoji::source::EmojiSource::Twemoji)
    }

    #[staticmethod]
    pub fn Dir(path: String) {
        imagetext::emoji::source::EmojiSource::Dir(path);
    }
}

#[derive(FromPyObject, Clone)]
pub struct EmojiOptions {
    pub scale: f32,
    pub shift: (i64, i64),

    pub parse_shortcodes: bool,
    pub parse_discord_emojis: bool,
    pub source: EmojiSource,
}

impl EmojiOptions {
    pub fn to_emoji_options(&self) -> imagetext::emoji::EmojiOptions {
        imagetext::emoji::EmojiOptions {
            scale: self.scale,
            shift: self.shift,

            parse_shortcodes: self.parse_shortcodes,
            parse_discord_emojis: self.parse_discord_emojis,
            source: self.source.0.clone(),
        }
    }
}

impl Default for EmojiOptions {
    fn default() -> Self {
        Self {
            scale: 1.0,
            shift: (0, 0),

            parse_shortcodes: true,
            parse_discord_emojis: false,
            source: EmojiSource(imagetext::emoji::source::EmojiSource::Twitter),
        }
    }
}

#[derive(Clone, Copy)]
#[pyclass]
pub enum WrapStyle {
    Word,
    Character,
}

impl WrapStyle {
    pub fn to_wrap_style(&self) -> imagetext::wrap::WrapStyle {
        match self {
            WrapStyle::Word => imagetext::wrap::WrapStyle::Word,
            WrapStyle::Character => imagetext::wrap::WrapStyle::Character,
        }
    }
}
