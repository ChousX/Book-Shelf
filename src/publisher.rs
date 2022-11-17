use crate::share::Id;
pub struct Publisher {
    name: Option<String>,
    books: Option<Vec<Id>>,
}
impl PartialEq for Publisher {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {}
}
