#![macro_use]
#[allow(unused_macros)]
macro_rules! load_array {
    ( $size:expr; [ $( [$index:expr] => $e:expr ),* ]) => ({
        let mut array = [0; $size];
        let mut index = 0;

        $(
            array[index] = $e;
        )*

         array
    });
}
