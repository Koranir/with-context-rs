//! ## `with-context`
//! `with-context` is a set of macros to allow for easy singleton initialization & usage.
//! 
//! If a singleton(context) has not been initialized, it will panic in runtime *only* in debug mode. Release mode disables safety catches so test thoroughly.
//! ### Usage:
//! ```rs
//! use with_context::*;
//! 
//! pub struct ExampleContext {
//!     pub initialized: bool,
//!     pub name: String,
//! }
//! 
//! // This defines a context that can be used with either ctx_req! on a function or block,
//! // Or ctx_get!.
//! ctx_def!(ectx: ExampleContext);
//! 
//! // The context must be initialized before we can do anything with it, or it will panic.
//! fn main() {
//!     ctx_init!(ectx => {
//!         ExampleContext { initilized: true, name: String::from("Example Context") }
//!     });
//! 
//!     // When using ctx_req!, the context can be renamed.
//!     ctx_req!(ec: ectx => {
//!         println!("ExampleContext is{} initialized.", if ec.initilized {""} else {"n't"});
//!     });
//!     // ctx_req_mut! allows mutating the contents.
//!     ctx_req_mut!(ec: ectx => {
//!         println!("ExampleContext has name {}", ec.name);
//!         ec.name = String::from("New Name");
//!         println!("Now ec has name {}", ec.name);
//!     });
//! 
//!     let example_ref: &ExampleContext = ctx_get!(ec)
//!     // with_context does not have borrow checking. Make sure the base struct has mutexes for thread safety, or wrap it in another struct.
//!     let example_ref_mut: &mut ExampleContext = ctx_get_mut!(ectx);
//! 
//!     // Functions that use a context look just like normal functions.
//!     set_name(String::from("New Name 2)"));
//! }
//! 
//! // ctx_req! and ctx_req_mut! can be used on pub fn and fn declarations.
//! // Does not support constant or unsafe functions.
//! ctx_req_mut!(ec: ectx => {
//!     fn set_name(name: String) {
//!         ec.name = name;
//!     }
//! });
//! 
//! ```

#[macro_export]
macro_rules! ctx_def {
    ($visibility:vis $name:ident: $($ty:tt)::*) => {
        $visibility mod $name {
            use super::*;
            pub static mut STATIC_CONTEXT: $crate::WithContext<$($ty)::*> = $crate::WithContext{context: None};
        }
    };
}
#[macro_export]
macro_rules! ctx_init {
    ($($path:ident)::+ => $code:block) => {
        unsafe {
            $($path)::+::STATIC_CONTEXT.context = Some(
                $code
            );
        }
    };
}
#[macro_export]
macro_rules! ctx_req {
    ($($context:ident: $($path:ident)::+),* => {
        $visibility:vis fn $name:ident ($($arg:ident: $argt:ty),*) $(-> $ret:ty)? {
            $($body:tt)*
        }
    }) => {
        $visibility fn $name($($arg: $argt), *) $(-> $ret)? {
            $(
                let $context = unsafe {$($path)::+::STATIC_CONTEXT.get()};
            )*
            $($body)*
        }
    };
    ($($context:ident: $($path:ident)::+),* => {
        $($body:tt)*
    }) => {
        {
            $(
                let $context = unsafe {$($path)::+::STATIC_CONTEXT.get()};
            )*
            $($body)*
        }
    }
}
#[macro_export]
macro_rules! ctx_req_mut {
    ($($context:ident: $($path:ident)::+),* => {
        $visibility:vis fn $name:ident ($($arg:ident: $argt:ty),*) $(-> $ret:ty)? {
            $($body:tt)*
        }
    }) => {
        $visibility fn $name($($arg: $argt), *) $(-> $ret)? {
            $(
                let $context = unsafe {$($path)::+::STATIC_CONTEXT.get_mut()};
            )*
            $($body)*
        }
    };
    ($($context:ident: $($path:ident)::+),* => {
        $($body:tt)*
    }) => {
        {
            $(
                let $context = unsafe {$($path)::+::STATIC_CONTEXT.get_mut()};
            )*
            $($body)*
        }
    }
}
#[macro_export]
macro_rules! ctx_get {
    ($($path:ident)::+) => {
        unsafe {$($path)::+::STATIC_CONTEXT.get()}
    };
}
#[macro_export]
macro_rules! ctx_get_mut {
    ($($path:ident)::+) => {
        unsafe {$($path)::+::STATIC_CONTEXT.get_mut()}
    };
}

pub struct WithContext<T> {
    pub context: Option<T>,
}
impl<T> WithContext<T> {
    #[cfg(debug_assertions)]
    pub fn get(&self) -> &T {
        match &self.context {
            Some(t) => {
                t
            }
            None => {
                panic!("Context {} has not been initialized yet!", std::any::type_name::<T>())
            }
        }
    }
    #[cfg(not(debug_assertions))]
    pub fn get(&self) -> &T {
        unsafe { self.context.as_ref().unwrap_unchecked() }
    }
    #[cfg(debug_assertions)]
    pub fn get_mut(&mut self) -> &mut T {
        match &mut self.context {
            Some(t) => {
                t
            }
            None => {
                panic!("Context '{}' has not been initialized yet!", std::any::type_name::<T>())
            }
        }
    }
    #[cfg(not(debug_assertions))]
    pub fn get_mut(&mut self) -> &mut T {
        unsafe { self.context.as_mut().unwrap_unchecked() }
    }
}
