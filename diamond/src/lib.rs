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

    pub fn create(&self) -> String {
        println!("************************");
        println!(
            "Creating a diamonf for {}",
            self.letter.to_ascii_uppercase()
        );

        let mut result = String::new();
        let mut letter_index = 0;

        // Find letter index
        while letter_index < self.alphabet.len()
            && self.letter.to_ascii_uppercase() != self.alphabet[letter_index]
        {
            letter_index += 1;
        }
        println!("Index of {} is {}", self.letter, letter_index);

        if letter_index == 0 {
            result.push(self.letter.to_ascii_uppercase());
            return result;
        }

        // Calculate diamond size
        let rows = ((letter_index + 1) * 2) - 1;
        let columns = rows;
        let mut buffer: Vec<String> = vec![];

        println!("Diamond has {rows} rows and {columns} columns");

        let mut initial_spaces = 0;
        let mut middle_spaces = 0;
        let mut ending_spaces = 0;

        for row in 0..letter_index + 1 {
            println!("************************");

            let mut row_buffer = "".to_string();
            let current_letter = self.alphabet[row];

            println!("Processing row {} with {}", row, current_letter);

            for _ in 0..letter_index - row {
                row_buffer.push(' ');
            }

            row_buffer.push(current_letter);

            initial_spaces = letter_index - row;
            ending_spaces = initial_spaces;
            if row != 0 {
                middle_spaces = columns - 2 - initial_spaces - ending_spaces
            } else {
                middle_spaces = 0;
            }

            println!("Will have {initial_spaces} initial spaces");
            println!("Will have {middle_spaces} middle spaces");
            println!("Will have {ending_spaces} ending spaces");

            for _ in 0..middle_spaces {
                row_buffer.push(' ');
            }

            if row != 0 {
                row_buffer.push(current_letter);
            }

            for _ in 0..ending_spaces {
                row_buffer.push(' ');
            }

            println!("Row buffer is '{row_buffer}'");
            buffer.push(row_buffer);
        }

        println!("Buffer content is '{:?}'", buffer);

        for i in 0..letter_index + 1 {
            println!(
                "Appending line {} containing {} to result",
                i + 1,
                buffer[i]
            );
            result.push_str(buffer[i].as_str());
            result.push('\n');
        }

        for i in (letter_index + 1)..rows - 1 {
            let distance = i - letter_index;
            println!(
                "Appending line {} containing {} to result",
                i + 1,
                buffer[letter_index - distance]
            );
            result.push_str(buffer[letter_index - distance].as_str());
            result.push('\n');
        }

        println!(
            "Final: Appending line {} containing {} to result",
            rows, buffer[0]
        );

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
}
