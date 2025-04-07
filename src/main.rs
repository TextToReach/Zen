use library::{
    Environment::Environment,
    Methods::{FileAndLineInformation, Str, Throw},
    Types::{self, New, Number, ZenError, ZenType},
    Array::Array
};
mod library;
use std::{any, env};

#[allow(non_snake_case, non_upper_case_globals)]
fn main() {
    let mut a = Array::new(
        vec![
            Number::new_enum(1.0),
            Number::new_enum(2.0),
            Number::new_enum(3.0),
            Number::new_enum(4.0),
            Number::new_enum(5.0),
            Number::new_enum(6.0),
            Number::new_enum(7.0),
            Number::new_enum(8.0),
            Number::new_enum(9.0),
            Number::new_enum(10.0),
            Number::new_enum(11.0),
            Number::new_enum(12.0),
            Number::new_enum(13.0),
            Number::new_enum(14.0),
            Number::new_enum(15.0),
            Number::new_enum(16.0),
            Number::new_enum(17.0),
            Number::new_enum(18.0),
            Number::new_enum(19.0),
            Number::new_enum(20.0),
        ]
    );

    Debug!(a.get().between(0, 21));
    Debug!("selam");
    // a.push().toEnd(Number::new_enum(4.0)); // appends item (4.0) at the end of the array
    // a.push().toStart(Number::new_enum(4.0)); // appends item (4.0) at the start of the array
    // a.remove().byItem(Number::new_enum(19.0)); // removes item at index 
    // a.remove().byIndex(5) // removes item at index 5
    // a.remove.byItem(2.0); // removes item (2.0) from the list. Doesn't perform any action if item doesn't exist
    // a.remove.atEnd();
    // a.remove.atStart();
    
    // a.check.isSorted(); // checks if the array is sorted
    // a.check.filter(...);
    // a.check.includes(2.0); // true
    // a.get.asSorted(); // returns the sorted version of the array
    // a.get.length(); // 3
    // a.get.indexOf(2.0); // 1
    // a.get.between(0, 1); // 1, 2
    // a.get.splice(0, 1); // 1 (0'dan itibaren 1 element)
    // a.modify.reverse();
    // a.modify.sort(); // modifies the array and sorts the elements.
    // a.modify.replaceFirst();
    // a.modify.replaceLast();
    // a.modify.replaceAll();







}
