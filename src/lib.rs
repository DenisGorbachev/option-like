#[macro_export]
macro_rules! option_like {
    (
        $(#[$meta:meta])*
        $vis:vis enum $name:ident<T> {
            $some:ident(T),
            $none:ident,
        }

        is_some => $is_some:ident
        is_none => $is_none:ident
    ) => {
        $(#[$meta])*
        $vis enum $name<T> {
            $some(T),
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
    use std::sync::LazyLock;

    option_like!(
        #[derive(Ord, PartialOrd, Eq, PartialEq, Clone, Debug)]
        enum Knowledge<T> {
            Known(T),
            Unknown,
        }

        is_some => is_known
        is_none => is_unknown
    );

    static KNOWN: LazyLock<Knowledge<bool>> = LazyLock::new(|| Known(true));
    static UNKNOWN: LazyLock<Knowledge<bool>> = LazyLock::new(|| Unknown);

    #[test]
    fn test_boolean_methods() {
        assert!(KNOWN.is_known());
        assert!(UNKNOWN.is_unknown());
    }

    #[test]
    fn test_from() {
        assert_eq!(Option::<bool>::from(KNOWN.clone()), Some(true));
        assert_eq!(Option::<bool>::from(UNKNOWN.clone()), None);
        assert_eq!(Knowledge::<bool>::from(Some(true)), Known(true));
        assert_eq!(Knowledge::<bool>::from(None), Unknown);
    }
}
