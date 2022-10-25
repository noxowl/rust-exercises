#[cfg(feature = "parallel")]
pub fn parallel() {
    println!("parallel is enabled");
}

#[cfg(feature = "serde")]
pub fn serde() {
    println!("serde is enabled");
}

#[cfg(feature = "special")]
pub fn special() {
    println!("special is enabled");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(feature = "parallel")]
    #[test]
    fn test_parallel() {
        parallel();
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_serde() {
        serde(); // not found in this scope in compile. because we didn't add #[cfg(feature = "parallel)"]
    }

    // #[test]
    // fn test_special() {
    //     special(); // not found in this scope in compile. because we didn't add #[cfg(feature = "parallel)"]
    // }
    #[cfg(feature = "special")]
    #[test]
    fn test_special() {
        special(); // not found in this scope in compile. because we didn't add #[cfg(feature = "parallel)"]
    }
}
