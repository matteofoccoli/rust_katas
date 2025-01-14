pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct Sms {
    origin: String,
    text: String,
}

impl Summary for Sms {
    fn summarize(&self) -> String {
        format!("sms from {} saying {}", self.origin, self.text).to_string()
    }
}

pub struct Email {
    sender: String,
    content: String,
}

impl Summary for Email {
    fn summarize(&self) -> String {
        format!("email from {} saying {}", self.sender, self.content).to_string()
    }
}

pub fn print_message(message: impl Summary) -> String {
    format!("You received: {}", message.summarize())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn print_messages() {
        let sms = Sms {
            origin: "12345".to_string(),
            text: "hello world".to_string(),
        };
        let email = Email {
            sender: "foo@bar.com".to_string(),
            content: "this is the content of the email".to_string(),
        };

        assert_eq!(
            "You received: sms from 12345 saying hello world".to_string(),
            print_message(sms)
        );

        assert_eq!(
            "You received: email from foo@bar.com saying this is the content of the email"
                .to_string(),
            print_message(email)
        );
    }
}
