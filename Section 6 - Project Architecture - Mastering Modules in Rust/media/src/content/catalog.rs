// super is a reference to the parent module.
use super::media::Media;

#[derive(Debug)]
pub struct Catalog {
    items: Vec<Media>
}

impl Catalog {
    pub fn new() -> Self {
        Catalog { items: vec![] }
    }

    pub fn add(&mut self, media: Media) {
        self.items.push(media);
    }

    /*fn get_by_index(&self, index: usize) -> MightHaveAValue {
        if self.items.len() > index {
            // Good! we have something to return
            MightHaveAValue::ThereIsAValue(&self.items[index])
        } else {
            // Bad! we don't have anything to return
            MightHaveAValue::NoValueAvailable
        }
    }*/
    pub fn get_by_index(&self, index: usize) -> Option<&Media> {
        if self.items.len() > index {
            Some(&self.items[index])
        } else {
            None
        }
    }
}