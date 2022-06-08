//! [`vars!`] macro implementation.

/// Constructs [`graphql::Variables`] via JSON-like syntax.
///
/// [`graphql::Variables`] key should implement [`Into`]`<`[`String`]`>`.
/// ```rust
/// # use std::borrow::Cow;
/// #
/// # use juniper::graphql;
/// #
/// let code = 200;
/// let features = vec!["key", "value"];
/// let key: Cow<'static, str> = "key".into();
///
/// let value: graphql::Variables = graphql::vars! {
///     "code": code,
///     "success": code == 200,
///     features[0]: features[1],
///     key: @var,
/// };
/// ```
///
/// See [`graphql::input_value!`] for more info on syntax of value after `:`.
///
/// [`graphql::input_value!`]: crate::graphql::input_value
/// [`graphql::Variables`]: crate::graphql::Variables
macro_rules! vars {
    ////////////
    // Object //
    ////////////

    // Done.
    (@object $object:ident () () ()) => {};

    // Insert the current entry followed by trailing comma.
    (@object $object:ident [$($key:tt)+] ($value:expr) , $($rest:tt)*) => {
        let _ = $object.insert(($($key)+).into(), $value);
        $crate::graphql::vars! {@object $object () ($($rest)*) ($($rest)*)};
    };

    // Current entry followed by unexpected token.
    (@object $object:ident [$($key:tt)+] ($value:expr) $unexpected:tt $($rest:tt)*) => {
        $crate::graphql::vars! {@unexpected $unexpected};
    };

    // Insert the last entry without trailing comma.
    (@object $object:ident [$($key:tt)+] ($value:expr)) => {
        let _ = $object.insert(($($key)+).into(), $value);
    };

    // Next value is `null`.
    (@object $object:ident ($($key:tt)+) (: null $($rest:tt)*) $copy:tt) => {
        $crate::graphql::vars! {
            @object $object
            [$($key)+]
            ($crate::graphql_input_value!(null)) $($rest)*
        };
    };

    // Next value is `None`.
    (@object $object:ident ($($key:tt)+) (: None $($rest:tt)*) $copy:tt) => {
        $crate::graphql::vars! {
            @object $object
            [$($key)+]
            ($crate::graphql_input_value!(None)) $($rest)*
        };
    };

    // Next value is a variable.
    (@object $object:ident ($($key:tt)+) (: @$var:ident $($rest:tt)*) $copy:tt) => {
        $crate::graphql::vars! {
            @object $object
            [$($key)+]
            ($crate::graphql_input_value!(@$var)) $($rest)*
        };
    };

    // Next value is an array.
    (@object $object:ident ($($key:tt)+) (: [$($array:tt)*] $($rest:tt)*) $copy:tt) => {
        $crate::graphql::vars! {
            @object $object
            [$($key)+]
            ($crate::graphql_input_value!([$($array)*])) $($rest)*
        };
    };

    // Next value is a map.
    (@object $object:ident ($($key:tt)+) (: {$($map:tt)*} $($rest:tt)*) $copy:tt) => {
        $crate::graphql::vars! {
            @object $object
            [$($key)+]
            ($crate::graphql_input_value!({$($map)*})) $($rest)*
        };
    };

    // Next value is `true`, `false` or enum ident followed by a comma.
    (@object $object:ident ($($key:tt)+) (: $ident:ident , $($rest:tt)*) $copy:tt) => {
        $crate::graphql::vars! {
            @object $object
            [$($key)+]
            ($crate::graphql_input_value!($ident)) , $($rest)*
        };
    };

    // Next value is `true`, `false` or enum ident without trailing comma.
    (@object $object:ident ($($key:tt)+) (: $last:ident ) $copy:tt) => {
        $crate::graphql::vars! {
            @object $object
            [$($key)+]
            ($crate::graphql_input_value!($last))
        };
    };

    // Next value is an expression followed by comma.
    (@object $object:ident ($($key:tt)+) (: $value:expr , $($rest:tt)*) $copy:tt) => {
        $crate::graphql::vars! {
            @object $object
            [$($key)+]
            ($crate::graphql_input_value!($value)) , $($rest)*
        };
    };

    // Last value is an expression with no trailing comma.
    (@object $object:ident ($($key:tt)+) (: $value:expr) $copy:tt) => {
        $crate::graphql::vars! {
            @object $object
            [$($key)+]
            ($crate::graphql_input_value!($value))
        };
    };

    // Missing value for last entry. Trigger a reasonable error message.
    (@object $object:ident ($($key:tt)+) (:) $copy:tt) => {
        // "unexpected end of macro invocation"
        $crate::graphql::vars! {};
    };

    // Missing colon and value for last entry. Trigger a reasonable error
    // message.
    (@object $object:ident ($($key:tt)+) () $copy:tt) => {
        // "unexpected end of macro invocation"
        $crate::graphql::vars! {};
    };

    // Misplaced colon. Trigger a reasonable error message.
    (@object $object:ident () (: $($rest:tt)*) ($colon:tt $($copy:tt)*)) => {
        // Takes no arguments so "no rules expected the token `:`".
        $crate::graphql::vars! {@unexpected $colon};
    };

    // Found a comma inside a key. Trigger a reasonable error message.
    (@object $object:ident ($($key:tt)*) (, $($rest:tt)*) ($comma:tt $($copy:tt)*)) => {
        // Takes no arguments so "no rules expected the token `,`".
        $crate::graphql::vars! {@unexpected $comma};
    };

    // Key is fully parenthesized. This avoids clippy double_parens false
    // positives because the parenthesization may be necessary here.
    (@object $object:ident () (($key:expr) : $($rest:tt)*) $copy:tt) => {
        $crate::graphql::vars! {
            @object $object ($key) (: $($rest)*) (: $($rest)*)
        };
    };

    // Refuse to absorb colon token into key expression.
    (@object $object:ident ($($key:tt)*) (: $($unexpected:tt)+) $copy:tt) => {
        $crate::graphql::vars! {@unexpected $($unexpected)+};
    };

    // Munch a token into the current key.
    (@object $object:ident ($($key:tt)*) ($tt:tt $($rest:tt)*) $copy:tt) => {
        $crate::graphql::vars! {
            @object $object
            ($($key)* $tt)
            ($($rest)*) ($($rest)*)
        };
    };

    ////////////
    // Errors //
    ////////////

    (@unexpected) => {};

    //////////////
    // Defaults //
    //////////////

    () => {{ $crate::graphql::Variables::<_>::new() }};

    ( $($map:tt)+ ) => {{
        let mut object = $crate::graphql::Variables::<_>::new();
        $crate::graphql::vars! {@object object () ($($map)*) ($($map)*)};
        object
    }};
}

#[doc(inline)]
pub(super) use vars;

#[cfg(test)]
mod tests {
    use indexmap::{indexmap, IndexMap};

    use crate::graphql;

    use super::vars;

    type V = graphql::Variables;

    type IV = graphql::InputValue;

    #[test]
    fn empty() {
        assert_eq!(vars! {}, V::new());
    }

    #[test]
    fn scalar() {
        let val = 42;

        assert_eq!(
            vars! {"key": 123},
            vec![("key".to_owned(), IV::scalar(123))]
                .into_iter()
                .collect::<V>(),
        );
        assert_eq!(
            vars! {"key": "val"},
            vec![("key".to_owned(), IV::scalar("val"))]
                .into_iter()
                .collect::<V>(),
        );
        assert_eq!(
            vars! {"key": 1.23},
            vec![("key".to_owned(), IV::scalar(1.23))]
                .into_iter()
                .collect::<V>(),
        );
        assert_eq!(
            vars! {"key": 1 + 2},
            vec![("key".to_owned(), IV::scalar(3))]
                .into_iter()
                .collect(),
        );
        assert_eq!(
            vars! {"key": false},
            vec![("key".to_owned(), IV::scalar(false))]
                .into_iter()
                .collect::<V>(),
        );
        assert_eq!(
            vars! {"key": (val)},
            vec![("key".to_owned(), IV::scalar(42))]
                .into_iter()
                .collect::<V>(),
        );
    }

    #[test]
    fn r#enum() {
        assert_eq!(
            vars! {"key": ENUM},
            vec![("key".to_owned(), IV::enum_value("ENUM"))]
                .into_iter()
                .collect::<V>(),
        );
        assert_eq!(
            vars! {"key": lowercase},
            vec![("key".to_owned(), IV::enum_value("lowercase"))]
                .into_iter()
                .collect::<V>(),
        );
    }

    #[test]
    fn variable() {
        assert_eq!(
            vars! {"key": @var},
            vec![("key".to_owned(), IV::variable("var"))]
                .into_iter()
                .collect::<V>(),
        );
        assert_eq!(
            vars! {"key": @array},
            vec![("key".to_owned(), IV::variable("array"))]
                .into_iter()
                .collect::<V>(),
        );
        assert_eq!(
            vars! {"key": @object},
            vec![("key".to_owned(), IV::variable("object"))]
                .into_iter()
                .collect::<V>(),
        );
    }

    #[test]
    fn list() {
        let val = 42;

        assert_eq!(
            vars! {"key": []},
            vec![("key".to_owned(), IV::list(vec![]))]
                .into_iter()
                .collect::<V>(),
        );

        assert_eq!(
            vars! {"key": [null]},
            vec![("key".to_owned(), IV::list(vec![IV::Null]))]
                .into_iter()
                .collect::<V>(),
        );

        assert_eq!(
            vars! {"key": [1]},
            vec![("key".to_owned(), IV::list(vec![IV::scalar(1)]))]
                .into_iter()
                .collect::<V>(),
        );
        assert_eq!(
            vars! {"key": [1 + 2]},
            vec![("key".to_owned(), IV::list(vec![IV::scalar(3)]))]
                .into_iter()
                .collect::<V>(),
        );
        assert_eq!(
            vars! {"key": [(val)]},
            vec![("key".to_owned(), IV::list(vec![IV::scalar(42)]))]
                .into_iter()
                .collect::<V>(),
        );

        assert_eq!(
            vars! {"key": [ENUM]},
            vec![("key".to_owned(), IV::list(vec![IV::enum_value("ENUM")]))]
                .into_iter()
                .collect::<V>(),
        );
        assert_eq!(
            vars! {"key": [lowercase]},
            vec![(
                "key".to_owned(),
                IV::list(vec![IV::enum_value("lowercase")])
            )]
            .into_iter()
            .collect::<V>(),
        );

        assert_eq!(
            vars! {"key": [@var]},
            vec![("key".to_owned(), IV::list(vec![IV::variable("var")]))]
                .into_iter()
                .collect::<V>(),
        );
        assert_eq!(
            vars! {"key": [@array]},
            vec![("key".to_owned(), IV::list(vec![IV::variable("array")]))]
                .into_iter()
                .collect::<V>(),
        );
        assert_eq!(
            vars! {"key": [@object]},
            vec![("key".to_owned(), IV::list(vec![IV::variable("object")]))]
                .into_iter()
                .collect::<V>(),
        );

        assert_eq!(
            vars! {"key": [1, [2], 3]},
            vec![(
                "key".to_owned(),
                IV::list(vec![
                    IV::scalar(1),
                    IV::list(vec![IV::scalar(2)]),
                    IV::scalar(3),
                ]),
            )]
            .into_iter()
            .collect::<V>(),
        );
        assert_eq!(
            vars! {"key": [1, [2 + 3], 3]},
            vec![(
                "key".to_owned(),
                IV::list(vec![
                    IV::scalar(1),
                    IV::list(vec![IV::scalar(5)]),
                    IV::scalar(3),
                ]),
            )]
            .into_iter()
            .collect::<V>(),
        );
        assert_eq!(
            vars! {"key": [1, [ENUM], (val)]},
            vec![(
                "key".to_owned(),
                IV::list(vec![
                    IV::scalar(1),
                    IV::list(vec![IV::enum_value("ENUM")]),
                    IV::scalar(42),
                ]),
            )]
            .into_iter()
            .collect::<V>(),
        );
        assert_eq!(
            vars! {"key": [1 + 2, [(val)], @val]},
            vec![(
                "key".to_owned(),
                IV::list(vec![
                    IV::scalar(3),
                    IV::list(vec![IV::scalar(42)]),
                    IV::variable("val"),
                ]),
            )]
            .into_iter()
            .collect::<V>(),
        );
        assert_eq!(
            vars! {"key": [1, [@val], ENUM]},
            vec![(
                "key".to_owned(),
                IV::list(vec![
                    IV::scalar(1),
                    IV::list(vec![IV::variable("val")]),
                    IV::enum_value("ENUM"),
                ]),
            )]
            .into_iter()
            .collect::<V>(),
        );
    }

    #[test]
    fn object() {
        let val = 42;

        assert_eq!(
            vars! {"key": {}},
            vec![("key".to_owned(), IV::object(IndexMap::<String, _>::new()))]
                .into_iter()
                .collect::<V>(),
        );

        assert_eq!(
            vars! {"key": {"key": null}},
            vec![("key".to_owned(), IV::object(indexmap! {"key" => IV::Null}))]
                .into_iter()
                .collect::<V>(),
        );

        assert_eq!(
            vars! {"key": {"key": 123}},
            vec![(
                "key".to_owned(),
                IV::object(indexmap! {"key" => IV::scalar(123)}),
            )]
            .into_iter()
            .collect::<V>(),
        );
        assert_eq!(
            vars! {"key": {"key": 1 + 2}},
            vec![(
                "key".to_owned(),
                IV::object(indexmap! {"key" => IV::scalar(3)}),
            )]
            .into_iter()
            .collect::<V>(),
        );
        assert_eq!(
            vars! {"key": {"key": (val)}},
            vec![(
                "key".to_owned(),
                IV::object(indexmap! {"key" => IV::scalar(42)}),
            )]
            .into_iter()
            .collect::<V>(),
        );

        assert_eq!(
            vars! {"key": {"key": []}},
            vec![(
                "key".to_owned(),
                IV::object(indexmap! {"key" => IV::list(vec![])}),
            )]
            .into_iter()
            .collect::<V>(),
        );
        assert_eq!(
            vars! {"key": {"key": [null]}},
            vec![(
                "key".to_owned(),
                IV::object(indexmap! {"key" => IV::list(vec![IV::Null])}),
            )]
            .into_iter()
            .collect::<V>(),
        );
        assert_eq!(
            vars! {"key": {"key": [1]}},
            vec![(
                "key".to_owned(),
                IV::object(indexmap! {"key" => IV::list(vec![IV::scalar(1)])}),
            )]
            .into_iter()
            .collect::<V>(),
        );
        assert_eq!(
            vars! {"key": {"key": [1 + 2]}},
            vec![(
                "key".to_owned(),
                IV::object(indexmap! {"key" => IV::list(vec![IV::scalar(3)])}),
            )]
            .into_iter()
            .collect::<V>(),
        );
        assert_eq!(
            vars! {"key": {"key": [(val)]}},
            vec![(
                "key".to_owned(),
                IV::object(indexmap! {"key" => IV::list(vec![IV::scalar(42)])}),
            )]
            .into_iter()
            .collect::<V>(),
        );
        assert_eq!(
            vars! {"key": {"key": ENUM}},
            vec![(
                "key".to_owned(),
                IV::object(indexmap! {"key" => IV::enum_value("ENUM")}),
            )]
            .into_iter()
            .collect::<V>(),
        );
        assert_eq!(
            vars! {"key": {"key": lowercase}},
            vec![(
                "key".to_owned(),
                IV::object(indexmap! {"key" => IV::enum_value("lowercase")}),
            )]
            .into_iter()
            .collect::<V>(),
        );
        assert_eq!(
            vars! {"key": {"key": @val}},
            vec![(
                "key".to_owned(),
                IV::object(indexmap! {"key" => IV::variable("val")}),
            )]
            .into_iter()
            .collect::<V>(),
        );
        assert_eq!(
            vars! {"key": {"key": @array}},
            vec![(
                "key".to_owned(),
                IV::object(indexmap! {"key" => IV::variable("array")}),
            )]
            .into_iter()
            .collect::<V>(),
        );
        assert_eq!(
            vars! {
                "inner": {
                    "key1": (val),
                    "key2": "val",
                    "key3": [{
                        "inner": 42,
                    }, {
                        "inner": ENUM,
                        "even-more": {
                            "var": @var,
                        },
                    }],
                    "key4": [1, ["val", 1 + 3], null, @array],
                },
                "more": @var,
            },
            vec![
                (
                    "inner".to_owned(),
                    IV::object(indexmap! {
                        "key1" => IV::scalar(42),
                        "key2" => IV::scalar("val"),
                        "key3" => IV::list(vec![
                            IV::object(indexmap! {
                                "inner" => IV::scalar(42),
                            }),
                            IV::object(indexmap! {
                                "inner" => IV::enum_value("ENUM"),
                                "even-more" => IV::object(indexmap! {
                                    "var" => IV::variable("var"),
                                }),
                            }),
                        ]),
                        "key4" => IV::list(vec![
                            IV::scalar(1),
                            IV::list(vec![
                                IV::scalar("val"),
                                IV::scalar(4),
                            ]),
                            IV::Null,
                            IV::variable("array"),
                        ]),
                    }),
                ),
                ("more".to_owned(), IV::variable("var")),
            ]
            .into_iter()
            .collect::<V>(),
        );
    }
}