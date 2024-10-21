#[derive(Debug)]
enum Media {
    Book { title: String, author: String},
    Movie { title: String, director: String },
    Audiobook { title: String },
    // Podcast { episode_number: u32 }, // if the name that you want to write is to long, you can replace with the following sentence
    Podcast(u32),
    Placeholder,
}

impl Media {
    fn description(&self) -> String {
        // is self a Book?
        // is self a Movie?
        // is self a Audiobook?
        // first you need to figure out what type of media we are working to access to any prop

        // One way to check enum types "Verbose"
        /*if let Media::Book { title, author } = self {
            format!("Book: {} {}", title, author)
        } else if let Media::Movie { title, director } = self {
            format!("Movie: {} {}", title, director)
        } else if let Media::Audiobook { title } = self {
            format!("Audiobook: {}", title)
        } else {
            String::from("Media description")
        }*/

        // Pattern Matching
        // you have to create a match for each type of the enum
        match self {
            Media::Book { title, author } => {
                format!("Book: {} {}", title, author)
            }
            Media::Movie { title, director } => {
                format!("Movie: {} {}", title, director)    
            }
            Media::Audiobook { title } => {
                format!("Audiobook: {}", title)
            }
            Media::Podcast(id) => {
                format!("Podcast: {}", id)
            }
            Media::Placeholder => {
                format!("Placeholder")
            }
        }
    }
}

#[derive(Debug)]
struct Catalog {
    items: Vec<Media>
}

impl Catalog {
    fn new() -> Self {
        Catalog { items: vec![] }
    }

    fn add(&mut self, media: Media) {
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
    fn get_by_index(&self, index: usize) -> Option<&Media> {
        if self.items.len() > index {
            Some(&self.items[index])
        } else {
            None
        }
    }
}

enum MightHaveAValue<'a> {
    ThereIsAValue(&'a Media),
    NoValueAvailable
}

fn print_media(media: Media) {
    println!("{:#?}", media)
}


fn main() {
    let audiobook = Media::Audiobook {
        title: String::from("An Audiobook"),
    };

    let good_movie = Media::Movie {
        title: String::from("Good movie"),
        director: String::from("good director")
    };

    let bad_book = Media::Book {
        title: String::from("bad book"),
        author: String::from("bad author")
    };

    let podcast = Media::Podcast(10);
    
    let placeholder = Media::Placeholder;

    //println!("{}", audiobook.description());
    //println!("{}", good_movie.description());
    //println!("{}", bad_book.description());

    //print_media(audiobook);
    //print_media(good_movie);
    //print_media(bad_book);

    let mut catalog = Catalog::new();

    catalog.add(audiobook);
    catalog.add(good_movie);
    catalog.add(bad_book);
    catalog.add(podcast);
    catalog.add(placeholder);

    // println!("{:#?}", catalog.items.get(100))
    /*match catalog.items.get(100) {
        //Option::Some or Some
        Option::Some(value) => {
            println!("Item: {:#?}", value)
        }
        //Option::None or None
        Option::None => {
            println!("Nothing at that index")
        }
    }*/

    /*let item = catalog.get_by_index(40);

    println!("Custom get: {:#?}", item);*/

    /*match catalog.get_by_index(90) {
        MightHaveAValue::ThereIsAValue(value) => {
            println!("Custom get: {:#?}", value);
        }
        MightHaveAValue::NoValueAvailable => {
            println!("No value here!");
        }
    }*/

    /*if let MightHaveAValue::ThereIsAValue(value) = catalog.get_by_index(0) {
        println!("Item in pattern match: {:#?}", value);
    } else {
        println!("No value here!!!");
    }*/
    /*if let Some(value) = catalog.get_by_index(0) {
        println!("Item in pattern match: {:#?}", value);
    } else {
        println!("No value here!!!");
    }
    if let Some(value) = catalog.get_by_index(90) {
        println!("Item in pattern match: {:#?}", value);
    } else {
        println!("No value here!!!");
    }*/

    //unwrap
    //let item = catalog.get_by_index(0); //Some
    //println!("{:#?}",item.unwrap());

    //let item = catalog.get_by_index(40); //Error
    //println!("{:#?}",item.unwrap());

    //expect
    //let item = catalog.get_by_index(0); //Some
    //println!("{:#?}",item.expect("Expected there to be an item here!"));

    //let item = catalog.get_by_index(40); //Error
    //println!("{:#?}",item.expect("Expected there to be an item here!"));

    //unwrap_or
    let item = catalog.get_by_index(0); //Some
    let placeholder = Media::Placeholder;
    println!("{:#?}",item.unwrap_or(&placeholder));

    let item = catalog.get_by_index(40); //Error
    let placeholder = Media::Placeholder;
    println!("{:#?}",item.unwrap_or(&placeholder));

}
