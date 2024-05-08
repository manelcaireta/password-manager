use super::length::PasswordLengths;
use super::Password;
use rand;
use rand::seq::SliceRandom;
use rand::Rng;

#[derive(Clone)]
pub struct PasswordBuilder {
    min_length: u8,
    max_length: u8,
}

impl Default for PasswordBuilder {
    fn default() -> Self {
        PasswordBuilder {
            min_length: 8,
            max_length: 16,
        }
    }
}

impl PasswordBuilder {
    pub fn new() -> Self {
        PasswordBuilder::default()
    }

    pub fn min_length(mut self, min_length: u8) -> Self {
        if min_length < 1 || min_length > self.max_length {
            panic!("Minimum length should be between 0 and maximum length");
        }
        self.min_length = min_length;
        self
    }

    pub fn max_length(mut self, max_length: u8) -> Self {
        if self.min_length > max_length {
            panic!("Maximum length should be greater than minimum length");
        }
        self.max_length = max_length;
        self
    }
}

impl PasswordBuilder {
    pub fn build(&self, name: String) -> Password {
        let password_lengths = self.gen_password_lengths();
        let password_value = self.gen_password_from_lengths(password_lengths);

        Password::new(name, password_value)
    }

    fn gen_password_lengths(&self) -> PasswordLengths {
        let length_difference = self.max_length - self.min_length + 1;
        let total_length =
            (rand::random::<u8>() % length_difference) + self.min_length;
        let password_lengths = PasswordLengths::new(total_length);
        password_lengths
    }

    fn gen_password_from_lengths(&self, password_lengths: PasswordLengths) -> String {
        let mut password_characters = Vec::new();
        for _ in 0..password_lengths.lower() {
            password_characters.push(self.gen_lowercase_letter());
        }
        for _ in 0..password_lengths.upper() {
            password_characters.push(self.gen_uppercase_letter());
        }
        for _ in 0..password_lengths.punctuation() {
            password_characters.push(self.gen_punctuation());
        }
        for _ in 0..password_lengths.numbers() {
            password_characters.push(self.gen_digit());
        }

        password_characters.shuffle(&mut rand::thread_rng());
        String::from_iter(password_characters)
    }

    fn gen_lowercase_letter(&self) -> char {
        let unicode_scalar_value: u8 = rand::thread_rng().gen_range(97..=122);
        unicode_scalar_value as char
    }

    fn gen_uppercase_letter(&self) -> char {
        let unicode_scalar_value: u8 = rand::thread_rng().gen_range(65..=90);
        unicode_scalar_value as char
    }

    fn gen_punctuation(&self) -> char {
        let mut rng = rand::thread_rng();
        let unicode_scalar_value: u8 = if rng.gen::<bool>() {
            rng.gen_range(33..=47)
        } else {
            rng.gen_range(58..=64)
        };
        unicode_scalar_value as char
    }

    fn gen_digit(&self) -> char {
        let unicode_scalar_value: u8 = rand::thread_rng().gen_range(48..=57);
        unicode_scalar_value as char
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn has_uppercase(password: &Password) -> bool {
        for character in password.value.chars() {
            if character.is_ascii_uppercase() {
                return true;
            }
        }
        false
    }

    fn has_lowercase(password: &Password) -> bool {
        for character in password.value.chars() {
            if character.is_ascii_lowercase() {
                return true;
            }
        }
        false
    }

    fn has_punctuation(password: &Password) -> bool {
        for character in password.value.chars() {
            if character.is_ascii_punctuation() {
                return true;
            }
        }
        false
    }

    fn has_numbers(password: &Password) -> bool {
        for character in password.value.chars() {
            if character.is_ascii_digit() {
                return true;
            }
        }
        false
    }

    #[test]
    fn secure_password() {
        let password_creator = PasswordBuilder::new();
        let password = password_creator.build("test_password".to_string());
        assert!(has_lowercase(&password));
        assert!(has_uppercase(&password));
        assert!(has_punctuation(&password));
        assert!(has_numbers(&password));
        assert!(password.value.len() > 8);
        assert!(password.value.len() < 16);
    }

    #[test]
    #[should_panic]
    fn invalid_min_length() {
        let password_creator = PasswordBuilder::new().min_length(0);
        password_creator.build("test_password".to_string());
    }

    #[test]
    #[should_panic]
    fn invalid_max_length() {
        let mut password_creator = PasswordBuilder::new();
        password_creator = password_creator
            .clone()
            .max_length(password_creator.min_length - 1);
        password_creator.build("test_password".to_string());
    }
}
