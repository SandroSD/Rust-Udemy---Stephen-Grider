//use rand::thread_rng; // in order not to write rand:: every time you want to use a method of this module.
use rand::{thread_rng, seq::SliceRandom/*random, rngs::OsRng*/}; // if you want to call more than one method from that module (kind of destructuring).
#[derive(Debug)]

struct Deck {
    cards: Vec<String>,
}

// inherent implementation
impl Deck {
    //fn new() -> Deck {
    fn new() -> Self {
        // List of 'suits' - 'hearts', 'spades'
        let suits= ["Hearts", "Spades", "Diamonds"];

        // List of 'values' - 'ace', 'two', 'three'
        let values = ["Ace", "Two", "Three"];

        let mut cards = vec![]; // make vector mutable using mut

        //Double nested for loop
        for suit in suits {
            for value in values {
                let card = format!("{} of {}", value, suit);
                cards.push(card);
            }
        }

        //let deck: Deck = Deck { cards }; // same as Javascript key-value objects
        //let deck: Deck = Deck { cards: Vec::new() };

        //return deck;
        //return Deck { cards };
        Deck { cards } // this is an implicit return
    }
    
    fn shuffle(&mut self) {
        let mut rng = thread_rng();

        self.cards.shuffle(&mut rng);
    }

    fn deal(&mut self, num_cards: usize) -> Vec<String> {
        self.cards.split_off(self.cards.len() - num_cards)
    }
}

fn main() {
    let mut deck = Deck::new(); // we have to make it mutable to change it.

    deck.shuffle();

    // Probably need to add error handling!!!
    let cards = deck.deal(3);


    println!("Here's your hand: {:#?}", cards);
    println!("Here's your deck: {:#?}", deck);
}
