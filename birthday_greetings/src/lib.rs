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

    pub fn send_greetings(&self) {
        self.sender.send();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greets() {
        let mut sender = MockSender::new();
        sender.expect_send().once().returning(|| Ok(()));

        let greeter = Greeter::new(Box::new(sender));

        greeter.send_greetings();
    }
}
