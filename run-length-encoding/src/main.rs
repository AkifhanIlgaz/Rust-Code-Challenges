mod run_length_encoding {
    pub fn encode(text: &str) -> String {
        let mut encoded = String::new();
        let mut chars = text.chars();
        let mut prev: Option<char> = None;
        let mut count = 0;

        while let Some(c) = chars.next() {
            if prev.is_none() {
                prev = Some(c);
            }

            let prev_ = prev.unwrap();

            if prev_ != c || count == 9 {
                encoded += &format!("{}{}", count, prev_);
                count = 0;
                prev = Some(c);
            }
            count += 1;
        }

        if prev.is_some() {
            encoded += &format!("{}{}", count, prev.unwrap());
        }

        encoded
    }

    pub fn decode(text: &str) -> String {
        let mut result = String::new();

        for i in (0..text.len() - 1).step_by(2) {
            let pair = &text[i..i + 2];
            let (count, ch) = pair.split_at(1);
            result.push_str(ch.repeat(count.parse().unwrap()).as_str());
        }

        return result;

        /*
        let mut encoded = String::with_capacity(text.len() * 2);
        let mut chars = text.chars();

        while let (Some(count), Some(ch)) = (chars.next(), chars.next()) {
            let n = count.to_digit(10).unwrap();
            for _ in 0..n {
                encoded.push(ch);
            }
        }

        encoded
        */
    }
}

fn main() {
    println!("{}", run_length_encoding::decode("5A1 9A1A1 9A9A2A"));
}

#[test]
fn abc() {
    use run_length_encoding::*;

    assert_eq!(encode("abc"), "1a1b1c");
}

#[test]
fn round_trip() {
    use run_length_encoding::*;

    let input = "LinkedIn";
    println!("{}", encode(input));
    assert_eq!(decode(&encode(input)), input);
}

#[test]
fn long_run() {
    use run_length_encoding::*;

    let input = "AAAAA AAAAAAAAAA AAAAAAAAAAAAAAAAAAAA";
    assert_eq!(encode(input), "5A1 9A1A1 9A9A2A");
}
