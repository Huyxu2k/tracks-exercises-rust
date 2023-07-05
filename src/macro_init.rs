#[macro_export]
macro_rules! add_1 {
    ($a:expr,$b:expr) => {
        $a + $b
    };
    ($a:expr) => {
        $a
    };
}
#[macro_export]
macro_rules! add_as {
    ($a:expr,$b:expr,$typ:ty) => {
        $a as $typ + $b as $typ
    };
}
#[macro_export]
macro_rules! add_multi {
    ($($a:expr),*) => {
        {
            0
            $(+$a)*
        }
    };
}
#[macro_export]
macro_rules! add {
    ($a:expr) => {
        $a
    };
    ($a:expr,$b:expr) => {
        $a+$b
    };
    ($a:expr,$($b:tt)*) => {
        $a+add!($($b)*)
    };
}