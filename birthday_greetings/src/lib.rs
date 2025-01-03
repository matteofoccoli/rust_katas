use chrono::{Datelike, NaiveDate};
use mockall::automock;

mod file_repository;

#[derive(Debug, PartialEq, Clone)]
pub struct Contact {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub date_of_birth: NaiveDate,
}

#[automock]
pub trait Sender {
    fn send(&self, contact: Contact) -> Result<(), String>;
}

#[automock]
pub trait Repository {
    fn load(&self) -> Result<Option<Vec<Contact>>, String>;
}

pub struct Greeter {
    sender: Box<dyn Sender>,
    repository: Box<dyn Repository>,
    today: NaiveDate,
}

impl Greeter {
    pub fn new(sender: Box<dyn Sender>, repository: Box<dyn Repository>, today: NaiveDate) -> Self {
        Self {
            sender,
            repository,
            today,
        }
    }

    pub fn send_greetings(&self) -> Result<String, String> {
        let contacts = self.repository.load()?;
        let mut sent_messages = 0;
        if let Some(contacts) = contacts {
            for contact in contacts {
                if self.today_is_their_birthday(&contact) {
                    match self.sender.send(contact) {
                        Ok(_) => sent_messages += 1,
                        Err(e) => return Err(e),
                    }
                }
            }
        }
        Ok(format!("Sent {sent_messages} messages"))
    }

    fn today_is_their_birthday(&self, contact: &Contact) -> bool {
        contact.date_of_birth.day0() == self.today.day0()
            && contact.date_of_birth.month0() == self.today.month0()
    }
}

#[cfg(test)]
mod tests {
    use mockall::predicate::eq;

    use super::*;

    #[test]
    fn successfully_greets_on_date_of_birth() {
        let contacts = contacts();

        let mut sender = MockSender::new();
        sender
            .expect_send()
            .with(eq(contacts[0].clone()))
            .once()
            .returning(|_| Ok(()));

        let mut repository = MockRepository::new();
        repository
            .expect_load()
            .once()
            .return_once(|| Ok(Some(contacts)));

        let greeter = Greeter::new(Box::new(sender), Box::new(repository), today());
        let result = greeter.send_greetings();

        assert_eq!(result, Ok("Sent 1 messages".to_string()));
    }

    fn today() -> NaiveDate {
        NaiveDate::parse_from_str("2025/10/08", "%Y/%m/%d").unwrap()
    }

    #[test]
    fn handles_error_when_sending() {
        let mut sender = MockSender::new();
        sender
            .expect_send()
            .once()
            .returning(|_| Err("Error sending".to_string()));

        let mut repository = MockRepository::new();
        repository
            .expect_load()
            .once()
            .return_once(|| Ok(Some(contacts())));

        let greeter = Greeter::new(Box::new(sender), Box::new(repository), today());
        let result = greeter.send_greetings();

        assert_eq!(result, Err("Error sending".to_string()));
    }

    #[test]
    fn handles_error_when_loading_contacts() {
        let mut sender = MockSender::new();
        sender.expect_send().never();

        let mut repository = MockRepository::new();
        repository
            .expect_load()
            .once()
            .return_once(|| Err("Error loading contacts".to_string()));

        let greeter = Greeter::new(Box::new(sender), Box::new(repository), today());
        let result = greeter.send_greetings();

        assert_eq!(result, Err("Error loading contacts".to_string()));
    }

    #[test]
    fn does_not_send_greeting_if_contacts_are_empty() {
        let mut sender = MockSender::new();
        sender.expect_send().never();

        let mut repository = MockRepository::new();
        repository.expect_load().once().return_once(|| Ok(None));

        let greeter = Greeter::new(Box::new(sender), Box::new(repository), today());
        let result = greeter.send_greetings();

        assert_eq!(result, Ok("Sent 0 messages".to_string()));
    }

    fn contacts() -> Vec<Contact> {
        vec![
            Contact {
                first_name: "Matteo".to_string(),
                last_name: "Foo".to_string(),
                email: "matteo@test.com".to_string(),
                date_of_birth: NaiveDate::parse_from_str("1982/10/08", "%Y/%m/%d").unwrap(),
            },
            Contact {
                first_name: "Lily".to_string(),
                last_name: "Bar".to_string(),
                email: "lily@test.com".to_string(),
                date_of_birth: NaiveDate::parse_from_str("1943/11/22", "%Y/%m/%d").unwrap(),
            },
        ]
    }
}
