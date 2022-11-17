pub struct Person {
    name: Option<String>,
    date_of_birth: Option<String>,
    books: Vec<usize>,
}
impl PartialEq for Person {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}
