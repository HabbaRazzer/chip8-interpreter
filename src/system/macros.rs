#![macro_escape]
#[macro_use]
macro_rules! array {
    ( $size:expr; [ $( $e:expr ),* ]) => ({
        let mut array = [0; $size];
        let mut index = 0;

        $(
            array[index] = $e;
            index += 1;
        )*

         array
    });
}
