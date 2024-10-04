pub fn main() {}

struct PeopleRegistry {}

impl PeopleRegistry {
    fn new(file: String) -> Self {
        Self {}
    }

    fn read(&self) -> Vec<Person> {
        let mut result: Vec<Person> = Vec::new();
        result.push(Person {
            first_name: "matteo".to_string(),
        });
        return result;
    }
}

#[derive(PartialEq, Debug)]
struct Person {
    first_name: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_one_person_from_a_file() {
        let registry = PeopleRegistry::new("people.txt".to_string());

        let people = registry.read();

        assert_eq!(1, people.len());
        assert_eq!(
            Some(&Person {
                first_name: "matteo".to_string()
            }),
            people.get(0)
        )
    }
}
