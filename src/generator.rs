use rand::seq::{IteratorRandom, SliceRandom};
use rand::{Rng, RngExt};
use zeroize::Zeroize;
use std::fmt;
use crate::config::{Config, CharMask};
use crate::error::{Result, ErrorT};

#[derive(Zeroize)]
#[zeroize(drop)]
pub struct SecurePassword(String);

impl SecurePassword {
    pub fn as_str(&self) -> &str { &self.0 }
}

impl fmt::Display for SecurePassword {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub fn generate_password(config: &Config) -> Result<SecurePassword> {
    let mut alphabet = config.build_alphabet();
    if alphabet.is_empty() {
        return Err(ErrorT::InvalidArgument("No character classes selected".into()));
    }

    if !config.repeat_allowed && config.length > alphabet.len() {
        return Err(ErrorT::InvalidArgument(format!(
            "Length {} exceeds alphabet size {} with no repetitions allowed",
            config.length, alphabet.len()
        )));
    }

    let mut rng = rand::rng();
    let mut password_chars = Vec::with_capacity(config.length);

    let mut extract = |pool: &mut Vec<char>, condition: fn(&char) -> bool| {
        if let Some(pos) = pool.iter().position(condition) {
            if config.repeat_allowed {
                password_chars.push(pool[pos]);
            } else {
                password_chars.push(pool.swap_remove(pos));
            }
        }
    };

    // Гарантированное добавление
    if config.resolved_mask.contains(CharMask::LATIN) { extract(&mut alphabet, |c| c.is_ascii_alphabetic()); }
    if config.resolved_mask.contains(CharMask::CYRILLIC) { extract(&mut alphabet, |c| matches!(c, 'а'..='я' | 'А'..='Я' | 'ё' | 'Ё')); }
    if config.resolved_mask.contains(CharMask::DIGIT) { extract(&mut alphabet, |c| c.is_ascii_digit()); }
    if config.resolved_mask.contains(CharMask::SPECIAL) { extract(&mut alphabet, |c| c.is_ascii_punctuation() || c.is_ascii()); }

    if password_chars.len() > config.length {
        return Err(ErrorT::InvalidArgument("Password length is too short to include all requested character classes".into()));
    }

    // Добивка длины
    let remaining = config.length - password_chars.len();
    if config.repeat_allowed {
        for _ in 0..remaining {
            password_chars.push(alphabet[rng.random_range(0..alphabet.len())]);
        }
    } else {
        let mut subset: Vec<char> = alphabet.into_iter().choose_multiple(&mut rng, remaining);
        password_chars.append(&mut subset);
    }

    password_chars.shuffle(&mut rng);
    Ok(SecurePassword(password_chars.into_iter().collect()))
}