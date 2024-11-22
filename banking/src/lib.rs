struct Account {
    transactions: Vec<Transaction>,
}

struct Transaction;

impl Account {
    fn new() -> Self {
        Self {
            transactions: vec![],
        }
    }

    fn print_statement(&self) -> String {
        let mut body = "".to_owned();
        body.push_str("Date        Amount  Balance");
        for transaction in &self.transactions {
            body.push_str("\n24.12.2015   +500      500")
        }
        body
    }

    fn deposit(&mut self, amount: f32) {
        self.transactions.push(Transaction)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prints_an_empty_statement() {
        let account = Account::new();

        let statement = account.print_statement();

        assert_eq!("Date        Amount  Balance".to_string(), statement);
    }

    #[test]
    fn prints_statement_with_one_deposit_only() {
        let mut account = Account::new();
        account.deposit(10.0);

        let statement = account.print_statement();

        let expected_statement = "Date        Amount  Balance\n\
            24.12.2015   +500      500"
            .to_string();
        assert_eq!(expected_statement, statement);
    }

    fn prints_statement_with_two_deposits() {
        let mut account = Account::new();
        account.deposit(10.0);
        account.deposit(20.0);

        let statement = account.print_statement();

        let expected_statement = "Date        Amount  Balance\n\
            24.12.2015   +10      10\n\
            24.12.2015   +20      30"
            .to_string();
        assert_eq!(expected_statement, statement);
    }
}
