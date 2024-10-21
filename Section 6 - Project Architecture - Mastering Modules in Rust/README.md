# Section 6 - Modules

## Three different ways to create modules

### Option 1

Create a mod in an existing file.

Most appropiate when you have a really large file with a lot of stuff going on.

Functions, structs, enums, etc, must have the 'pub' (public) keyword to make them visible outside the module

```Rust
mod content {
    pub enum Media { /* fields */ }
    /* Media impl */
    pub struct catalog { /* fields */ }
    /* Catalog impl */
}

fn main() {
    let catalog = content::Catalog::new();
}
```

### Option 2

Create a module in a new single file in the same folder.

Most appropiate when you want a separate module to organize code, but it doesn't need to span several files.

```Rust
src/content.rs

    pub enum Media { /* fields */ }
    /* Media impl */
    pub struct catalog { /* fields */ }
    /* Catalog impl */
```

```Rust
main.rs

mod content; // There's a module called 'content' and we want access to everything inside of it.

fn main() {
    let catalog = content::Catalog::new();
}

// or if you don't want to repeat the same thing at the beginning

mod content;

use content::Catalog;

fn main() {
    let catalog = Catalog::new();
}
```

### Option 3

Spread code out among several separate files in a new folder.

Most appropiate when you have a large module.

**Has a couple of confusing parts**

_Every file **and folder** makes it's own separate module_

```Rust
'content' folder

content/media.rs

pub enum Media { /* fields */ }
/* Media impl */

content/catalog.rs

pub struct Catalog { /* fields */ }
/* Catalog impl */

content/mod.rs

pub mod media;
pub mod catalog;
```

```Rust
main.rs

mod content;

fn main() {
    let catalog = content::catalog::Catalog::new();
}
```

**You can't do deeply nested imports.**

root module can't reach enum Media directly

**You have to chain imports**

root module imports everything from the 'content' module.

content module imports and exports everything from the 'media' module.
