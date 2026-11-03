# Tooltip

1. **Or-patterns.** `(Top | Bottom, Start) => class!("left-0")` matches either
   side in one arm. `|` inside a pattern means "this or that", so the six arms
   cover all twelve side and alignment pairs.

2. **A component that adds nothing.** `tooltip_provider` returns
   `view! { (child) }`: it renders its children unchanged. Components are just
   async functions, so keeping Octane's shape costs one function call and no
   markup.

3. **Types are nominal.** `TooltipSide` and `HoverCardSide` have the same
   variants, but they are different types; you can't pass one where the other
   is expected. TypeScript compares shapes; Rust compares names. Each component
   file defines its own enums because `topcoat ui add` copies files one by one.
