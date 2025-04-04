use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct Buddy {
    pub alias: String,
    pub name: String,
    pub email: String,
}

impl Buddy {
    pub fn format_co_author(&self) -> String {
        format!("Co-authored-by: {} <{}>", self.name, self.email)
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Buddies {
    pub buddies: Vec<Buddy>,
}

impl Buddies {
    pub fn new(buddies: Vec<Buddy>) -> Self {
        Buddies { buddies }
    }

    pub fn get(&self, alias: &str) -> Option<&Buddy> {
        self.buddies
            .iter()
            .find(|buddy| buddy.alias == alias)
            .map(|buddy| (buddy))
    }

    pub fn has(&self, alias: &str) -> bool {
        match self.get(alias) {
            Some(_) => true,
            None => false,
        }
    }

    pub fn get_buddy_by_email(&self, email: &str) -> Option<&Buddy> {
        self.buddies
            .iter()
            .find(|buddy| buddy.email == email)
            .map(|buddy| (buddy))
    }

    pub fn add(&mut self, buddy: Buddy) -> Result<()> {
        if self.has(&buddy.alias) {
            anyhow::bail!("Buddy with alias '{}' already exists", buddy.alias);
        }

        self.buddies.push(buddy);
        Ok(())
    }

    pub fn forget(&mut self, alias: &str) -> Result<()> {
        if let Some(index) = self.buddies.iter().position(|buddy| buddy.alias == alias) {
            self.buddies.swap_remove(index);
            return Ok(());
        }

        anyhow::bail!("Buddy with alias '{}' doesn't exist", alias);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_buddy_by_email() {
        let mut buddies = Buddies::default();
        let _ = buddies.add(Buddy {
            alias: "peter".to_string(),
            name: "Peter Pan".to_string(),
            email: "peter.pan@example.com".to_string(),
        });

        let result = buddies.get_buddy_by_email("peter.pan@example.com");
        assert!(result.is_some());
        let buddy = result.unwrap();
        assert_eq!(buddy.alias, "peter");
        assert_eq!(buddy.name, "Peter Pan");

        // Non-existent email
        //
        let result = buddies.get_buddy_by_email("captain.hook@example.com");
        assert!(result.is_none());
    }

    #[test]
    fn test_format_co_author() {
        let buddy = Buddy {
            alias: "peter".to_string(),
            name: "Peter Pan".to_string(),
            email: "peter.pan@example.com".to_string(),
        };

        let co_author = buddy.format_co_author();
        assert_eq!(
            co_author,
            "Co-authored-by: Peter Pan <peter.pan@example.com>".to_string()
        );
    }
}
