// iterators2.rs
// In this exercise, you'll learn some of the unique advantages that iterators
// can offer. Follow the steps to complete the exercise.
// As always, there are hints if you execute `rustlings hint iterators2`!



// Step 1.
// Complete the `capitalize_first` function.
// "hello" -> "Hello"
pub fn capitalize_first(input: &str) -> String {
    // Method 1
    let mut c = input.chars();
    match c.next() {
        None => String::new(),
        Some(first) => {
	    let fc = first.to_ascii_uppercase();
	    let mut rest = c.collect::<String>();  // iter -> String
	    rest.insert(0,fc);
	    rest
	}
    }

    // Method 2
    // if input.len() == 0 {
    // 	return String::from("");
    // }
    // let mut s = input.to_string();
    // s[0..1].make_ascii_uppercase();
    // println!("{}", s);
    // s
}

// Step 2.
// Apply the `capitalize_first` function to a slice of string slices.
// Return a vector of strings.
// ["hello", "world"] -> ["Hello", "World"]
pub fn capitalize_words_vector(words: &[&str]) -> Vec<String> {
    let mut ret = vec![];
    for w in words.iter() {
	let cw = capitalize_first(w);
	ret.push(cw)
    }
    ret
}

// Step 3.
// Apply the `capitalize_first` function again to a slice of string slices.
// Return a single string.
// ["hello", " ", "world"] -> "Hello World"
pub fn capitalize_words_string(words: &[&str]) -> String {
    let mut ret = vec![];
    for w in words.iter() {
	let cw = capitalize_first(w);
	ret.push(cw)
    }
    for w in ret.iter() {
    }
    ret.join("")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        assert_eq!(capitalize_first("hello"), "Hello");
    }

    #[test]
    fn test_empty() {
        assert_eq!(capitalize_first(""), "");
    }

    #[test]
    fn test_iterate_string_vec() {
        let words = vec!["hello", "world"];
        assert_eq!(capitalize_words_vector(&words), ["Hello", "World"]);
    }

    #[test]
    fn test_iterate_into_string() {
        let words = vec!["hello", " ", "world"];
        assert_eq!(capitalize_words_string(&words), "Hello World");
    }
}
