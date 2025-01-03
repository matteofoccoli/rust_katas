use crate::{Contact, Repository};
use std::fs::read_to_string;

pub struct FileRepository {
    file_path: String,
}

impl Repository for FileRepository {
    fn load(&self) -> Result<Option<Vec<Contact>>, String> {
        let result = read_to_string(self.file_path.clone())
            .map_err(|_| "Error reading file".to_string())?
            .lines()
            .into_iter()
            .skip(1)
            .map(|line| self.create_contact(line))
            .collect();
        return Ok(Some(result));
    }
}

impl FileRepository {
    fn create_contact(&self, line: &str) -> Contact {
        let fragments: Vec<&str> = line.split(',').collect();
        Contact {
            first_name: fragments[1].trim().to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fails_miserably() {
        let repo = FileRepository {
            file_path: "contacts.txt".to_string(),
        };

        let result = repo.load();

        let contacts = result.unwrap().unwrap();
        assert_eq!(contacts.len(), 2);
        assert_eq!(
            contacts[0],
            Contact {
                first_name: "John".to_string()
            }
        );
        assert_eq!(
            contacts[1],
            Contact {
                first_name: "Mary".to_string()
            }
        );
    }
}
