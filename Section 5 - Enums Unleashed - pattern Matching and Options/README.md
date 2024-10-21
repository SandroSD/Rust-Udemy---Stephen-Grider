# Section 5

## Structs vs Enums

Does each thing you're modeling have the **same methods**? -> You might want to use an **enum**.

Does each thing have **some same, but some different methods**? -> You might want to use **structs**.

---

- **Rust doesn't have null, nil, or undefined**.
- Instead, we get a built-in enum called 'Option'.
- Has two variants - _Some_ and _None_.
- If you want to work with Option you have to use pattern matching (the 'if let' thing) or a match statement.
- Forces you to handle the case in which you have a value and the case in which you don't.

```Rust
enum Option {
    Some(value),
    None
}
```

---

## Other ways to handle Options

### UNWRAP

```Rust
item.unwrap()
```

- If 'item' is a Some, returns the value in the Some.
- If 'item' is a None, panics!
- **Use for quick debugging or examples**

### EXPECT

```Rust
item.expect("There should be a value here")
```

- If 'item' is a Some, returns the value in the Some.
- If 'item' is a None, prints the provided debug message and panics!
- **Use when we _want_ to crash if there is no value**

### UNWRAP_OR

```Rust
item.unwrap_or(&placeholder)
```

- If 'item' is a Some, returns the value in the Some.
- If 'item' is a None, returns the provided default value.
- **Use when it makes sense to provide a fallback value**

### Other ways

- Link: https://doc.rust-lang.org/stable/std/option/enum.Option.html
