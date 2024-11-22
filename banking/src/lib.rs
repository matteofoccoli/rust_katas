struct Account;

impl Account {
    fn print_statement(&self) -> String {
        "Date        Amount  Balance".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prints_an_empty_statement() {
        let account = Account;

        let result = account.print_statement();

        assert_eq!("Date        Amount  Balance".to_string(), result);
    }
}
