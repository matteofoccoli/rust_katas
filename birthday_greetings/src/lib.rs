use mockall::automock;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[automock]
pub trait Sender {
    fn send(&self) -> Result<(), String>;
}

pub struct Contact {
    pub first_name: String,
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

    pub fn send_greetings(&self) -> Result<(), String> {
        self.repository.load();
        self.sender.send()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn successfully_greets() {
        let contacts = create_contacts();

        let mut sender = MockSender::new();
        sender.expect_send().once().returning(|| Ok(()));

        let mut repository = MockRepository::new();
        repository
            .expect_load()
            .once()
            .return_once(|| Ok(Some(contacts)));

        let greeter = Greeter::new(Box::new(sender), Box::new(repository));
        let result = greeter.send_greetings();

        assert_eq!(result, Ok(()));
    }

    fn create_contacts() -> Vec<Contact> {
        vec![Contact {
            first_name: "Matteo".to_string(),
        }]
    }

    #[test]
    #[ignore]
    fn handles_error_when_sending() {
        let contacts = create_contacts();

        let mut sender = MockSender::new();
        sender
            .expect_send()
            .once()
            .returning(|| Err("Error sending".to_string()));

        let mut repository = MockRepository::new();
        repository
            .expect_load()
            .once()
            .return_once(|| Ok(Some(contacts)));

        let greeter = Greeter::new(Box::new(sender), Box::new(repository));
        let result = greeter.send_greetings();

        assert_eq!(result, Err("Error sending".to_string()));
    }
}
