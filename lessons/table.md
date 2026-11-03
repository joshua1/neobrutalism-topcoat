# Table

1. **`struct` with named fields.** `struct Invoice { invoice: &'static str,
   .. }` declares a concrete type, like a TypeScript `interface` that values
   are actually built from. Every field must be given when you create one.

2. **Fixed-size arrays.** `const INVOICES: [Invoice; 7]` has its length in
   its type. A `Vec<Invoice>` can grow; an array's length is fixed at compile
   time, so it fits constant data like this.

3. **Borrowing in a loop, and iterators.** `for invoice in &INVOICES` loops
   over references, leaving the array untouched. `INVOICES.iter().take(3)`
   replaces `invoices.slice(0, 3)`: iterators are lazy and copy nothing.

4. **`format!` builds a `String`.** `format!("table-actions-{slug}")` works
   like a template literal, and `{slug}` reads the variable named `slug`. The
   result is an owned `String`, so using it in three places takes
   `menu_id.clone()`.
