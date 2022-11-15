use crate::share::Id;

pub struct Publisher{
    name: Option<String>,
    books: Option<Vec<Id>>,
}
#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn basic(){

    }
}