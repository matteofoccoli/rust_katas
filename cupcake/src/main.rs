fn main() {
    println!("Hello, world!");
}

struct Cupcake {

}

impl Cupcake {
    fn print(&self) -> String {
        "🧁".to_string()
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn creats_a_plain_cupcake() {
        let cupcake = Cupcake {};

        let result = cupcake.print();

        assert_eq!("🧁", result);
    }

}
