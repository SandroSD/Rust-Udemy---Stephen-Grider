#[derive(Debug)]
pub enum Media {
    Book { title: String, author: String},
    Movie { title: String, director: String },
    Audiobook { title: String },
    // Podcast { episode_number: u32 }, // if the name that you want to write is to long, you can replace with the following sentence
    Podcast(u32),
    Placeholder,
}

impl Media {
    pub fn description(&self) -> String {
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