// TODO: add the necessary `Clone` implementations (and invocations)
//  to get the code to compile.

// MY_NOTE: You could improve `summary(self)`'s implimention:
// MY_NOTE: pub fn summary(&self) -> Summary {
// MY_NOTE:     Summary {
// MY_NOTE:         title: self.title.clone(),
// MY_NOTE:         status: self.status.clone(),
// MY_NOTE:     }
// MY_NOTE: }
// MY_NOTE: and then you can write like:
// MY_NOTE: pub fn summary(ticket: Ticket) -> (Ticket, Summary) {
// MY_NOTE:     let summary = ticket.summary();
// MY_NOTE:     (ticket, summary)
// MY_NOTE: }
pub fn summary(ticket: Ticket) -> (Ticket, Summary) {
    (ticket.clone(), ticket.summary())
}

pub struct Ticket {
    pub title: String,
    pub description: String,
    pub status: String,
}

impl Ticket {
    pub fn summary(self) -> Summary {
        Summary {
            title: self.title,
            status: self.status,
        }
    }
}

impl Clone for Ticket {
    fn clone(&self) -> Self {
        Ticket {
            title: self.title.clone(),
            description: self.description.clone(),
            status: self.status.clone(),
        }
    }
}

pub struct Summary {
    pub title: String,
    pub status: String,
}
