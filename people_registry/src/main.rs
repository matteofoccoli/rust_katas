use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

pub fn main() {}

struct PeopleRegistry {
    file_path: PathBuf,
}

impl PeopleRegistry {
    fn new(file_path: PathBuf) -> Self {
        Self { file_path }
    }

    fn read(&self) -> Result<Vec<Person>, String> {
        let mut result: Vec<Person> = Vec::new();
        let mut file = File::open(self.file_path.clone()).map_err(|e| e.to_string())?;
        let mut file_content = String::new();
        if let Err(e) = file.read_to_string(&mut file_content) {
            return Err(e.to_string());
        }
        for line in file_content.lines() {
            if let Ok(line) = Self::parse_line(line) {
                result.push(line);
            }
        }

        return Ok(result);
    }

    fn parse_line(line: &str) -> Result<Person, String> {
        let parts: Vec<&str> = line.split(';').collect();
        let name = parts[0].to_string();
        let genre = parts[1].try_into()?;
        let age = parts[2].parse().map_err(|_| "Error parsing age")?;
        Ok(Person { name, genre, age })
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
    fn handles_not_existing_file() {
        let registry = PeopleRegistry::new(PathBuf::from("./not_existing.txt"));

        let result = registry.read();

        assert!(result.is_err())
    }

    #[test]
    fn reads_one_person_from_a_file() {
        let registry = PeopleRegistry::new(PathBuf::from("./person.txt"));

        let people = registry
            .read()
            .expect("Unexpected error reading file in tests");

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
        let registry = PeopleRegistry::new(PathBuf::from("./people.txt"));

        let people = registry
            .read()
            .expect("Unexpected error reading file in tests");

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

    #[test]
    fn skips_lines_with_errors() {
        let registry = PeopleRegistry::new(PathBuf::from("./with_errors.txt"));

        let people = registry
            .read()
            .expect("Unexpected error reading file in tests");

        assert_eq!(1, people.len());
        assert_eq!(
            Some(&Person {
                name: "Melissa".to_string(),
                genre: Genre::Woman,
                age: 33
            }),
            people.get(0)
        );
    }
}
