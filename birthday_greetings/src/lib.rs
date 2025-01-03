use mockall::automock;

mod file_repository;

#[derive(Debug, PartialEq)]
pub struct Contact {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
}

#[automock]
pub trait Sender {
    fn send(&self, contacts: Vec<Contact>) -> Result<usize, String>;
}

#[automock]
pub trait Repository {
    fn load(&self) -> Result<Option<Vec<Contact>>, String>;
}

pub struct Greeter {
    sender: Box<dyn Sender>,
    repository: Box<dyn Repository>,
}

impl Greeter {
    pub fn new(sender: Box<dyn Sender>, repository: Box<dyn Repository>) -> Self {
        Self { sender, repository }
    }

    pub fn send_greetings(&self) -> Result<String, String> {
        let contacts = self.repository.load()?;
        let mut sent_messages = 0;
        if let Some(contacts) = contacts {
            sent_messages = self.sender.send(contacts)?;
        }
        Ok(format!("Sent {sent_messages} message"))
    }
}

#[cfg(test)]
mod tests {
    use mockall::predicate::eq;

    use super::*;

    #[test]
    fn successfully_greets() {
        let mut sender = MockSender::new();
        sender
            .expect_send()
            .with(eq(contacts()))
            .once()
            .returning(|_| Ok(contacts().len()));

        let mut repository = MockRepository::new();
        repository
            .expect_load()
            .once()
            .return_once(|| Ok(Some(contacts())));

        let greeter = Greeter::new(Box::new(sender), Box::new(repository));
        let result = greeter.send_greetings();

        assert_eq!(result, Ok("Sent 2 message".to_string()));
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

        let greeter = Greeter::new(Box::new(sender), Box::new(repository));
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

        let greeter = Greeter::new(Box::new(sender), Box::new(repository));
        let result = greeter.send_greetings();

        assert_eq!(result, Err("Error loading contacts".to_string()));
    }

    #[test]
    fn does_not_send_greeting_if_contacts_are_empty() {
        let mut sender = MockSender::new();
        sender.expect_send().never();

        let mut repository = MockRepository::new();
        repository.expect_load().once().return_once(|| Ok(None));

        let greeter = Greeter::new(Box::new(sender), Box::new(repository));
        let result = greeter.send_greetings();

        assert_eq!(result, Ok("Sent 0 message".to_string()));
    }

    fn contacts() -> Vec<Contact> {
        vec![
            Contact {
                first_name: "Matteo".to_string(),
                last_name: "Foo".to_string(),
                email: "matteo@test.com".to_string(),
            },
            Contact {
                first_name: "Lily".to_string(),
                last_name: "Bar".to_string(),
                email: "lily@test.com".to_string(),
            },
        ]
    }
}
