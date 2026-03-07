use crate::config::CharMask;

pub fn calculate_entropy(password: &str, mask: CharMask) -> f64 {
    let alphabet_size = alphabet_size_from_mask(mask) as f64;
    if alphabet_size == 0.0 || password.is_empty() {
        return 0.0;
    }

    // Базовая теоретическая энтропия
    let base_entropy = (password.len() as f64) * alphabet_size.log2();

    // Расчет штрафов за словарные последовательности (эвристика)
    let mut penalty = 0.0;
    let chars: Vec<char> = password.chars().collect();
    
    for window in chars.windows(2) {
        let diff = (window[1] as i32) - (window[0] as i32);
        if diff == 0 {
            // Штраф за повторяющиеся символы (например, 'aa', '11')
            penalty += 3.0; 
        } else if diff == 1 || diff == -1 {
            // Штраф за последовательности (например, 'ab', '12', 'zy')
            penalty += 2.0; 
        }
    }

    // Энтропия не может быть отрицательной
    (base_entropy - penalty).max(0.0)
}

fn alphabet_size_from_mask(mask: CharMask) -> usize {
    let mut size = 0;
    if mask.contains(CharMask::LATIN) { size += 26; if mask.contains(CharMask::CAPITAL) { size += 26; } }
    if mask.contains(CharMask::CYRILLIC) { size += 33; if mask.contains(CharMask::CAPITAL) { size += 33; } }
    if mask.contains(CharMask::DIGIT) { size += 10; }
    if mask.contains(CharMask::SPECIAL) { size += 32; }
    size
}