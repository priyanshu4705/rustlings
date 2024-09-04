fn bigger(a: i32, b: i32) -> i32 {
    match a > b {
        true => a,
        false => b,
    }
}

fn main() {
    // You can optionally experiment here.
    println!("{}", bigger(10, 8));
}

// Don't mind this for now :)
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ten_is_bigger_than_eight() {
        assert_eq!(10, bigger(10, 8));
    }

    #[test]
    fn fortytwo_is_bigger_than_thirtytwo() {
        assert_eq!(42, bigger(32, 42));
    }

    #[test]
    fn equal_numbers() {
        assert_eq!(42, bigger(42, 42));
    }
}
