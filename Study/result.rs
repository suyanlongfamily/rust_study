use std::num::ParseIntError;

fn double_number(number_str: &str) -> Result<i32,ParseIntError> {
    // 2 * number_str.parse::<i32>().unwrap()
    number_str.parse::<i32>().map(|n| 2*n )    
}



// fn main() {
//     let n: i32 = double_number("10");
//     assert_eq!(n, 20);
// }

fn find(haystack: &str, needle: char) -> Option<usize> {
    for (offset, c) in haystack.char_indices() {
        if c == needle {
            return Some(offset);
        }
    }
    None
}

fn extension_explicit(file_name: &str) -> Option<&str> {
    match find(file_name, '.') {
        None => None,
        Some(i) => Some(&file_name[i + 1..]),
    }

}

fn extension(file_name: &str) -> Option<&str> {
    find(file_name, '.').map(|i| &file_name[i + 1..])
}

fn main() {
    match extension_explicit("foo.rs") {
        None => println!("no extension"),
        Some(ext) => assert_eq!(ext, "rs"),
    }

    assert_eq!(extension("foo.rs").unwrap_or("rs"), "rs");
    assert_eq!(extension("bar").unwrap_or("rs"), "rs");
    
    
    

}
