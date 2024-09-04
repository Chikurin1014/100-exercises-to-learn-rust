// TODO: whenever `title` and `description` are returned via their accessor methods, they
//   should be normalized—i.e. leading and trailing whitespace should be removed.
//   There is a method in Rust's standard library that can help with this, but you won't
//   find it in the documentation for `String`.
//   Can you figure out where it is defined and how to use it?

#[allow(dead_code)]
pub struct Ticket {
    title: String,
    description: String,
    status: String,
}

impl Ticket {
    pub fn title(&self) -> &str {
        // MY_NOTE: The `deref` method, which is declaired in std::ops::Deref and implemented in alloc::string is called implicitly
        // MY_NOTE: The code is equivalent to:
        // MY_NOTE: use std::ops::Deref;
        // MY_NOTE: &self.title.deref().trim()
        &self.title.trim()
    }

    pub fn description(&self) -> &str {
        // MY_NOTE: The `deref` method, which is declaired in std::ops::Deref and implemented in alloc::string is called implicitly
        // MY_NOTE: The code is equivalent to:
        // MY_NOTE: use std::ops::Deref;
        // MY_NOTE: &self.title.deref().trim()
        &self.description.trim()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalization() {
        let ticket = Ticket {
            title: "   A title ".to_string(),
            description: " A description   ".to_string(),
            status: "To-Do".to_string(),
        };

        assert_eq!("A title", ticket.title());
        assert_eq!("A description", ticket.description());
    }
}
