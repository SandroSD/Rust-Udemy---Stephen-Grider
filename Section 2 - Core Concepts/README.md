# Rust

Cargo is used to create, compile, run and manage projects

- Create a new project -> cargo new <projectname>
- Run a project -> cargo run -q (no debug logs)

---

## Arrays in rust

- vectors are like arrays that can grow/shrink in size
- rust also has 'arrays' they have fixed lengths

---

```
let deck: Deck = Deck { cards: vec![]}
```

`let deck: Deck` = Declares a new _binding_ (variable).

`: Deck` = Type Anotation. Describes the type of value 'deck' refers to.

`Deck { cards: vec![]}` = Struct literal. Creates an instance of a struct.

`vec![]` = Creates an empty vector. Again, the "!" indicates a macro

---

println("text {:?}", <variable>) // {:?} is a formatter for debug
println("text {:#?}", <variable>) // {:#?} is a good formatter for arrays for debug

---

```
#[derive(Debug)]
struct Deck {
    cards: Vec<String>,
}
```

`#[derive(Debug)]` = Whole statement defines 'attributes' for the Deck struct. Gives the rust compiler some extra instructions.

`derive` = Called the 'derive attribute'. Specifies which 'traits' automatically implement for this struct.

`Debug` = Called the 'Debug' trait. Traits a set of functions

---

Creates an **array** of strings. Arrays are fixed in size (can't grow/shrink)

```
let suits = ["Diamonds", "Clubs"];
let values = ["2", "3", "4", "5"];
```

Creates a **vector** of strings. Vectors are dynamic (they can grow/shrink)

```
let suits = vec!["Diamonds", "Clubs"];
let values = vec!["2", "3", "4", "5"];
```

---

Variables are **immutable** (can't change) by default

- Immutable

```
let numbers = vec![];

// Error! Can't change the value
numbers.push(1); // Error!

// Error! Can't reasign either
numbers = vec![]
```

- Mutable

```
let mut numbers = vec![];

// Ok!
numbers.push(1);

// Ok!
numbers = vec![];
```

---

Inherent Implementations

Fancy term for 'add a function to a struct'. Used to define **methdos** and **associated functions** (class methods). It has to have the same name as the struct.

---

Associated Functions

Use when you have functionality not tied to a specific instance.

**Example:** 'full_deck()', 'with_n_cards(10)', 'empty_deck()'.

```
impl Deck {
    fn new() ->  Self {
        // stuff...
    }
}

fn main() {
    Deck::new();
}
```

Methods

Use when you need to read or change fields on a specific instance.

**Example:** shuffling cards, adding a card, removing a card, checking if a card exists

```
impl Deck {
    fn shuffle(&self) {
        // stuff...
    }
}

fn main() {
    deck.shuffle();
}
```

---

Crate == Package

- Rust Standard Library

  - Included with every project without any additional install
  - Docks at **_doc.rust-lang.org/std_**

- External Crates
  - Have to be installed into our project with _cargo add rand_
  - Crate listing at **_crates.io_**
  - Docs also at **_docs.rs_**

---

rand crate

Code in all crates + programs is organized into _modules_.

Every crate has a 'root' module and might have some additional submodules

(root)
_functions, structs, etc_
(submodule 1)
_functions, structs, etc_
(end submodule1)
(submodule 2)
_functions, structs, etc_
(end submodule 2)
(end root)

---

numbers are a lot of groups

https://doc.rust-lang.org/beta/book/ch03-02-data-types.html#integer-types

https://doc.rust-lang.org/beta/book/ch03-02-data-types.html#floating-point-types
