pub struct Base36(u64);

impl Base36 {
    pub fn new(num: u64) -> Self {
        Self(num)
    }
}

impl core::fmt::Display for Base36 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        let mut value = self.0;

        let mut output = heapless::String::<13>::new();

        while value > 0 {
            #[allow(
                clippy::integer_division_remainder_used,
                reason = "not timing sensitive"
            )]
            let remainder: u32 = (value % 36).try_into().expect("u64 % 36 is always a u32");

            let digit = std::char::from_digit(remainder, 36)
                .expect("remainder is always a valid base36 value");

            output
                .push(digit)
                .expect("output has space for a u64 base36 string");

            value /= 36;
        }

        write!(
            f,
            "{}",
            output.chars().rev().collect::<heapless::String::<13>>()
        )
        .expect("unhandleable");

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::Base36;

    #[test]
    fn compare_to_radix() {
        for i in 0..10000 {
            let value = Base36::new(i).to_string();

            assert!(u64::from_str_radix(&value, 36).unwrap() == i);
        }
    }
}
