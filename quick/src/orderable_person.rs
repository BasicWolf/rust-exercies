use std::cmp::Ordering;

pub struct OrderablePerson {
    pub name: String,
    pub age: u8,
}

impl Ord for OrderablePerson {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.age == other.age {
            Ordering::Equal
        } else if self.age > other.age {
            Ordering::Greater
        } else {
            Ordering::Less
        }
    }
}

impl PartialOrd for OrderablePerson {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for OrderablePerson {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.age == other.age
    }
}

impl Eq for OrderablePerson {}
