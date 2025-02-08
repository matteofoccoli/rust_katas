use std::fs::read_to_string;

pub struct Parser {}

const LINE_BUFFER_WIDTH: usize = 27;
const LINE_BUFFER_HEIGHT: usize = 3;
const NUMBER_BUFFER_WIDTH: usize = 3;
const NUMBER_BUFFER_HEIGHT: usize = 3;

#[derive(PartialEq, Debug)]
pub struct AccountNumber {
    number: [u8; 9],
}

impl Parser {
    pub fn parse(&self, file_path: String) -> Vec<AccountNumber> {
        let file_content = read_to_string(file_path).unwrap();
        let mut line_buffer = [[' '; LINE_BUFFER_WIDTH]; LINE_BUFFER_HEIGHT];
        let mut number_buffer = [[' '; NUMBER_BUFFER_WIDTH]; NUMBER_BUFFER_HEIGHT];
        let mut buffer_line_index;

        for (line_index, line) in file_content.lines().into_iter().enumerate() {
            buffer_line_index = line_index % 4;
            if buffer_line_index == 3 {
                println!("> Reached empty line {}, processing buffer", line_index);
                // TODO processare il contenuto del buffer a questo punto
                println!("Buffer content is: {:?}", line_buffer);
                break;
            }
            println!("> Parsing line {}, content is: '{}'", line_index, line);
            for (current_char_index, current_char) in line.chars().into_iter().enumerate() {
                line_buffer[buffer_line_index][current_char_index] = current_char;
            }
            println!(
                "> Buffer line {}, content is: {:?}",
                buffer_line_index, line_buffer[buffer_line_index]
            );
        }

        vec![AccountNumber {
            number: [1, 2, 3, 4, 5, 6, 7, 8, 9],
        }]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_parsing() {
        let parser = Parser {};

        let result = parser.parse("./accounts.txt".to_string());

        assert_eq!(
            vec!(AccountNumber {
                number: [1, 2, 3, 4, 5, 6, 7, 8, 9]
            }),
            result
        );
    }
}
