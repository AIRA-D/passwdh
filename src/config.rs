//! Модуль конфигурации и работы с наборами символов.
use crate::error::{ErrorT, Result};
use crate::args::RepeatMode;

/// Набор спецсимволов, используемый в приложении.
pub const SPECIAL_CHARS: &str = "!@#$%^&*()_+[]:;<>,.?~-";

bitflags::bitflags! {
    /// Битовая маска типов символов.
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub struct CharMask: u8 {
        const LATIN = 1 << 0;
        const CYRILLIC = 1 << 1;
        const CAPITAL = 1 << 2;
        const DIGIT = 1 << 3;
        const SPECIAL = 1 << 4;
    }
}

/// Конфигурация для операций с паролями.
pub struct Config {
    pub length: usize,
    pub repeat_allowed: bool,
    pub resolved_mask: CharMask,
}

impl Config {
    pub fn new(length: usize, rep: Option<RepeatMode>, mask: CharMask) -> Self {
        Self {
            length,
            repeat_allowed: matches!(rep, Some(RepeatMode::Yes)),
            resolved_mask: Self::resolve_mask(mask),
        }
    }

    fn resolve_mask(mask: CharMask) -> CharMask {
        let mut resolved = mask;
        if resolved.contains(CharMask::CAPITAL) && 
           !(resolved.contains(CharMask::LATIN) || resolved.contains(CharMask::CYRILLIC)) {
            resolved.insert(CharMask::LATIN);
        }
        resolved
    }

    /// Создает вектор всех доступных символов на основе маски.
    pub fn build_alphabet(&self) -> Vec<char> {
        let mut alphabet = Vec::new();
        if self.resolved_mask.contains(CharMask::LATIN) {
            alphabet.extend('a'..='z');
            if self.resolved_mask.contains(CharMask::CAPITAL) { alphabet.extend('A'..='Z'); }
        }
        if self.resolved_mask.contains(CharMask::CYRILLIC) {
            alphabet.extend("абвгдеёжзийклмнопрстуфхцчшщъыьэюя".chars());
            if self.resolved_mask.contains(CharMask::CAPITAL) {
                alphabet.extend("АБВГДЕЁЖЗИЙКЛМНОПРСТУФХЦЧШЩЪЫЬЭЮЯ".chars());
            }
        }
        if self.resolved_mask.contains(CharMask::DIGIT) {
            alphabet.extend('0'..='9');
        }
        if self.resolved_mask.contains(CharMask::SPECIAL) {
            alphabet.extend(SPECIAL_CHARS.chars());
        }
        alphabet
    }
}

pub fn parse_parameters(s: &str) -> Result<CharMask> {
    let mut mask = CharMask::empty();
    for ch in s.chars() {
        match ch {
            'l' => mask.insert(CharMask::LATIN),
            'c' => mask.insert(CharMask::CYRILLIC),
            'b' => mask.insert(CharMask::CAPITAL),
            'n' => mask.insert(CharMask::DIGIT),
            's' => mask.insert(CharMask::SPECIAL),
            _ => return Err(ErrorT::InvalidArgument(format!("Unknown flag '{}'", ch))),
        }
    }
    Ok(mask)
}

// pub fn parse_repeat(s: Option<&str>) -> Result<bool> {
//     match s {
//         Some("yes") | Some("y") => Ok(true),
//         _ => Ok(false),
//     }
// }