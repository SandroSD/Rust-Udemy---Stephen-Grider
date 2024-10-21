mod content;

use content::media::Media;
use content::catalog::Catalog;

enum MightHaveAValue<'a> {
    ThereIsAValue(&'a Media),
    NoValueAvailable
}

fn print_media(media: Media) {
    println!("{:#?}", media)
}

fn main() {
    // if I don't use the 'use' keyword
    /*let audiobook = content::media::Media::Audiobook {
        title: String::from("An Audiobook"),
    };*/
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
