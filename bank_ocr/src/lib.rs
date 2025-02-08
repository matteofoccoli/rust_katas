use std::fs::read_to_string;

pub struct Parser {}

const ACCOUNT_NUMBER_COLUMNS: usize = 27;
const ACCOUNT_NUMBER_ROWS: usize = 3;
const ACCOUNT_NUMBER_EMPTY_ROW_INDEX: usize = 4;
const DIGIT_COLUMNS: usize = 3;
const DIGIT_ROWS: usize = 3;

#[derive(PartialEq, Debug)]
pub struct AccountNumber {
    digits: [u8; 9],
}

impl Parser {
    pub fn parse(&self, file_path: String) -> Vec<AccountNumber> {
        let file_content = read_to_string(file_path).unwrap();

        let mut line_buffer = [[' '; ACCOUNT_NUMBER_COLUMNS]; ACCOUNT_NUMBER_ROWS];
        let mut account_numbers: Vec<AccountNumber> = vec![];

        for (line_index, line) in file_content.lines().into_iter().enumerate() {
            let buffer_line_index = line_index % ACCOUNT_NUMBER_EMPTY_ROW_INDEX;
            if buffer_line_index == ACCOUNT_NUMBER_ROWS {
                account_numbers.push(parse_account_number_line(line_buffer.clone()));
            } else {
                for (current_char_index, current_char) in line.chars().into_iter().enumerate() {
                    line_buffer[buffer_line_index][current_char_index] = current_char;
                }
            }
        }
        account_numbers
    }
}

fn parse_account_number_line(
    buffer: [[char; ACCOUNT_NUMBER_COLUMNS]; ACCOUNT_NUMBER_ROWS],
) -> AccountNumber {
    let mut digits = [0; 9];
    for offset in (0..ACCOUNT_NUMBER_COLUMNS).step_by(DIGIT_COLUMNS) {
        digits[offset / 3] = parse_single_digit(buffer, offset);
    }
    println!("Numbers is {:?}", digits);
    AccountNumber { digits }
}

fn parse_single_digit(
    buffer: [[char; ACCOUNT_NUMBER_COLUMNS]; ACCOUNT_NUMBER_ROWS],
    offset: usize,
) -> u8 {
    let mut digit_buffer = [[' '; DIGIT_COLUMNS]; DIGIT_ROWS];
    for row in 0..DIGIT_ROWS {
        for column in offset..offset + DIGIT_COLUMNS {
            digit_buffer[row][column - offset] = buffer[row][column];
        }
    }
    into_digit(digit_buffer)
}

fn into_digit(buffer: [[char; 3]; 3]) -> u8 {
    match buffer {
        [[' ', '_', ' '], ['|', ' ', '|'], ['|', '_', '|']] => 0,
        [[' ', ' ', ' '], [' ', ' ', '|'], [' ', ' ', '|']] => 1,
        [[' ', '_', ' '], [' ', '_', '|'], ['|', '_', ' ']] => 2,
        [[' ', '_', ' '], [' ', '_', '|'], [' ', '_', '|']] => 3,
        [[' ', ' ', ' '], ['|', '_', '|'], [' ', ' ', '|']] => 4,
        [[' ', '_', ' '], ['|', '_', ' '], [' ', '_', '|']] => 5,
        [[' ', '_', ' '], ['|', '_', ' '], ['|', '_', '|']] => 6,
        [[' ', '_', ' '], [' ', ' ', '|'], [' ', ' ', '|']] => 7,
        [[' ', '_', ' '], ['|', '_', '|'], ['|', '_', '|']] => 8,
        [[' ', '_', ' '], ['|', '_', '|'], [' ', '_', '|']] => 9,
        _ => panic!("{:?} is not a number", buffer),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_account_number_of_zeros() {
        let parser = Parser {};

        let result = parser.parse("./fixtures/single_account_number_with_0s.txt".to_string());

        assert_eq!(vec!(AccountNumber { digits: [0; 9] }), result);
    }

    #[test]
    fn one_account_number() {
        let parser = Parser {};

        let result = parser.parse("./fixtures/single_account_number.txt".to_string());

        assert_eq!(
            vec!(AccountNumber {
                digits: [1, 2, 3, 4, 5, 6, 7, 8, 9]
            }),
            result
        );
    }

    #[test]
    fn two_account_numbers() {
        let parser = Parser {};

        let result = parser.parse("./fixtures/two_account_numbers.txt".to_string());

        assert_eq!(
            vec!(
                AccountNumber {
                    digits: [1, 2, 3, 4, 5, 6, 7, 8, 9]
                },
                AccountNumber {
                    digits: [1, 2, 3, 4, 5, 6, 7, 8, 9]
                }
            ),
            result
        );
    }

    #[test]
    fn four_account_numbers() {
        let parser = Parser {};

        let result = parser.parse("./fixtures/four_account_numbers.txt".to_string());

        assert_eq!(
            vec!(
                AccountNumber {
                    digits: [1, 2, 3, 4, 5, 6, 7, 8, 9]
                },
                AccountNumber {
                    digits: [1, 2, 3, 4, 5, 6, 7, 8, 9]
                },
                AccountNumber { digits: [4; 9] },
                AccountNumber {
                    digits: [8, 9, 8, 9, 8, 9, 8, 9, 8]
                }
            ),
            result
        );
    }
}
