use crate::share::Id;
#[derive(Debug, Default)]
pub struct Publisher {
    books: Option<Vec<Id>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {}
}
