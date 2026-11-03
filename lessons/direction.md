# Direction

1. **Implementing a library trait for your type.**
   `impl AttributeValueViewParts for Direction` teaches Topcoat how to render a
   `Direction` as an attribute value, so `dir=(direction)` just works. It's
   like implementing an interface after the fact. Neither Topcoat nor
   `Direction` had to know about the other in advance.

2. **`&self` vs `self`.** `attribute_present(&self)` only looks at the value,
   so it borrows it. `into_view_parts(self, ..)` consumes it, because rendering
   is the value's last use. The signature tells you which one happens.

3. **`_cx`: deliberately unused.** The trait hands every implementation the
   request context. We don't need it, and a leading underscore tells the
   compiler that's intentional, so it doesn't warn.

4. **No `useDirection` hook.** Components run once on the server, top to
   bottom, so there's no React-style context to read later. Whoever picked the
   direction passes the same `Direction` value to anything else that needs it.
