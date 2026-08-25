//! Create your own enum type that behaves like Rust's `Option` but with custom names.
//!
//! ## Example
//!
//! ```
//! use option_like::option_like;
//!
//! option_like!(
//!     #[derive(Debug, PartialEq)]
//!     pub enum Cached<T> {
//!         Miss,
//!         Hit(T),
//!     }
//!
//!     is_none => is_miss
//!     is_some => is_hit
//! );
//!
//! // Create instances
//! let c1 = Cached::<u32>::Hit(42);
//! let c2 = Cached::<u32>::Miss;
//!
//! // Boolean tests
//! assert!(c1.is_hit());
//! assert!(c2.is_miss());
//!
//! // Convert to Option
//! assert_eq!(Option::<u32>::from(c1), Some(42));
//! assert_eq!(Option::<u32>::from(c2), None);
//!
//! // Convert from Option
//! assert_eq!(Cached::<u32>::from(Some(42)), Cached::Hit(42));
//! assert_eq!(Cached::<u32>::from(None), Cached::Miss);
//! ```

#![no_std]
#![cfg_attr(not(test), deny(unused_crate_dependencies))]

/// Creates the inherent implementation block for an option-like enum.
#[macro_export]
macro_rules! option_like_impl {
    (
        $name:ident,
        $none:ident,
        $some:ident,
        $is_none:ident,
        $is_some:ident $(,)?
    ) => {
        impl<T> $name<T> {
            pub fn $is_none(&self) -> bool {
                match self {
                    Self::$none => true,
                    Self::$some(_) => false,
                }
            }

            pub fn $is_some(&self) -> bool {
                match self {
                    Self::$none => false,
                    Self::$some(_) => true,
                }
            }

            #[inline]
            pub fn map<U, F>(self, f: F) -> $name<U>
            where
                F: FnOnce(T) -> U,
            {
                match self {
                    Self::$none => $name::$none,
                    Self::$some(x) => $name::$some(f(x)),
                }
            }

            #[inline(always)]
            #[track_caller]
            pub fn unwrap(self) -> T {
                match self {
                    Self::$none => Self::unwrap_failed(),
                    Self::$some(val) => val,
                }
            }

            #[inline]
            pub fn unwrap_or_default(self) -> T
            where
                T: Default,
            {
                match self {
                    Self::$none => T::default(),
                    Self::$some(x) => x,
                }
            }

            #[inline]
            #[track_caller]
            pub fn unwrap_or_else<F>(self, f: F) -> T
            where
                F: FnOnce() -> T,
            {
                match self {
                    Self::$none => f(),
                    Self::$some(x) => x,
                }
            }

            #[inline]
            #[track_caller]
            pub fn expect(self, msg: &str) -> T {
                match self {
                    Self::$none => Self::expect_failed(msg),
                    Self::$some(val) => val,
                }
            }

            #[cold]
            #[track_caller]
            const fn unwrap_failed() -> ! {
                panic!(stringify!("called `", $name, "::unwrap()` on a `", $none, "` value"))
            }

            #[cold]
            #[track_caller]
            const fn expect_failed(msg: &str) -> ! {
                panic!("{}", msg)
            }
        }
    };
}

/// Creates the `From<Option<T>>` and `Into<Option<T>>` conversions for an option-like enum.
#[macro_export]
macro_rules! option_like_from_into_option {
    (
        $name:ident,
        $none:ident,
        $some:ident $(,)?
    ) => {
        impl<T> From<Option<T>> for $name<T> {
            fn from(value: Option<T>) -> Self {
                match value {
                    None => Self::$none,
                    Some(inner) => Self::$some(inner),
                }
            }
        }

        impl<T> From<$name<T>> for Option<T> {
            fn from(value: $name<T>) -> Option<T> {
                match value {
                    $name::$none => None,
                    $name::$some(inner) => Some(inner),
                }
            }
        }
    };
}

/// Creates a new enum type that behaves like Rust's `Option<T>` but with custom names.
///
/// This macro allows you to create your own Option-like enum with customized names for the variants
/// and boolean test methods, while providing automatic conversions to and from the standard Option type.
///
/// # Parameters
///
/// - `$(#[$meta:meta])*`: Optional attributes to apply to the enum (e.g., `#[derive(...)]`)
/// - `$vis`: Visibility of the enum (e.g., `pub`)
/// - `$name`: Name of the enum (e.g., `Cached`)
/// - `$none`: Name of the empty variant (e.g., `Miss`)
/// - `$some`: Name of the variant that holds a value (e.g., `Hit`)
/// - `is_none => $is_none`: Name of the method that checks if the enum is empty (e.g., `is_miss`)
/// - `is_some => $is_some`: Name of the method that checks if the enum holds a value (e.g., `is_hit`)
#[macro_export]
macro_rules! option_like {
    (
        $(#[$meta:meta])*
        $vis:vis enum $name:ident<T> {
            $(#[$none_meta:meta])*
            $none:ident,
            $(#[$some_meta:meta])*
            $some:ident(T),
        }

        is_none => $is_none:ident
        is_some => $is_some:ident
    ) => {
        $(#[$meta])*
        $vis enum $name<T> {
            $(#[$none_meta])*
            $none,
            $(#[$some_meta])*
            $some(T),
        }

        $crate::option_like_impl!(
            $name,
            $none,
            $some,
            $is_none,
            $is_some,
        );

        $crate::option_like_from_into_option!(
            $name,
            $none,
            $some,
        );
    };
}

#[cfg(test)]
mod tests {
    option_like!(
        #[derive(Ord, PartialOrd, Eq, PartialEq, Default, Clone, Debug)]
        enum Cached<T> {
            #[default]
            Miss,
            Hit(T),
        }

        is_none => is_miss
        is_some => is_hit
    );

    use Cached::*;

    fn hit() -> Cached<bool> {
        Hit(true)
    }

    fn miss() -> Cached<bool> {
        Miss
    }

    #[test]
    fn test_boolean_methods() {
        assert!(hit().is_hit());
        assert!(miss().is_miss());
    }

    #[test]
    fn test_from() {
        assert_eq!(Option::<bool>::from(hit()), Some(true));
        assert_eq!(Option::<bool>::from(miss()), None);
        assert_eq!(Cached::<bool>::from(Some(true)), Hit(true));
        assert_eq!(Cached::<bool>::from(None), Miss);
    }

    #[test]
    fn test_map() {
        assert_eq!(hit().map(|t| !t), Hit(false));
        assert_eq!(miss().map(|t| !t), Miss);
    }

    #[test]
    fn test_unwrap_or_default() {
        assert!(hit().unwrap_or_default());
        assert!(!miss().unwrap_or_default());
    }

    #[test]
    fn test_unwrap_or_else() {
        assert!(hit().unwrap_or_else(|| false));
        assert!(miss().unwrap_or_else(|| true));
    }

    #[test]
    fn test_unwrap_no_panic() {
        assert!(hit().unwrap());
    }

    #[test]
    #[should_panic]
    fn test_unwrap_panic() {
        miss().unwrap();
    }

    #[test]
    fn test_expect_no_panic() {
        assert!(hit().expect("should not panic"));
    }

    #[test]
    #[should_panic]
    fn test_expect_panic() {
        miss().expect("should panic");
    }
}
