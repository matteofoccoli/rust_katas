pub struct Diamond {
    letter: char,
}

impl Diamond {
    pub fn new(letter: char) -> Self {
        Self { letter }
    }

    pub fn create(&self) -> String {
        "A".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_diamond() {
        let diamond = Diamond::new('a');

        let result = diamond.create();

        assert_eq!("A".to_string(), result);
    }
}
