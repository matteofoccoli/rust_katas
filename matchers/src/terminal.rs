pub enum Color {
    Rgb(i32, i32, i32),
    Hex(String),
}

pub enum Message {
    Resize { width: i32, height: i32 },
    Print(String),
    SetBackground(Color),
}

pub struct Terminal;

impl Terminal {
    pub fn receive(&self, message: Message) -> String {
        match message {
            Message::Print(text) => format!("Terminal printed: {text}"),
            Message::SetBackground(Color::Rgb(r, g, b)) => {
                format!("Terminal set background color to: {r}, {g}, {b}")
            }
            Message::SetBackground(Color::Hex(hex)) => {
                format!("Terminal set background color to: #{hex}")
            }
            Message::Resize { width, height } => {
                format!("Terminal resized to (width, height): {width}px, {height}px")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn terminal_reacts_to_messages() {
        let terminal = Terminal;

        assert_eq!(
            "Terminal printed: Hello world",
            terminal.receive(Message::Print("Hello world".to_owned()))
        );

        assert_eq!(
            "Terminal set background color to: 168, 149, 50",
            terminal.receive(Message::SetBackground(Color::Rgb(168, 149, 50)))
        );

        assert_eq!(
            "Terminal set background color to: #a89532",
            terminal.receive(Message::SetBackground(Color::Hex("a89532".to_string())))
        );

        assert_eq!(
            "Terminal resized to (width, height): 100px, 200px",
            terminal.receive(Message::Resize {
                width: 100,
                height: 200
            })
        );
    }
}
