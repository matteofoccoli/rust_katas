use chrono::{DateTime, Datelike, Utc};

pub struct Account {
    transactions: Vec<Transaction>,
}

pub struct Transaction {
    timestamp: DateTime<Utc>,
    amount: f32,
}

const AMOUNT_ERROR: &str = "Amount must be positive";

impl Account {
    pub fn new() -> Self {
        Self {
            transactions: vec![],
        }
    }

    pub fn deposit(&mut self, amount: f32, timestamp: DateTime<Utc>) -> Result<(), String> {
        if amount < 0.0 {
            return Err(AMOUNT_ERROR.to_string());
        }
        self.transactions.push(Transaction { timestamp, amount });
        self.sort_transaction_by_timestamp_asc();
        Ok(())
    }

    pub fn withdraw(&mut self, amount: f32, timestamp: DateTime<Utc>) -> Result<(), String> {
        if amount < 0.0 {
            return Err(AMOUNT_ERROR.to_string());
        }
        self.transactions.push(Transaction {
            timestamp,
            amount: -amount,
        });
        self.sort_transaction_by_timestamp_asc();
        Ok(())
    }

    pub fn print_statement(&self) -> String {
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
        format!(
            "\n{}   {}{: <8} {}{}",
            self.format_date(transaction.timestamp),
            self.get_sign(transaction.amount),
            transaction.amount.abs(),
            self.get_sign(current_balance),
            current_balance.abs()
        )
    }

    fn get_sign(&self, current_balance: f32) -> char {
        if current_balance < 0.0 {
            '-'
        } else {
            '+'
        }
    }

    fn format_date(&self, date: DateTime<Utc>) -> String {
        if date.month() < 10 {
            format!("{} ", date.format("%d.%-m.%Y"))
        } else {
            format!("{}", date.format("%d.%-m.%Y"))
        }
    }

    fn sort_transaction_by_timestamp_asc(&mut self) {
        self.transactions
            .sort_by(|a, b| a.timestamp.cmp(&b.timestamp));
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
        account.deposit(500.0, parse_date("2015-12-24")).unwrap();

        let statement = account.print_statement();

        let expected_statement = "Date        Amount  Balance\n\
            24.12.2015   +500      +500"
            .to_string();
        assert_eq!(expected_statement, statement);
    }

    #[test]
    fn accepts_only_positive_numbers_in_deposit() {
        let mut account = Account::new();

        let result = account.deposit(-500.0, parse_date("2015-12-24"));

        assert!(matches!(result, Err(message) if message == AMOUNT_ERROR.to_string()))
    }

    #[test]
    fn accepts_only_positive_numbers_in_withdraw() {
        let mut account = Account::new();

        let result = account.withdraw(-500.0, parse_date("2015-12-24"));

        assert!(matches!(result, Err(message) if message == AMOUNT_ERROR.to_string()))
    }

    #[test]
    fn prints_statement_with_two_deposits() {
        let mut account = Account::new();
        account.deposit(10.0, parse_date("2015-12-22")).unwrap();
        account.deposit(20.0, parse_date("2015-12-24")).unwrap();

        let statement = account.print_statement();

        let expected_statement = "Date        Amount  Balance\n\
            22.12.2015   +10       +10\n\
            24.12.2015   +20       +30"
            .to_string();
        assert_eq!(expected_statement, statement);
    }

    #[test]
    fn prints_statement_with_one_deposit_and_one_withdraw() {
        let mut account = Account::new();
        account.deposit(500.0, parse_date("2015-12-24")).unwrap();
        account.withdraw(100.0, parse_date("2016-08-23")).unwrap();

        let statement = account.print_statement();

        let expected_statement = "Date        Amount  Balance\n\
            24.12.2015   +500      +500\n\
            23.8.2016    -100      +400"
            .to_string();
        assert_eq!(expected_statement, statement);
    }

    #[test]
    fn prints_statement_with_transaction_ordered_by_timestamp_ascending() {
        let mut account = Account::new();
        account
            .deposit(200.0, parse_date_with_time("2018-12-24", "13:00:00"))
            .unwrap();
        account
            .deposit(100.0, parse_date_with_time("2017-11-23", "01:00:00"))
            .unwrap();
        account
            .deposit(500.0, parse_date_with_time("2018-12-24", "10:00:00"))
            .unwrap();

        let statement = account.print_statement();

        let expected_statement = "Date        Amount  Balance\n\
            23.11.2017   +100      +100\n\
            24.12.2018   +500      +600\n\
            24.12.2018   +200      +800"
            .to_string();
        assert_eq!(expected_statement, statement);
    }

    #[test]
    fn pretty_prints() {
        let mut account = Account::new();
        account.deposit(500.0, parse_date("2018-12-24")).unwrap();
        account.deposit(20.5, parse_date("2019-01-06")).unwrap();
        account.withdraw(100.0, parse_date("2017-11-23")).unwrap();
        account.withdraw(10.0, parse_date("2018-12-27")).unwrap();

        let statement = account.print_statement();

        let expected_statement = "Date        Amount  Balance\n\
            23.11.2017   -100      -100\n\
            24.12.2018   +500      +400\n\
            27.12.2018   -10       +390\n\
            06.1.2019    +20.5     +410.5"
            .to_string();
        assert_eq!(expected_statement, statement);
    }

    fn parse_date(date_string: &str) -> DateTime<Utc> {
        let date_string = &format!("{} 00:00:00.000 +0000", date_string);
        let date_format = "%Y-%m-%d %H:%M:%S%.3f %z";
        let result = DateTime::parse_from_str(date_string, date_format);
        result.unwrap().into()
    }

    fn parse_date_with_time(date_string: &str, time_string: &str) -> DateTime<Utc> {
        let date_string = &format!("{} {}.000 +0000", date_string, time_string);
        let date_format = "%Y-%m-%d %H:%M:%S%.3f %z";
        let result = DateTime::parse_from_str(date_string, date_format);
        result.unwrap().into()
    }
}
