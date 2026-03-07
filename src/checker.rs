use crate::config::{Config, CharMask};
use crate::validator::{analyze_classes, satisfies_mask, has_unique_chars};
use crate::entropy::calculate_entropy;

pub struct CheckResult {
    pub password: String,
    pub length: usize,
    pub present_classes: CharMask,
    pub unique: bool,
    pub entropy: f64,
    pub meets_requirements: bool,
}

pub fn check_password(password: &str, config: Option<&Config>) -> CheckResult {
    let present = analyze_classes(password);
    let unique = has_unique_chars(password);
    let entropy = calculate_entropy(password, present);
    let meets_requirements = if let Some(cfg) = config {
        // Проверяем длину, если задана
        let length_ok = cfg.length == 0 || password.len() == cfg.length;
        let repeat_ok = !cfg.repeat_allowed || unique; // если запрещены повторы, то unique должен быть true
        let classes_ok = satisfies_mask(password, cfg.resolved_mask);
        length_ok && repeat_ok && classes_ok
    } else {
        true // если нет требований, считаем что соответствует
    };

    CheckResult {
        password: password.to_string(),
        length: password.len(),
        present_classes: present,
        unique,
        entropy,
        meets_requirements,
    }
}