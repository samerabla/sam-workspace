use regex::Regex;
use validator::ValidateEmail;

pub fn validate_password(password: &str, password_len: usize) -> Result<(), String> {
    if password.len() == 0 {
        return Err("Empty not allowed".to_string());
    }
    if password.len() < password_len {
        return Err(format!(
            "Your password must be at least {} characters long",
            password_len
        ));
    }

    let lowercase_regex = Regex::new(r"[a-z]").unwrap();
    if !lowercase_regex.is_match(password) {
        return Err("Your password must contain at least one lowercase letter".to_string());
    }

    let uppercase_regex = Regex::new(r"[A-Z]").unwrap();
    if !uppercase_regex.is_match(password) {
        return Err("Your password must contain at least one uppercase letter".to_string());
    }

    let symbol_regex = Regex::new(r"[!@#$%^&*(),.?:{}|<>]").unwrap();
    if !symbol_regex.is_match(password) {
        return Err("Your password must contain at least one symbol".to_string());
    }

    let number_regex = Regex::new(r"[0-9]").unwrap();
    if !number_regex.is_match(password) {
        return Err("Your password must contain at least one number".to_string());
    }
    Ok(())
}

pub fn validate_email(email: &str) -> Result<(), String> {
    if email.len() == 0 {
        return Err("Empty not allowed".to_string());
    }
    if !email.validate_email() {
        return Err("Invalid Email".to_string());
    }
    Ok(())
}
