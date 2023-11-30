#[macro_export]
macro_rules! bool_xor {
    ($x:expr, $y:expr) => {
        ($x && !$y) || ($y && !$x)
    };
}
