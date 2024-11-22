use chrono::{DateTime, Datelike, Utc};

struct Account {
    transactions: Vec<Transaction>,
}

struct Transaction {
    timestamp: DateTime<Utc>,
    amount: f32,
}

impl Account {
    fn new() -> Self {
        Self {
            transactions: vec![],
        }
    }

    fn deposit(&mut self, amount: f32, timestamp: DateTime<Utc>) {
        self.transactions.push(Transaction { timestamp, amount })
    }

    fn withdraw(&mut self, amount: f32, timestamp: DateTime<Utc>) {
        self.transactions.push(Transaction {
            timestamp,
            amount: -amount,
        })
    }

    fn print_statement(&self) -> String {
        let mut balance = 0.0;
        let mut body = "".to_owned();
        body.push_str("Date        Amount  Balance");
        for transaction in &self.transactions {
            balance += transaction.amount;
            body.push_str(&self.print_transaction_line(&transaction, balance));
        }
        body
    }

    fn print_transaction_line(&self, transaction: &Transaction, current_balance: f32) -> String {
        if transaction.timestamp.month() < 10 {
            self.get_transaction_line_for_month_with_less_than_two_digits(
                transaction,
                current_balance,
            )
        } else {
            self.get_transaction_line_for_month_with_two_digits(transaction, current_balance)
        }
    }

    fn get_transaction_sign(&self, transaction: &Transaction) -> char {
        let transaction_sign = if transaction.amount >= 0.0 { '+' } else { '-' };
        transaction_sign
    }

    fn get_transaction_line_for_month_with_two_digits(
        &self,
        transaction: &Transaction,
        current_balance: f32,
    ) -> String {
        format!(
            "\n{}   {}{}      {}",
            transaction.timestamp.format("%d.%-m.%Y"),
            self.get_transaction_sign(transaction),
            transaction.amount.abs(),
            current_balance
        )
    }

    fn get_transaction_line_for_month_with_less_than_two_digits(
        &self,
        transaction: &Transaction,
        current_balance: f32,
    ) -> String {
        format!(
            "\n{}    {}{}      {}",
            transaction.timestamp.format("%d.%-m.%Y"),
            self.get_transaction_sign(transaction),
            transaction.amount.abs(),
            current_balance
        )
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
        account.deposit(500.0, parse_date("2015-12-24"));

        let statement = account.print_statement();

        let expected_statement = "Date        Amount  Balance\n\
            24.12.2015   +500      500"
            .to_string();
        assert_eq!(expected_statement, statement);
    }

    #[test]
    fn prints_statement_with_two_deposits() {
        let mut account = Account::new();
        account.deposit(10.0, parse_date("2015-12-22"));
        account.deposit(20.0, parse_date("2015-12-24"));

        let statement = account.print_statement();

        let expected_statement = "Date        Amount  Balance\n\
            22.12.2015   +10      10\n\
            24.12.2015   +20      30"
            .to_string();
        assert_eq!(expected_statement, statement);
    }

    #[test]
    fn prints_statement_with_one_deposit_and_one_withdraw() {
        let mut account = Account::new();
        account.deposit(500.0, parse_date("2015-12-24"));
        account.withdraw(100.0, parse_date("2016-08-23"));

        let statement = account.print_statement();

        let expected_statement = "Date        Amount  Balance\n\
            24.12.2015   +500      500\n\
            23.8.2016    -100      400"
            .to_string();
        assert_eq!(expected_statement, statement);
    }

    #[test]
    fn prints_statement_with_transaction_ordered_by_timestamp_ascending() {
        let mut account = Account::new();
        account.deposit(500.0, parse_date("2018-12-24"));
        account.deposit(100.0, parse_date("2017-11-23"));

        let statement = account.print_statement();

        let expected_statement = "Date        Amount  Balance\n\
            23.11.2017   +100      100\n\
            24.12.2018   +500      400"
            .to_string();
        assert_eq!(expected_statement, statement);
    }

    fn parse_date(date_string: &str) -> DateTime<Utc> {
        let date_string = &format!("{} 00:00:00.000 +0000", date_string);
        let date_format = "%Y-%m-%d %H:%M:%S%.3f %z";
        let result = DateTime::parse_from_str(date_string, date_format);
        result.unwrap().into()
    }
}
