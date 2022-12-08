pub fn split_line<'a>(string: &'a str, pattern: &str) -> Result<Vec<&'a str>, &'static str> {
    let mut results = vec![];
    let mut last_idx = 0;

    let mut first = true;
    let splits: Vec<&str> = pattern.split("{}").collect();
    for i in 0..splits.len() {
        if splits[i].chars().count() == 0 {
            if i + 1 == splits.len() {
                results.push(&string[last_idx..]);
                continue;
            } else if i != 0 {
                return Err("Invalid pattern, found consecutive captures {}{}.");
            }
        }
        match string.get(last_idx..).and_then(|s| s.find(splits[i])) {
            Some(idx) => {
                let idx = last_idx + idx;
                if !first {
                    results.push(&string[last_idx..idx]);
                }
                last_idx = idx + splits[i].chars().count();
                first = false;
            }
            None => return Err("Could not match pattern to string."),
        }
    }

    Ok(results)
}

/// parseln!(text, pattern, variables...)
///
/// A println! counterpart for simple parsing problems.
///
/// It uses a pattern where {} captures something, and is parsed to the type of the supplied variable.
/// If two consecutive captures are included in the patter the call will panic.
/// If too few variables are supplied the remaining captures are dropped, if too many are supplied the call will panic.
///
/// ## Example
/// It can be used either with already defined variables as
/// ```rust
/// # use parseline::parseln;
/// let month: String;
/// let day: isize;
/// parseln!("Date: apr 13", "Date: {} {}", month, day);
/// assert_eq!((month, day), (String::from("apr"), 13))
/// ```
/// or by generating new binding, though then we need to supply the type to be parsed
/// ```rust
/// # use parseline::parseln;
/// parseln!("Date: apr 13", "Date: {} {}", month: String, day: i32);
/// assert_eq!((month, day), (String::from("apr"), 13))
/// ```
///
/// Currently it is not possible to mix these methods.
#[macro_export]
macro_rules! parseln {
    ($line:expr, $pattern:expr) => {
        $crate::split_line($line, $pattern).expect("Failed to parse")
    };
    ($line:expr, $pattern:expr, $($var:ident),+) => {
        {
            let result = match $crate::split_line($line, $pattern) {
                Ok(x) => x,
                Err(e) => panic!("Parsing error: {}", e),
            };
            let mut result_iter = result.iter();
            $(
                $var = result_iter.next().expect("Too many variables").parse().expect("Incorrect type for captured variable");
            )+
        }
    };
    ($line:expr, $pattern:expr, $($var:ident:$type:ty),+) => {
        $(let $var: $type);+;
        parseln!($line, $pattern, $($var),+);
        /*
        let ($($var),+) = { // why does result not get overwritten even without this?
            let result = match $crate::split_line($line, $pattern) {
                Ok(x) => x,
                Err(e) => panic!("Parsing error: {}", e),
            };
            let mut result_iter = result.iter();
            $(
                let $var: $type = result_iter.next().expect("Too many variables").parse().expect("Incorrect type for captured variable");
            )+
            ($($var),+)
        };*/
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_line_empty_corners() {
        let result = split_line("@23x", "{}@{}x{}").unwrap();
        assert_eq!(result, vec!["", "23", ""]);
    }

    #[test]
    fn test_split_line_with_corners() {
        let result = split_line("a@23x*", "{}@{}x{}").unwrap();
        assert_eq!(result, vec!["a", "23", "*"]);
    }

    #[test]
    fn test_macro_vec() {
        let result = parseln!("#1 @ c,1.44: firstxtrue", "#{} @ {},{}: {}x{}");
        assert_eq!(result, vec!["1", "c", "1.44", "first", "true"]);
    }

    #[test]
    fn test_macro_external_vars() {
        let a: u32;
        let b: char;
        let c: f32;
        let d: String;
        let e: bool;
        parseln!(
            "#1 @ c,1.44: firstxtrue",
            "#{} @ {},{}: {}x{}",
            a,
            b,
            c,
            d,
            e
        );

        assert_eq!(a, 1);
        assert_eq!(b, 'c');
        assert_eq!(c, 1.44);
        assert_eq!(d, "first");
        assert!(e);
    }

    #[test]
    fn test_macro_inside_vars() {
        parseln!(
            "#1 @ c,1.44: firstxtrue",
            "#{} @ {},{}: {}x{}",
            a: i32,
            b: char,
            c: f32,
            d: String,
            e: bool
        );

        assert_eq!(a, 1);
        assert_eq!(b, 'c');
        assert_eq!(c, 1.44);
        assert_eq!(d, "first");
        assert!(e);
    }

    #[test]
    fn test_macro_text_var() {
        let text = "1@a^";
        parseln!(text, "{}@{}^{}", a: i32, b: char, c: String);

        assert_eq!(a, 1);
        assert_eq!(b, 'a');
        assert_eq!(c, "");
    }

    #[test]
    fn test_macro_not_overwrite() {
        let result = "hello";
        parseln!("1@a^", "{}@{}^{}", _a: u32, _b: char, _c: String);

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
    #[should_panic(expected = "Invalid pattern, found consecutive captures {}{}")]
    fn test_macro_double_capture() {
        parseln!("1@a^", "{}@{}{}^{}", _a: u32, _b: char, _c: String);
    }

    #[test]
    #[should_panic(expected = "Too many variables")]
    fn test_macro_panic_too_many_variables() {
        parseln!("1@a^", "{}@{}^{}", _a: u32, _b: char, _c: String, _d: bool);
    }

    #[test]
    #[should_panic(expected = "Incorrect type for captured variable")]
    fn test_macro_panic_incorrect_variable_type() {
        parseln!("1@a^", "{}@{}^{}", _a: u32, _b: bool, _c: String);
    }

    #[test]
    #[should_panic(expected = "Could not match pattern to string")]
    fn test_macro_panic_parse() {
        parseln!("1@a^", "{}#{}^{}", _a: u32, _b: bool, _c: String);
    }
}
