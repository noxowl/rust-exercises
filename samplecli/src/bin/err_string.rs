fn get_int_from_file() -> Result<i32, String> {
    let path = "number.txt";
    let num_str = std::fs::read_to_string(path).map_err(|e| e.to_string())?;
    /// same as above (? operator)
    /// let num_str_result = std::fs::read_to_string(path).map_err(|e| e.to_string());
    /// let num_str = match num_str_result {
    ///     Ok(t) => t,
    ///     Err(e) => return Err(e)
    /// };

    num_str // init with '&str'
        .trim()  // trim whitespace. still '&str'
        .parse::<i32>() // parse '&str' to 'i32'. the return is 'Result<i32, ParseIntError>'
        .map(|t| t * 2) // if 'parse()' return 'Ok(t)', execute 't * 2' and return 'Ok(t * 2)'
        .map_err(|e| e.to_string()) // if 'parse()' return 'Err(e)', return 'Err(e as String)'
}

fn main() {
    match get_int_from_file() {
        Ok(x) => println!("{}", x),
        Err(e) => println!("{}", e)
    }
}