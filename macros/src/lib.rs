#[macro_export]
macro_rules! hashmap {
    () => {
        // Start with :: to prevent user-override
        ::std::collections::HashMap::new()
    };
    // k => v must be present at least once,
    // otherwise, a user can do hashmap!(,).
    // We handle the empty case separately.
    ( $($k:expr => $v:expr),+ $(,)? ) => {
        // Create a block so that we can have more than
        // one statements
        {
            // Start with :: to prevent user-override
            let mut hm = ::std::collections::HashMap::new();
            $(
                // insert returns the old value, if present,
                // that we ignore
                let _ = hm.insert($k, $v);
            )*
            hm
        }
    };
}
