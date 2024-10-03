# Section 4 - Lifetime

- Lifetimes: Refers to how long an owner/reference exists.
- Generic Lifetime / Lifetime Annotations: Extra syntax added in to clarify relationship between different lifetimes.

---

Think about function argument types

- **Need to store the argument somewhere?** -> Favor taking ownership (receive a value).
- **Need to do a calculation with the value?** -> Favor receiving a read-only ref.
- **Need to change the value in some way?** -> Favor receiving a mutable ref.

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
<td>Method</td>
<td>add_account()</td>
<td>account: Account</td>
<td>-</td>
</tr>
<tr>
<td>Calculate the total balance of all accounts</td>
<td>Method</td>
<td>total_balance()</td>
<td>-</td>
<td>i32</td>
</tr>
<tr>
<td>Create a Vec containing the summaries of all accounts</td>
<td>Method</td>
<td>summary()</td>
<td>-</td>
<td>Vec< String ></td>
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
<td>Method</td>
<td>deposit()</td>
<td>amount: i32</td>
<td>i23</td>
</tr>
<tr>
<td>Remove the given amount of money from the accounts 'balance'</td>
<td>Method</td>
<td>withdraw()</td>
<td>amount: i32</td>
<td>i32</td>
</tr>
<tr>
<td>Create an account summary as a string and return it</td>
<td>Method</td>
<td>summary()</td>
<td>-</td>
<td>String</td>
</tr>
</tbody>
</table>
