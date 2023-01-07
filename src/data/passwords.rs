use lazy_static::lazy_static;
use passwords::PasswordGenerator;
use regex::Regex;

lazy_static! {
    static ref PASSWORD_REGEX: Regex =
        Regex::new(r"/^(?=.*[a-z])(?=.*[A-Z])(?=.*\d)(?=.*[#$@!%&*?])[A-Za-z\d#$@!%&*?]{12,128}$/")
            .unwrap();
}

pub const PASSWORD_PEPPER: &str = "gkHbhXQG3JIbvyGjI1GfsMAxSQgnI1XesBcfT7GcznBi7Htbd7MD0gJlmYlC5t";

pub fn validate_password(password: &str) -> bool {
    return PASSWORD_REGEX.is_match(password);
}

pub fn generate_password() -> String {
    return PasswordGenerator {
        length: 38,
        numbers: true,
        lowercase_letters: true,
        uppercase_letters: true,
        symbols: true,
        spaces: false,
        exclude_similar_characters: true,
        strict: true,
    }
    .generate_one()
    .unwrap();
}
