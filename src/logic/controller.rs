use std::collections::HashMap;
/// Primitive for KV storage.
/// If we're trying to make hard version of this project - we should change this struct
#[derive(Default)]
pub struct Controller {
    data: HashMap<String, String>,
}

impl Controller {
    /// Inserts (key, value) in storage
    pub fn insert(&mut self, key: String, value: String) {
        self.data.insert(key, value);
    }

    /// Returns size of storage
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Gets value by key
    pub fn get(&self, key: &String) -> Option<String> {
        self.data.get(key).cloned()
    }
}

#[cfg(test)]
mod test {
    use crate::Controller;

    #[test]
    pub fn it_works() {
        let mut controller = Controller::default();
        assert_eq!(controller.len(), 0);

        controller.insert("a".parse().unwrap(), "b".parse().unwrap());
        assert_eq!(controller.len(), 1);
        assert_eq!(
            controller.get(&"a".parse().unwrap()),
            Some("b".parse().unwrap())
        );

        controller.insert("a".parse().unwrap(), "a".parse().unwrap());
        assert_eq!(controller.len(), 1);
        assert_eq!(
            controller.get(&"a".parse().unwrap()),
            Some("a".parse().unwrap())
        );
        assert_eq!(controller.get(&"b".parse().unwrap()), None);
    }
}
