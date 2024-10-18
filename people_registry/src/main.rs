use std::fs::File;
use std::io::prelude::*;

pub fn main() {}

struct PeopleRegistry {
    file: String,
}

impl PeopleRegistry {
    fn new(file: String) -> Self {
        Self { file }
    }

    fn read(&self) -> Vec<Person> {
        let mut result: Vec<Person> = Vec::new();
        let mut file = File::open(self.file.clone()).expect("Error opening file");
        let mut file_content = String::new();
        file.read_to_string(&mut file_content)
            .expect("Error reading file content");
        for line in file_content.lines() {
            println!(">>>>> {line}");
            let parts: Vec<&str> = line.split(';').collect();
            result.push(Person {
                name: parts[0].to_string(),
                genre: parts[1].try_into().expect("Invalid genre"),
                age: parts[2].parse().expect("Invalid age"),
            })
        }

        return result;
    }
}

#[derive(PartialEq, Debug)]
struct Person {
    name: String,
    genre: Genre,
    age: u8,
}

#[derive(PartialEq, Debug)]
enum Genre {
    Man,
    Woman,
}

impl TryFrom<&str> for Genre {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "m" => Ok(Genre::Man),
            "w" => Ok(Genre::Woman),
            _ => Err("Invalid value for genre"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_one_person_from_a_file() {
        let registry = PeopleRegistry::new("person.txt".to_string());

        let people = registry.read();

        assert_eq!(1, people.len());
        assert_eq!(
            Some(&Person {
                name: "Gigi".to_string(),
                genre: Genre::Man,
                age: 11
            }),
            people.get(0)
        )
    }

    #[test]
    fn reads_two_people_from_a_file() {
        let registry = PeopleRegistry::new("people.txt".to_string());

        let people = registry.read();

        assert_eq!(2, people.len());
        assert_eq!(
            Some(&Person {
                name: "Matteo".to_string(),
                genre: Genre::Man,
                age: 22
            }),
            people.get(0)
        );
        assert_eq!(
            Some(&Person {
                name: "Melissa".to_string(),
                genre: Genre::Woman,
                age: 33
            }),
            people.get(1)
        );
    }
}
