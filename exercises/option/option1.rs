use std::convert::TryFrom;

// option1.rs
// Make me compile! Execute `rustlings hint option1` for hints

// hard.. there might be a better way of doing this.

// you can modify anything EXCEPT for this function's sig
fn print_number(maybe_number: Option<u16>) {
    println!("printing: {}", maybe_number.unwrap());
}

fn main() {
    print_number(Some(13));
    print_number(Some(99));

    let mut numbers: [Option<u16>; 5] = Default::default();

    for iter in 0..5 {
        let number_to_add: u16 = {
            u16::try_from(((iter * 1235) + 2) / (4 * 16)).unwrap()
        };
	numbers[iter as usize] = Some(number_to_add);  
    }
}
