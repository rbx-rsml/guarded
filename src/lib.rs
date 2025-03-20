#[macro_export]
macro_rules! guarded_unwrap {
    (@inner $expr:expr, $none_case:expr) => {
        match $expr {
            Some(value) => value,
            None => $none_case,
        }
    };

    ($expr:expr, return $ret:expr) => {
        guarded_unwrap!(@inner $expr, { return $ret; })
    };

    ($expr:expr, return) => {
        guarded_unwrap!(@inner $expr, { return; })
    };

    ($expr:expr, break $ret:expr) => {
        guarded_unwrap!(@inner $expr, { break $ret; })
    };

    ($expr:expr, break) => {
        guarded_unwrap!(@inner $expr, { break; })
    };

    ($expr:expr, continue $ret:expr) => {
        guarded_unwrap!(@inner $expr, { continue $ret; })
    };

    ($expr:expr, continue) => {
        guarded_unwrap!(@inner $expr, { continue; })
    };
}