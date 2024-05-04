use rand::seq::SliceRandom;
use rand::Rng;

/// Lengths for the different characters the password will need
pub struct PasswordLengths {
    /// Number of lowercase letters
    lower: u8,
    /// Number of uppercase letters
    upper: u8,
    /// Number of punctuation characters
    punctuation: u8,
    /// Number of numbers
    numbers: u8,
}

impl PasswordLengths {
    pub fn new(total_length: u8) -> Self {
        let mut rng = rand::thread_rng();
        let mut lengths = Vec::new();

        if total_length < 4 {
            panic!("Total length must be at least 4");
        }

        (0..3)
            .map(|_| rng.gen_range(1..(total_length - 3)))
            .for_each(|length| lengths.push(length));

        let sum: u8 = lengths.iter().sum();

        if sum >= total_length {
            return PasswordLengths::new(total_length);
        }
        lengths.push(total_length - sum);
        lengths.shuffle(&mut rng);

        PasswordLengths {
            lower: lengths[0],
            upper: lengths[1],
            punctuation: lengths[2],
            numbers: lengths[3],
        }
    }

    pub fn total_length(&self) -> u8 {
        self.lower + self.upper + self.punctuation + self.numbers
    }

    pub fn lower(&self) -> u8 {
        self.lower
    }

    pub fn upper(&self) -> u8 {
        self.upper
    }

    pub fn punctuation(&self) -> u8 {
        self.punctuation
    }  

    pub fn numbers(&self) -> u8 {
        self.numbers
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let lengths = PasswordLengths::new(10);
        assert!(lengths.lower > 0);
        assert!(lengths.upper > 0);
        assert!(lengths.punctuation > 0);
        assert!(lengths.numbers > 0);
        assert!(lengths.total_length() == 10);
    }
}