//! The `with` macro.
//!
//! This is a macro that takes an object and lets you call methods on that object without naming it.
//! The first argument is an expression that will be assigned to a variable in let binding. To make
//! that binding mutable, prepend `mut` to the expression.
//! Calling a function that starts with a `.` will be converted into a method call using this
//! variable.
//!
//! The supported forms are:
//! - `.method(args..)`
//! - `let pat = .method(args..);`
//! - `var = .method(args..);`
//!
//! Anything else will be evaluated unmodified as an expression.
//!
//! # Example
//! ```
//! use with_macro::with;
//!
//! let vec = with! {
//!     mut Vec::new() =>
//!         .push(1)
//!         .push(42)
//!         .push(-13)
//!         let l = .len();
//!         assert_eq!(l, 3);
//! };
//!
//! assert_eq!(vec, [1, 42, -13]);
//! ```

/// The `with` macro.
///
/// See the module documentation for more details.
#[macro_export]
macro_rules! with {
    // mut expr => ...
    (mut $obj:expr => $($body:tt)*) => ({
        let mut obj = $obj;
        with!(@parse obj $($body)*);
        obj
    });

    // expr => ...
    ($obj:expr => $($body:tt)*) => ({
        let obj = $obj;
        with!(@parse obj $($body)*);
        obj
    });

    // termination rule
    (@parse $obj:ident) => ();

    // .method(args..)
    (@parse $obj:ident . $method:ident ( $($args:expr),* ) $($tail:tt)*) => {
        $obj.$method($($args),*);
        with!(@parse $obj $($tail)*)
    };

    // let pat = .method(args..);
    (@parse $obj:ident let $var:pat = . $method:ident ( $($args:expr),* ) ; $($tail:tt)*) => {
        let $var = $obj.$method($($args),*);
        with!(@parse $obj $($tail)*)
    };

    // var = .method(args..);
    (@parse $obj:ident $var:ident = . $method:ident ( $($args:expr),* ) ; $($tail:tt)*) => {
        $var = $obj.$method($($args),*);
        with!(@parse $obj $($tail)*)
    };

    // arbitrary expresion
    (@parse $obj:ident $exp:expr ; $($tail:tt)*) => {
        $exp;
        with!(@parse $obj $($tail)*)
    }
}

#[cfg(test)]
mod tests {
    use std::cell::Cell;

    #[derive(Debug, PartialEq, Eq)]
    struct Foo(Cell<i32>);

    impl Foo {
        fn new(val: i32) -> Self {
            Foo(Cell::new(val))
        }

        fn get_val(&self) -> i32 {
            self.0.get()
        }

        fn set_val(&self, val: i32) {
            self.0.set(val)
        }

        fn add(&self, n: i32) {
            self.0.set(self.0.get() + n)
        }

        fn mul(&self, n: i32) {
            self.0.set(self.0.get() * n)
        }
    }

    #[test]
    fn basic() {
        let a;
        let foo = with! {
            Foo::new(0) =>
                .set_val(10)
                .mul(2)
                a = .get_val();
                .add(1)
                let n = .get_val();
                assert_eq!(n, 21);
                .mul(2)
        };

        assert_eq!(a, 20);
        assert_eq!(foo.get_val(), 42);
    }

    #[test]
    fn mutable() {
        let vec = with! {
            mut Vec::new() =>
                .push(1)
                .push(42)
                .push(-13)
                let l = .len();
                assert_eq!(l, 3);
        };

        assert_eq!(vec, [1, 42, -13]);
    }

    #[test]
    fn nested() {
        let vec = with! {
            mut Vec::new() =>
                .push(with!{
                    Foo::new(3) =>
                })
                .push(with!{
                    Foo::new(0) =>
                        .set_val(10)
                        .add(3)
                })
        };

        assert_eq!(vec, [Foo::new(3), Foo::new(13)]);
    }
}
