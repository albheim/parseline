/// Not allowed to be multiple {}{} in a row, unclear
/// 
pub fn split_line<'a>(string: &'a str, pattern: &str) -> Result<Vec<&'a str>, &'static str> {
    let mut results = vec![];
    let mut last_idx = 0;

    let mut first = true;
    for pat in pattern.split("{}") {
        if pat.chars().count() == 0 && last_idx != 0 {
            results.push(&string[last_idx..]);
            continue
        }
        match string.get(last_idx..).and_then(|s| s.find(pat)) {
            Some(idx) => {
                let idx = last_idx + idx;
                if !first {
                    results.push(&string[last_idx..idx]);
                }
                last_idx = idx + pat.chars().count();
                first = false;
            },
            None => return Err("Could not match pattern to string."),
        }
    }

    Ok(results) 
}

#[macro_export]
macro_rules! parseln {
    ($line:expr, $pattern:expr) => {
        $crate::split_line($line, $pattern).expect("Failed to parse")
    };
    ($line:expr, $pattern:expr, $($var:ident),+) => {
        {
            let result = $crate::split_line($line, $pattern).expect("Failed to parse");
            let mut result_iter = result.iter();
            $(
                $var = result_iter.next().expect("Too many variables").parse().expect("Incorrect type for captured variable");
            )+
        }
    };
    ($line:expr, $pattern:expr, $($var:ident:$type:ty),+) => {
        let ($($var),+) = { // why does result not get overwritten even without this?
            let result = $crate::split_line($line, $pattern).expect("Failed to parse");
            let mut result_iter = result.iter();
            $(
                let $var: $type = result_iter.next().expect("Too many variables").parse().expect("Incorrect type for captured variable");
            )+
            ($($var),+)
        };
    };
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_line() {
        let result = split_line("@23x", "{}@{}x{}").unwrap();
        assert_eq!(result, vec!["", "23", ""]);
    }

    #[test]
    fn test_split_line2() {
        let result = split_line("#1 @ 23,45: 43x56", "#{} @ {},{}: {}x{}").unwrap();
        assert_eq!(result, vec!["1", "23", "45", "43", "56"]);
    }

    #[test]
    fn test_macro() {
        let result = parseln!("#1 @ 23,45: 43x56", "#{} @ {},{}: {}x{}");
        assert_eq!(result, vec!["1", "23", "45", "43", "56"]);
    }

    #[test]
    fn test_macro_external_vars() {
        let a: u32;
        let b: char;
        let c: f32;
        let d: String;
        let e: bool;
        parseln!("#1 @ c,1.44: firstxtrue", "#{} @ {},{}: {}x{}", a, b, c, d, e);

        assert_eq!(a, 1);
        assert_eq!(b, 'c');
        assert_eq!(c, 1.44);
        assert_eq!(d, "first");
        assert_eq!(e, true);
    }

    #[test]
    fn test_macro_text_var() {
        let text = "#1 @ c,1.44: firstxtrue";
        parseln!(text, "#{} @ {},{}: {}x{}", a: i32, b: char, c: f32, d: String, e: bool);

        assert_eq!(a, 1);
        assert_eq!(b, 'c');
        assert_eq!(c, 1.44);
        assert_eq!(d, "first");
        assert_eq!(e, true);
    }

    #[test]
    fn test_macro_types() {
        parseln!("#1 @ c,1.44: firstxtrue", "#{} @ {},{}: {}x{}", a: u32, b: char, c: f32, d: String, e: bool);

        assert_eq!(a, 1);
        assert_eq!(b, 'c');
        assert_eq!(c, 1.44);
        assert_eq!(d, "first");
        assert_eq!(e, true);
    }

    #[test]
    fn test_macro_not_overwrite() {
        let result = "hello";
        parseln!("#1 @ c,1.44: firstxtrue", "#{} @ {},{}: {}x{}", a: u32, b: char, c: f32, d: String, e: bool);

        assert_eq!(result, "hello");
    }

    /*/
    #[test]
    #[ignore = "not yet implemented"]
    fn test_macro_underscore() {
        parseln!("#1 @ c,1.44: firstxtrue", "#{} @ {},{}: {}x{}", a: u32, _, c: f32, d: String, e: bool);

        assert_eq!(a, 1);
        assert_eq!(c, 1.44);
        assert_eq!(d, "first");
        assert_eq!(e, true);
    }
    */

    #[test]
    #[should_panic(expected = "Too many variables")]
    fn test_macro_panic_too_many_variables() {
        parseln!("#1 @ c,1.44: firstxtrue", "#{} @ {},{}: {}x{}", a: u32, b: char, c: f32, d: String, e: bool, f: i32);
    }

    #[test]
    #[should_panic(expected = "Incorrect type for captured variable")]
    fn test_macro_panic_incorrect_variable_type() {
        parseln!("#1 @ c,1.44: firstxtrue", "#{} @ {},{}: {}x{}", a: u32, b: char, c: f32, d: String, e: i32);
    }

    #[test]
    #[should_panic(expected = "Failed to parse")]
    fn test_macro_panic_parse() {
        parseln!("#1  c,1.44: firstxtrue", "#{} @ {},{}: {}x{}", a: u32, b: char, c: f32, d: String);
    }
}
