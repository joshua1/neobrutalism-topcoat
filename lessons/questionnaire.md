# Questionnaire

1. **`expr!` outside `view!`.** `$(..)` is shorthand for `expr!(..)` inside a
   view. Calling `expr!` directly, before the view, builds the same
   `Expr<bool>` as a value you can name and pass around:
   `let email_invalid = expr!(..)` and then
   `questionnaire_item(invalid: email_invalid, ..)`. It still updates in the
   browser whenever the `email` signal it reads changes.

2. **Blocks are expressions.** `expr!({ let value = email.get(); if .. })`
   is one expression: a `{ }` block runs its statements and evaluates to its
   last line, the one without a semicolon. TypeScript needs an IIFE,
   `(() => { const value = ..; return .. })()`, for the same thing.

3. **`else if` chains return values.** With no `&&` or `||` in runtime
   expressions, the email check is a chain of `if` / `else if` / `else`, each
   branch a `bool`. Every branch of an `if` used as a value must have the
   same type, so the compiler rejects a branch that forgets to return one.

4. **Rust strings and browser strings agree.** `contains`, `starts_with`,
   `ends_with` and `is_empty` belong to the runtime's shared vocabulary: the
   server runs them as Rust `String` methods for the first render, and the
   browser runs the generated JavaScript as the user types. A regular
   expression is not in that vocabulary, so the example approximates
   Octane's regex with these methods.
