use std::fs::read_to_string;

pub struct Parser {}

const ACCOUNT_NUMBER_BUFFER_COLUMNS: usize = 27;
const ACCOUNT_NUMBER_BUFFER_ROWS: usize = 3;
const ACCOUNT_NUMBER_BUFFER_MAX_ROWS: usize = 4;
const NUMBER_BUFFER_COLUMNS: usize = 3;
const NUMBER_BUFFER_ROWS: usize = 3;

#[derive(PartialEq, Debug)]
pub struct AccountNumber {
    numbers: [u8; 9],
}

impl Parser {
    pub fn parse(&self, file_path: String) -> Vec<AccountNumber> {
        let file_content = read_to_string(file_path).unwrap();
        let mut line_buffer = [[' '; ACCOUNT_NUMBER_BUFFER_COLUMNS]; ACCOUNT_NUMBER_BUFFER_ROWS];
        let mut buffer_line_index;
        let mut account_numbers: Vec<AccountNumber> = vec![];

        for (line_index, line) in file_content.lines().into_iter().enumerate() {
            buffer_line_index = line_index % ACCOUNT_NUMBER_BUFFER_MAX_ROWS;
            if buffer_line_index == ACCOUNT_NUMBER_BUFFER_ROWS {
                account_numbers.push(parse_account_number_line(line_buffer.clone()));
                break;
            }
            for (current_char_index, current_char) in line.chars().into_iter().enumerate() {
                line_buffer[buffer_line_index][current_char_index] = current_char;
            }
        }
        account_numbers
    }
}

fn parse_account_number_line(
    line_buffer: [[char; ACCOUNT_NUMBER_BUFFER_COLUMNS]; ACCOUNT_NUMBER_BUFFER_ROWS],
) -> AccountNumber {
    let mut number_buffer = [[' '; NUMBER_BUFFER_COLUMNS]; NUMBER_BUFFER_ROWS];
    let mut numbers = [9; 9];
    for offset in (0..ACCOUNT_NUMBER_BUFFER_COLUMNS).step_by(NUMBER_BUFFER_COLUMNS) {
        for row in 0..NUMBER_BUFFER_ROWS {
            for column in offset..offset + NUMBER_BUFFER_COLUMNS {
                number_buffer[row][column - offset] = line_buffer[row][column];
            }
        }
        let number = convert(number_buffer);
        numbers[offset / 3] = number;
    }
    println!("Numbers is {:?}", numbers);
    AccountNumber { numbers }
}

fn convert(buffer: [[char; 3]; 3]) -> u8 {
    match buffer {
        [[' ', '_', ' '], ['|', ' ', '|'], ['|', '_', '|']] => 0,
        [[' ', ' ', ' '], [' ', ' ', '|'], [' ', ' ', '|']] => 1,
        _ => todo!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_account_number_of_zeros() {
        let parser = Parser {};

        let result = parser.parse("./all_0s_account_number.txt".to_string());

        assert_eq!(vec!(AccountNumber { numbers: [0; 9] }), result);
    }

    #[test]
    fn one_account_number_of_ones() {
        let parser = Parser {};

        let result = parser.parse("./all_1s_account_number.txt".to_string());

        assert_eq!(vec!(AccountNumber { numbers: [1; 9] }), result);
    }
}
