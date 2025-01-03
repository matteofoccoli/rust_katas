use mockall::automock;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[automock]
pub trait Sender {
    fn send(&self) -> Result<(), String>;
}

pub struct Greeter {
    sender: Box<dyn Sender>,
}

impl Greeter {
    pub fn new(sender: Box<dyn Sender>) -> Self {
        Self { sender }
    }

    pub fn send_greetings(&self) -> Result<(), String> {
        self.sender.send()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn successfully_greets() {
        let mut sender = MockSender::new();
        sender.expect_send().once().returning(|| Ok(()));

        let greeter = Greeter::new(Box::new(sender));
        let result = greeter.send_greetings();

        assert_eq!(result, Ok(()));
    }

    #[test]
    fn handles_error_when_sending() {
        let mut sender = MockSender::new();
        sender
            .expect_send()
            .once()
            .returning(|| Err("Error sending".to_string()));

        let greeter = Greeter::new(Box::new(sender));
        let result = greeter.send_greetings();

        assert_eq!(result, Err("Error sending".to_string()));
    }
}
