#[macro_export]
macro_rules! dict {
    ( $($name:expr => $value:expr),+ ) => {
        {
        let mut hm = HashMap::new();
        $(
            hm.insert($name, $value);
        )*
        hm
        }
    }
}
