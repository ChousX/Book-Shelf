use crate::share::Id;

pub struct Publisher{
    name: Option<String>,
    books: Option<Vec<Id>>,
}