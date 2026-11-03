# Navigation Menu

1. **Components can call components.** `navigation_menu_content` returns
   `navigation_menu_viewport(<div ...>(child)</div>)`: it wraps its children in
   markup and hands them on as the inner component's children, the way a React
   component renders another one.

2. **A function that returns a constant.** `navigation_menu_trigger_style()`
   just returns `TRIGGER_STYLE`. `StaticClass` is `Copy`, so each call hands
   back a cheap copy. A function keeps the Octane call shape
   (`navigationMenuTriggerStyle()`) and leaves room to add arguments later.

3. **Private helper components.** The example's `list_item` is
   `async fn`, not `pub async fn`: a component used only in this file, the
   same as a non-exported function in a `.tsrx` module.

4. **Array constants with a fixed length.** `const COMPONENTS: [(&str, &str, &str); 6]`
   is an array of six tuples. The length is part of the type, so the compiler
   counts the entries for you, and no memory is allocated at run time.
