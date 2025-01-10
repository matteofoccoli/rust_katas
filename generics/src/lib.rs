pub fn largest_i32(numbers: &[i32]) -> &i32 {
    let mut largest = &numbers[0];
    for number in numbers {
        if number > largest {
            largest = number;
        }
    }
    largest
}

pub fn largest_char(chars: &[char]) -> &char {
    let mut largest = &chars[0];
    for char in chars {
        if char > largest {
            largest = char;
        }
    }
    largest
}

pub fn largest<T: std::cmp::PartialOrd>(items: &[T]) -> &T {
    let mut largest = &items[0];
    for item in items {
        if item > largest {
            largest = item;
        }
    }
    largest
}

pub struct Point<T> {
    pub x: T,
    pub y: T,
}

impl<T: std::fmt::Display> Point<T> {
    pub fn print(&self) -> String {
        format!("x: {}, y: {}", self.x, self.y)
    }
}

impl Point<f32> {
    // This method is available only for points with f32 generic type
    pub fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_largest_i32() {
        let numbers = [12, 23, 1, 45, 34, 11];

        let largest = largest_i32(&numbers);

        assert_eq!(45, *largest);
    }

    #[test]
    fn get_largest_char() {
        let chars = ['a', 'z', 'd', 'f', 'c'];

        let largest = largest_char(&chars);

        assert_eq!('z', *largest);
    }

    #[test]
    fn get_largest_with_generics() {
        let numbers = [12, 23, 1, 45, 34, 11];
        let chars = ['a', 'z', 'd', 'f', 'c'];

        assert_eq!(45, *largest(&numbers));
        assert_eq!('z', *largest(&chars));
    }

    #[test]
    fn create_point_with_different_types() {
        let int_point = Point { x: 1, y: 2 };
        let float_point = Point { x: 1.12, y: 2.23 };

        assert_eq!(1, int_point.x);
        assert_eq!(2, int_point.y);
        assert_eq!(1.12, float_point.x);
        assert_eq!(2.23, float_point.y);
    }

    #[test]
    fn print_point_content() {
        let int_point = Point { x: 1, y: 2 };
        let float_point = Point { x: 1.12, y: 2.23 };

        assert_eq!("x: 1, y: 2".to_string(), int_point.print());
        assert_eq!("x: 1.12, y: 2.23".to_string(), float_point.print());
    }

    #[test]
    fn calculates_distance_from_center_for_f32_points() {
        let point = Point { x: 1.0, y: 1.0 };

        assert_eq!(1.4142135, point.distance_from_origin())
    }
}
