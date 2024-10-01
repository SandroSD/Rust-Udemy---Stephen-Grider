# Section 3 - Ownership & Borrowing

Bank

<table>
<thead>
<tr>
<th>Description</th>
<th>Method or Assoc. Func?</th>
<th>Name</th>
<th>Args</th>
<th>Returns</th>
</tr>
</thead>
<tbody>
<tr>
<td>Create a 'Bank' instance</td>
<td>Assoc. Func</td>
<td>new()</td>
<td>-</td>
<td>Bank</td>
</tr>
<tr>
<td>Add an account to the list of accounts</td>
<td></td>
<td></td>
<td></td>
<td></td>
</tr>
<tr>
<td>Calculate the total balance of all accounts</td>
<td></td>
<td></td>
<td></td>
<td></td>
</tr>
<tr>
<td>Create a Vec containing the summaries of all accounts</td>
<td></td>
<td></td>
<td></td>
<td></td>
</tr>
</tbody>
</table>

---

Account

<table>
<thead>
<tr>
<th>Description</th>
<th>Method or Assoc. Func?</th>
<th>Name</th>
<th>Args</th>
<th>Returns</th>
</tr>
</thead>
<tbody>
<tr>
<td>Create a 'Account' instance</td>
<td>Assoc. Func</td>
<td>new()</td>
<td>id: u32<br/>holder: String</td>
<td>Account</td>
</tr>
<tr>
<td>Add the given amount of money to the accounts 'balance'</td>
<td></td>
<td></td>
<td></td>
<td></td>
</tr>
<tr>
<td>Remove the given amount of money from the accounts 'balance'</td>
<td></td>
<td></td>
<td></td>
<td></td>
</tr>
<tr>
<td>Create an account summary as a string and return it</td>
<td></td>
<td></td>
<td></td>
<td></td>
</tr>
</tbody>
</table>

---

## Ownership - Borrowing - Lifetimes

Three connected systems.

Tough to understand, but represent 90% of the difficulty of rust.

Dramatically change the way you will design and write code (compared to other languages).

### Rules:

- Ownership

  1. Every value is **_owned_** by a single variable, struct, vector, etc at a time.
  2. Reassigning the value to another variable, passing it to a function, putting it into a vector, etc, **_moves_** the value. The old variable can't be used anymore!

- Borrowing

  3. You can create many read-only references to a value that exist at the same time
  4. You can't move a value while a ref to the value exists
  5. You can make a writeable (mutable) reference to a value **_only if_** there are no read-only references currently in use. One mutable ref to a value can exist at a time
  6. You can't mutate a value through the owner when any ref(mutable or immutable) to the value exists
  7. Some types of values are **_copied_** instead of moved (numbers, bools, chars, arrays/tuples with copyable elements)

- Lifetimes

  8. When a variable goes out of scope, the value owned by it is **_dropped_** (cleaned up in memory)
  9. Values can't be dropped if there are still active references to it
  10. References to a value can't outlive the value they refer to

---

11. **These rules will dramatically change how you write code (compared to other languages)**
12. **When in doubt, remember that Rust wants to minimize unexpected updates to data**

---

The goal of **ownership** is to limit the ways you can reference and change data.

This limitation will reduce the number of bugs + make your code easier to understand

**Lesson #1** (rules 3, 5, 6)

Multiple things can refer to a value at the same time, but they are all read-only

**Lesson #2** (rules 1, 5, 6)

A value can _only_ be updated when there are no read-only references to it

---

These form the basis of ownership + borrowing systems

These are rules that are implemented in Rust

The goal of these rules is to reduce bugs like the one we saw in the 'car' example

---

Given the ownership system, how do we write useful code?

1. Manually move values back and forth between different owners.
2. Use the borrowing system.
