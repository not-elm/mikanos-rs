use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

use common_lib::math::size::Size;

use crate::gop::console::DISPLAY_BACKGROUND_COLOR;
use crate::gop::pixel::pixel_color::PixelColor;
use crate::layers::text::colors::TextColors;
use crate::layers::text::command::{Command, CommandResult};

pub struct Builder {
    colors: TextColors,
    text_unit: Size,
    scrollable: bool,
    prefix: Option<char>,
    commands: Vec<Command>,
}


impl Builder {
    pub const fn new() -> Self {
        Self {
            scrollable: false,
            colors: TextColors::new(PixelColor::yellow(), DISPLAY_BACKGROUND_COLOR),
            text_unit: Size::new(8, 16),
            prefix: None,
            commands: Vec::new(),
        }
    }


    pub fn terminal() -> Self {
        Self::new()
            .set_scrollable()
            .prefix('>')
    }


    pub fn set_scrollable(mut self) -> Self {
        self.scrollable = true;
        self
    }


    pub fn colors(mut self, colors: TextColors) -> Self {
        self.colors = colors;
        self
    }


    pub fn foreground(mut self, foreground: PixelColor) -> Self {
        self.colors
            .set_foreground(foreground);
        self
    }


    pub fn background(mut self, background: PixelColor) -> Builder {
        self.colors
            .set_background(background);
        self
    }


    pub fn prefix(mut self, prefix: char) -> Self {
        self.prefix = Some(prefix);
        self
    }


    pub fn add_command(mut self, command: Command) -> Self {
        self.commands.push(command);
        self
    }


    pub fn build(self) -> TextConfig {
        TextConfig {
            scrollable: self.scrollable,
            colors: self.colors,
            text_unit: self.text_unit,
            prefix: self.prefix,
            commands: self.commands,
        }
    }
}


#[derive(Clone)]
pub struct TextConfig {
    pub colors: TextColors,
    pub text_unit: Size,
    pub scrollable: bool,
    pub prefix: Option<char>,
    commands: Vec<Command>,
}


impl TextConfig {
    #[inline]
    pub fn exists_prefix(&self) -> bool {
        self.prefix.is_some()
    }


    #[inline]
    pub fn not_exists_command(&self) -> bool {
        self.commands.is_empty()
    }


    pub fn try_execute_command(&self, chars: &[char]) -> CommandResult {
        let command: String = chars.iter().collect();
        let args: Vec<&str> = command.split(' ').collect();
        if let Some(command) = self
            .commands
            .iter()
            .find(|c| c.name() == args[0])
        {
            command.execute(&args[1..])
        } else {
            Err(format!("No such command `{}`", args[0]))
        }
    }
}
