pub struct Series {
    title: Option<String>,
    book_ids: Vec<usize>,
}

impl PartialEq for Series {
    fn eq(&self, other: &Self) -> bool {
        self.title == other.title
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {}
}
