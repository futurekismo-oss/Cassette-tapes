use std::sync::atomic::{AtomicBool, Ordering};

static DEBUG: AtomicBool = AtomicBool::new(false);

pub fn set_debug(enabled: bool) {
    DEBUG.store(enabled, Ordering::Relaxed);
    if enabled {
        yansi::disable();
    }
}

pub fn is_debug() -> bool {
    DEBUG.load(Ordering::Relaxed)
}

pub fn status_ok(action: &str) {
    if is_debug() {
        println!("status=ok action={}", action);
    }
}

pub fn status_ok_kv(action: &str, key: &str, value: &str) {
    if is_debug() {
        println!("status=ok action={} {}={}", action, key, value);
    }
}

pub fn status_ok_fields(action: &str, fields: &[(&str, &str)]) {
    if is_debug() {
        let pairs: Vec<String> = fields
            .iter()
            .map(|(k, v)| format!("{}={}", k, v))
            .collect();
        println!("status=ok action={} {}", action, pairs.join(" "));
    }
}

pub fn status_error(message: &str) {
    if is_debug() {
        eprintln!("status=error message={}", message);
    }
}

pub fn hook(cmd: &str) {
    if is_debug() {
        println!("status=ok action=hook command={}", cmd);
    }
}

pub fn info(key: &str, value: &str) {
    if is_debug() {
        println!("{}={}", key, value);
    }
}

pub fn dependency(name: &str) {
    if is_debug() {
        println!("status=ok action=dependency name={}", name);
    }
}

pub fn file_entry(name: &str) {
    if is_debug() {
        println!("status=ok action=file name={}", name);
    }
}

pub fn missing_dep(name: &str) {
    if is_debug() {
        eprintln!("status=error action=missing_dependency name={}", name);
    }
}
