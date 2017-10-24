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

// Thanks Alan Malloy
macro_rules! wait_for_borrow {
    ($cell:expr) => ({
        let mut result = None;

        while result.is_none() {
            if let Ok(borrow) = $cell.try_borrow_mut() {
                result = Some(borrow);
            }
        }

        result.unwrap()
    });
}
