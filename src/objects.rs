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

#[pyclass]
#[derive(Clone, Copy)]
pub enum EmojiSource {
    Twitter,
    Apple,
    Google,
    Microsoft,
    Samsung,
    WhatsApp,
    // Facebook, NOT WORKING
    JoyPixels,
    OpenMoji,
    Emojidex,
    Messenger,
    Mozilla,
    Lg,
    Htc,

    Twemoji,
}

impl EmojiSource {
    pub fn to_emoji_source(&self) -> imagetext::emoji::source::EmojiSource {
        match self {
            EmojiSource::Twitter => imagetext::emoji::source::EmojiSource::Twitter,
            EmojiSource::Apple => imagetext::emoji::source::EmojiSource::Apple,
            EmojiSource::Google => imagetext::emoji::source::EmojiSource::Google,
            EmojiSource::Microsoft => imagetext::emoji::source::EmojiSource::Microsoft,
            EmojiSource::Samsung => imagetext::emoji::source::EmojiSource::Samsung,
            EmojiSource::WhatsApp => imagetext::emoji::source::EmojiSource::WhatsApp,
            EmojiSource::JoyPixels => imagetext::emoji::source::EmojiSource::JoyPixels,
            EmojiSource::OpenMoji => imagetext::emoji::source::EmojiSource::OpenMoji,
            EmojiSource::Emojidex => imagetext::emoji::source::EmojiSource::Emojidex,
            EmojiSource::Messenger => imagetext::emoji::source::EmojiSource::Messenger,
            EmojiSource::Mozilla => imagetext::emoji::source::EmojiSource::Mozilla,
            EmojiSource::Lg => imagetext::emoji::source::EmojiSource::Lg,
            EmojiSource::Htc => imagetext::emoji::source::EmojiSource::Htc,
            EmojiSource::Twemoji => imagetext::emoji::source::EmojiSource::Twemoji,
        }
    }
}

#[derive(FromPyObject, Clone, Copy)]
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
            source: self.source.to_emoji_source(),
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
            source: EmojiSource::Twitter,
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
