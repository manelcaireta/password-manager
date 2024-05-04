use super::length::PasswordLengths;
use super::{Password, PasswordBuilder};
use rand::{self, Rng};
use rand::seq::SliceRandom;

impl PasswordBuilder {
    pub fn build(&self, name: String) -> Password {
        // between 8 and 20 characters
        let length = (rand::random::<u8>() % 13) + 8;
        let password_lengths = PasswordLengths::new(length);

        let password_value = self.generate_password_value(password_lengths);

        Password {
            name,
            value: password_value,
        }
    }

    fn generate_password_value(
        &self,
        password_lengths: PasswordLengths,
    ) -> String {
        let mut password_characters = Vec::new();
        for _ in 0..password_lengths.lower() {
            password_characters.push(self.generate_lowercase_letter());
        }
        for _ in 0..password_lengths.upper() {
            password_characters.push(self.generate_uppercase_letter());
        }
        for _ in 0..password_lengths.punctuation() {
            password_characters.push(self.generate_punctuation());
        }
        for _ in 0..password_lengths.numbers() {
            password_characters.push(self.generate_digit());
        }

        password_characters.shuffle(&mut rand::thread_rng());
        String::from_iter(password_characters)
    }

    fn generate_lowercase_letter(&self) -> char {
        let unicode_scalar_value: u8 = rand::thread_rng().gen_range(97..=122);
        unicode_scalar_value as char
    }

    fn generate_uppercase_letter(&self) -> char {
        let unicode_scalar_value: u8 = rand::thread_rng().gen_range(65..=90);
        unicode_scalar_value as char
    }

    fn generate_punctuation(&self) -> char {
        let mut rng = rand::thread_rng();
        let unicode_scalar_value: u8 = if rng.gen::<bool>() {
            rng.gen_range(33..=47)
        } else {
            rng.gen_range(58..=64)
        };
        unicode_scalar_value as char
    }

    fn generate_digit(&self) -> char {
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
        let password_creator = PasswordBuilder;
        let password = password_creator.build("test_password".to_string());
        assert!(has_lowercase(&password));
        assert!(has_uppercase(&password));
        assert!(has_punctuation(&password));
        assert!(has_numbers(&password));
        assert!(password.value.len() > 8);
        assert!(password.value.len() < 50);
    }
}
