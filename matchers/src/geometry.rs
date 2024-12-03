pub struct Point {
    x: i32,
    y: i32,
}

pub fn x_falls_in_range(p: &Point) -> String {
    match p {
        Point { x: 0..=100, y } => "Falls in range".to_string(),
        _ => "Doesn't fall in range".to_string(),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn fail_miserably() {
        assert_eq!(
            "Falls in range".to_string(),
            x_falls_in_range(&Point { x: 12, y: 45 })
        );
        assert_eq!(
            "Doesn't fall in range".to_string(),
            x_falls_in_range(&Point { x: 120, y: 45 })
        );
    }
}
