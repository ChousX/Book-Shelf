use crate::share::Id;
#[derive(Debug, Default)]
pub struct Publisher {
    pub works: Vec<Id>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {}
}
