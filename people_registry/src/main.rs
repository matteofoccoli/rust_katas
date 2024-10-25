use std::fmt::Display;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

pub fn main() {}

struct PeopleRegistry {
    errors: Vec<Error>,
}

#[derive(Clone, PartialEq, Debug)]
struct Error {
    line_number: usize,
    message: String,
}

impl PeopleRegistry {
    fn new() -> Self {
        Self { errors: vec![] }
    }

    fn errors(&self) -> &Vec<Error> {
        &self.errors
    }

    fn read(&mut self, file_path: PathBuf) -> Result<Vec<Person>, String> {
        self.errors = vec![];
        let mut result: Vec<Person> = Vec::new();
        let mut file = File::open(file_path).map_err(|e| e.to_string())?;
        let mut file_content = String::new();
        if let Err(e) = file.read_to_string(&mut file_content) {
            return Err(e.to_string());
        }
        for (line_index, line_content) in file_content.lines().enumerate() {
            match Self::parse_line(line_content) {
                Ok(person) => result.push(person),
                Err(e) => self.errors.push(Error {
                    line_number: line_index + 1,
                    message: e.to_string(),
                }),
            }
        }

        return Ok(result);
    }

    fn write(&self, people: Vec<Person>, file_path: PathBuf) -> Result<(), String> {
        let file_path_as_a_string = file_path.display().to_string();
        let mut file = File::create(file_path).map_err(|_| format!("Cannot create file '{}'", file_path_as_a_string))?;
        for person in people {
            let _ = file.write_fmt(format_args!("{};{};{};\n", person.name, person.genre, person.age));
        }
        Ok(())
    }

    fn parse_line(line: &str) -> Result<Person, String> {
        let parts: Vec<&str> = line.split(';').collect();
        if parts.len() < 3 {
            return Err("Invalid line".to_string());
        }
        let name = parts[0].to_string();
        let genre = parts[1].try_into()?;
        let age = parts[2].parse().map_err(|_| "Invalid age")?;
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
            _ => Err("Invalid genre"),
        }
    }
}

impl Display for Genre {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Genre::Man => write!(f, "m"),
            Genre::Woman => write!(f, "w"),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{env, time::{SystemTime, UNIX_EPOCH}};

    use super::*;

    #[test]
    fn handles_not_existing_file() {
        let mut registry = PeopleRegistry::new();

        let result = registry.read(PathBuf::from("./not_existing.txt"));

        assert!(result.is_err())
    }

    #[test]
    fn reads_one_person_from_a_file() {
        let mut registry = PeopleRegistry::new();

        let people = registry
            .read(PathBuf::from("./person.txt"))
            .expect("Unexpected error reading file in tests");

        assert_eq!(1, people.len());
        assert_eq!(
            Person {
                name: "Gigi".to_string(),
                genre: Genre::Man,
                age: 11
            },
            people[0]
        )
    }

    #[test]
    fn reads_two_people_from_a_file() {
        let mut registry = PeopleRegistry::new();

        let people = registry
            .read(PathBuf::from("./people.txt"))
            .expect("Unexpected error reading file in tests");

        assert_eq!(2, people.len());
        assert_eq!(
            Person {
                name: "Matteo".to_string(),
                genre: Genre::Man,
                age: 22
            },
            people[0]
        );
        assert_eq!(
            Person {
                name: "Melissa".to_string(),
                genre: Genre::Woman,
                age: 33
            },
            people[1]
        );
    }

    #[test]
    fn skips_lines_with_errors() {
        let mut registry = PeopleRegistry::new();

        let people = registry
            .read(PathBuf::from("./with_errors.txt"))
            .expect("Unexpected error reading file in tests");

        assert_eq!(1, people.len());
        assert_eq!(
            Person {
                name: "Melissa".to_string(),
                genre: Genre::Woman,
                age: 33
            },
            people[0]
        );
    }

    #[test]
    fn gets_errors() {
        let mut registry = PeopleRegistry::new();

        let _ = registry
            .read(PathBuf::from("./with_errors.txt"))
            .expect("Unexpected error reading file in tests");
        let errors = registry.errors();

        assert_eq!(3, errors.len());
        assert_eq!(
            Error {
                line_number: 1,
                message: "Invalid genre".to_string()
            },
            errors[0]
        );
        assert_eq!(
            Error {
                line_number: 3,
                message: "Invalid age".to_string()
            },
            errors[1]
        );
        assert_eq!(
            Error {
                line_number: 4,
                message: "Invalid line".to_string()
            },
            errors[2]
        );
    }

    #[test]
    fn reinitialize_errors_when_reading_a_new_file() {
        let mut registry = PeopleRegistry::new();

        let file_path = "./with_errors.txt";
        let _ = registry
            .read(PathBuf::from(file_path))
            .expect("Unexpected error reading file in tests");
        let _ = registry
            .read(PathBuf::from(file_path))
            .expect("Unexpected error reading file in tests");

        assert_eq!(3, registry.errors.len())
    }

    #[test]
    fn writes_people_to_a_file() {
        let mut registry = PeopleRegistry::new();
        let people = vec![
            Person {
                name: "John".to_string(),
                genre: Genre::Man,
                age: 12,
            },
            Person {
                name: "Mary".to_string(),
                genre: Genre::Woman,
                age: 23,
            }
        ];
        let file_path = &format!("{}people_repository_kata_{}.txt", env::temp_dir().display(), current_timestamp());

        let write_result = registry.write(people, PathBuf::from(file_path));

        assert!(write_result.is_ok());
        let read_result = registry.read(PathBuf::from(file_path)).expect("Unexpected error reading file in tests");
        assert_eq!(2, read_result.len());
        assert_eq!(Person {name: "John".to_string(), genre: Genre::Man, age: 12}, read_result[0]);
        assert_eq!(Person {name: "Mary".to_string(), genre: Genre::Woman, age: 23}, read_result[1])
    }

    #[test]
    fn writes_fails_if_parent_dir_does_not_exist() {
        let registry = PeopleRegistry::new();
        let not_existing_file_path = &format!("./{}/people.txt", current_timestamp());

        let result = registry.write(vec!(), PathBuf::from(not_existing_file_path));

        assert!(result.is_err());
        assert_eq!(format!("Cannot create file '{}'", not_existing_file_path), result.unwrap_err());
    }

    fn current_timestamp() -> u32 {
        SystemTime::now().duration_since(UNIX_EPOCH).unwrap().subsec_nanos()
    }
    
}
