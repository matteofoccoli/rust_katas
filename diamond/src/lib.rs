pub struct Diamond {
    letter: char,
    alphabet: Vec<char>,
}

impl Diamond {
    pub fn new(letter: char) -> Self {
        Self {
            letter,
            alphabet: ('A'..='Z').collect(),
        }
    }

    fn get_letter_index(&self) -> usize {
        let mut index = 0;
        while index < self.alphabet.len()
            && self.letter.to_ascii_uppercase() != self.alphabet[index]
        {
            index += 1;
        }
        index
    }

    pub fn create(&self) -> String {
        let mut result = String::new();
        let mut buffer: Vec<String> = vec![];

        let letter_index = self.get_letter_index();
        if letter_index == 0 {
            result.push(self.letter.to_ascii_uppercase());
            return result;
        }

        let diamond_size = ((letter_index + 1) * 2) - 1;

        let mut external_spaces;
        let mut internal_spaces;
        let mut row_buffer;
        let mut current_letter;

        for row in 0..letter_index + 1 {
            current_letter = self.alphabet[row];
            row_buffer = "".to_string();

            external_spaces = letter_index - row;
            if row != 0 {
                internal_spaces = diamond_size - 2 - (2 * external_spaces);
            } else {
                internal_spaces = 0;
            }

            (0..external_spaces).for_each(|_| row_buffer.push(' '));

            row_buffer.push(current_letter);

            (0..internal_spaces).for_each(|_| row_buffer.push(' '));

            if row != 0 {
                row_buffer.push(current_letter);
            }

            (0..external_spaces).for_each(|_| row_buffer.push(' '));

            buffer.push(row_buffer);
        }

        for i in 0..letter_index + 1 {
            result.push_str(buffer[i].as_str());
            result.push('\n');
        }

        for i in (letter_index + 1)..diamond_size - 1 {
            let distance = i - letter_index;
            result.push_str(buffer[letter_index - distance].as_str());
            result.push('\n');
        }

        result.push_str(buffer[0].as_str());
        result
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

    #[test]
    fn b_diamond() {
        let diamond = Diamond::new('b');

        let result = diamond.create();

        let expected_result = r#" A 
B B
 A "#;

        assert_eq!(expected_result, result);
    }

    #[test]
    fn c_diamond() {
        let diamond = Diamond::new('c');

        let result = diamond.create();

        let expected_result = r#"  A  
 B B 
C   C
 B B 
  A  "#;

        assert_eq!(expected_result, result);
    }

    #[test]
    fn d_diamond() {
        let diamond = Diamond::new('d');

        let result = diamond.create();

        let expected_result = r#"   A   
  B B  
 C   C 
D     D
 C   C 
  B B  
   A   "#;

        assert_eq!(expected_result, result);
    }

    #[test]
    fn f_diamond() {
        let diamond = Diamond::new('f');

        let result = diamond.create();

        let expected_result = r#"     A     
    B B    
   C   C   
  D     D  
 E       E 
F         F
 E       E 
  D     D  
   C   C   
    B B    
     A     "#;

        assert_eq!(expected_result, result);
    }
}
