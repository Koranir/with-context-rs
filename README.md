# with-context-rs
Easy singleton initialization &amp; usage for Rust.

## `with-context`
`with-context` is a set of macros to allow for easy singleton initialization & usage.

If a singleton(context) has not been initialized, it will panic in runtime *only* in debug mode. Release mode disables safety catches so test thoroughly.
### Usage:
```rs
use with_context::*;

pub struct ExampleContext {
    pub initialized: bool,
    pub name: String,
}

// This defines a context that can be used with either ctx_req! on a function or block,
// Or ctx_get!.
ctx_def!(ectx: ExampleContext);

// The context must be initialized before we can do anything with it, or it will panic.
fn main() {
    ctx_init!(ectx => {
        ExampleContext { initilized: true, name: String::from("Example Context") }
    });

    // When using ctx_req!, the context can be renamed.
    ctx_req!(ec: ectx => {
        println!("ExampleContext is{} initialized.", if ec.initilized {""} else {"n't"});
    });
    // ctx_req_mut! allows mutating the contents.
    ctx_req_mut!(ec: ectx => {
        println!("ExampleContext has name {}", ec.name);
        ec.name = String::from("New Name");
        println!("Now ec has name {}", ec.name);
    });

    let example_ref: &ExampleContext = ctx_get!(ec)
    // with_context does not have borrow checking. Make sure the base struct has mutexes for thread safety, or wrap it in another struct.
    let example_ref_mut: &mut ExampleContext = ctx_get_mut!(ectx);

    // Functions that use a context look just like normal functions.
    set_name(String::from("New Name 2)"));
}

// ctx_req! and ctx_req_mut! can be used on pub fn and fn declarations.
// Does not support constant or unsafe functions.
ctx_req_mut!(ec: ectx => {
    fn set_name(name: String) {
        ec.name = name;
    }
});

```
