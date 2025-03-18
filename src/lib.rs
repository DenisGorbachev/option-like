#[macro_export]
macro_rules! option_like {
    (
        $(#[$meta:meta])*
        $vis:vis enum $name:ident<T> {
            $some_name:ident(T),
            $none_name:ident,
        }

        is_some => $is_some_name:ident
        is_none => $is_none_name:ident
    ) => {
        $(#[$meta])*
        $vis enum $name<T> {
            $some_name(T),
            $none_name,
        }

        use $name::*;

        impl<T> $name<T> {
            pub fn $is_some_name(&self) -> bool {
                match self {
                    $some_name(_) => true,
                    $none_name => false,
                }
            }

            pub fn $is_none_name(&self) -> bool {
                match self {
                    $some_name(_) => false,
                    $none_name => true,
                }
            }
        }
    };
}

#[cfg(test)]
mod tests {
    option_like!(
        #[derive(Clone, Debug)]
        enum Knowledge<T> {
            Known(T),
            Unknown,
        }

        is_some => is_known
        is_none => is_unknown
    );

    #[test]
    fn must_run_map() {
        use Knowledge::*;
        let known = Known::<bool>(true);
        let unknown = Unknown::<bool>;
        assert!(known.is_known());
        assert!(unknown.is_unknown());
    }
}
