//! Модуль проверки содержимого пароля.
use crate::config::{CharMask, SPECIAL_CHARS};

/// Определяет классы символов, присутствующие в строке.
pub fn analyze_classes(password: &str) -> CharMask {
    let mut mask = CharMask::empty();
    for c in password.chars() {
        if c.is_ascii_lowercase() { mask.insert(CharMask::LATIN); }
        else if c.is_ascii_uppercase() { mask.insert(CharMask::LATIN | CharMask::CAPITAL); }
        else if "абвгдеёжзийклмнопрстуфхцчшщъыьэюя".contains(c) { mask.insert(CharMask::CYRILLIC); }
        else if "АБВГДЕЁЖЗИЙКЛМНОПРСТУФХЦЧШЩЪЫЬЭЮЯ".contains(c) { mask.insert(CharMask::CYRILLIC | CharMask::CAPITAL); }
        else if c.is_ascii_digit() { mask.insert(CharMask::DIGIT); }
        else if SPECIAL_CHARS.contains(c) { mask.insert(CharMask::SPECIAL); }
    }
    mask
}

pub fn satisfies_mask(password: &str, required: CharMask) -> bool {
    analyze_classes(password).contains(required)
}

pub fn has_unique_chars(password: &str) -> bool {
    let mut seen = std::collections::HashSet::new();
    password.chars().all(|c| seen.insert(c))
}