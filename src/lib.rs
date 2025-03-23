//! Create your own enum type that behaves like Rust's `Option` but with custom names.
//!
//! # Example
//!
//! ```
//! use option_like::option_like;
//!
//! option_like!(
//!     #[derive(Debug, PartialEq)]
//!     pub enum Cached<T> {
//!         Hit(T),
//!         Miss,
//!     }
//!
//!     is_some => is_hit
//!     is_none => is_miss
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
/// - `$some`: Name of the variant that holds a value (e.g., `Hit`)
/// - `$none`: Name of the empty variant (e.g., `Miss`)
/// - `is_some => $is_some`: Name of the method that checks if the enum holds a value (e.g., `is_hit`)
/// - `is_none => $is_none`: Name of the method that checks if the enum is empty (e.g., `is_miss`)
#[macro_export]
macro_rules! option_like {
    (
        $(#[$meta:meta])*
        $vis:vis enum $name:ident<T> {
            $(#[$some_meta:meta])*
            $some:ident(T),
            $(#[$none_meta:meta])*
            $none:ident,
        }

        is_some => $is_some:ident
        is_none => $is_none:ident
    ) => {
        $(#[$meta])*
        $vis enum $name<T> {
            $(#[$some_meta])*
            $some(T),
            $(#[$none_meta])*
            $none,
        }

        use $name::*;

        impl<T> $name<T> {
            pub fn $is_some(&self) -> bool {
                match self {
                    $some(_) => true,
                    $none => false,
                }
            }

            pub fn $is_none(&self) -> bool {
                match self {
                    $some(_) => false,
                    $none => true,
                }
            }
        }

        impl<T> From<Option<T>> for $name<T> {
            fn from(value: Option<T>) -> Self {
                match value {
                    Some(inner) => $some(inner),
                    None => $none
                }
            }
        }

        impl<T> From<$name<T>> for Option<T> {
            fn from(value: $name<T>) -> Option<T> {
                match value {
                    $some(inner) => Some(inner),
                    $none => None
                }
            }
        }
    };
}

#[cfg(test)]
mod tests {
    option_like!(
        #[derive(Ord, PartialOrd, Eq, PartialEq, Default, Clone, Debug)]
        enum Cached<T> {
            Hit(T),
            #[default]
            Miss,
        }

        is_some => is_hit
        is_none => is_miss
    );

    const HIT: Cached<bool> = Hit(true);
    const MISS: Cached<bool> = Miss;

    #[test]
    fn test_boolean_methods() {
        assert!(HIT.is_hit());
        assert!(MISS.is_miss());
    }

    #[test]
    fn test_from() {
        assert_eq!(Option::<bool>::from(HIT.clone()), Some(true));
        assert_eq!(Option::<bool>::from(MISS.clone()), None);
        assert_eq!(Cached::<bool>::from(Some(true)), Hit(true));
        assert_eq!(Cached::<bool>::from(None), Miss);
    }
}
