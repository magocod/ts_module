pub fn luhn(cc_number: &str) -> bool {
    // ANCHOR_END: luhn
    let mut digits_seen = 0;
    let mut sum = 0;
    for (i, ch) in cc_number.chars().rev().filter(|&ch| ch != ' ').enumerate() {
        match ch.to_digit(10) {
            Some(d) => {
                sum += if i % 2 == 1 {
                    let dd = d * 2;
                    dd / 10 + dd % 10
                } else {
                    d
                };
                digits_seen += 1;
            }
            None => return false,
        }
    }

    if digits_seen < 2 {
        return false;
    }

    sum % 10 == 0
}

pub fn prefix_matches(prefix: &str, request_path: &str) -> bool {
    // ANCHOR_END: prefix_matches
    let mut prefixes = prefix
        .split('/')
        .map(|p| Some(p))
        .chain(std::iter::once(None));
    let mut request_paths = request_path
        .split('/')
        .map(|p| Some(p))
        .chain(std::iter::once(None));

    for (prefix, request_path) in prefixes.by_ref().zip(&mut request_paths) {
        match (prefix, request_path) {
            (Some(prefix), Some(request_path)) => {
                if (prefix != "*") && (prefix != request_path) {
                    return false;
                }
            }
            (Some(_), None) => return false,
            (None, None) => break,
            (None, Some(_)) => break,
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    // luhn

    #[test]
    fn test_non_digit_cc_number() {
        assert!(!luhn("foo"));
    }

    #[test]
    fn test_empty_cc_number() {
        assert!(!luhn(""));
        assert!(!luhn(" "));
        assert!(!luhn("  "));
        assert!(!luhn("    "));
    }

    #[test]
    fn test_single_digit_cc_number() {
        assert!(!luhn("0"));
    }

    #[test]
    fn test_two_digit_cc_number() {
        assert!(luhn(" 0 0 "));
    }

    #[test]
    fn test_valid_cc_number() {
        assert!(luhn("4263 9826 4026 9299"));
        assert!(luhn("4539 3195 0343 6467"));
        assert!(luhn("7992 7398 713"));
    }

    #[test]
    fn test_invalid_cc_number() {
        assert!(!luhn("4223 9826 4026 9299"));
        assert!(!luhn("4539 3195 0343 6476"));
        assert!(!luhn("8273 1232 7352 0569"));
    }

    // prefix_matches

    #[test]
    fn test_matches_without_wildcard() {
        assert!(prefix_matches("/v1/publishers", "/v1/publishers"));
        assert!(prefix_matches("/v1/publishers", "/v1/publishers/abc-123"));
        assert!(prefix_matches("/v1/publishers", "/v1/publishers/abc/books"));

        assert!(!prefix_matches("/v1/publishers", "/v1"));
        assert!(!prefix_matches("/v1/publishers", "/v1/publishersBooks"));
        assert!(!prefix_matches("/v1/publishers", "/v1/parent/publishers"));
    }

    #[test]
    fn test_matches_with_wildcard() {
        assert!(prefix_matches(
            "/v1/publishers/*/books",
            "/v1/publishers/foo/books"
        ));
        assert!(prefix_matches(
            "/v1/publishers/*/books",
            "/v1/publishers/bar/books"
        ));
        assert!(prefix_matches(
            "/v1/publishers/*/books",
            "/v1/publishers/foo/books/book1"
        ));

        assert!(!prefix_matches("/v1/publishers/*/books", "/v1/publishers"));
        assert!(!prefix_matches(
            "/v1/publishers/*/books",
            "/v1/publishers/foo/booksByAuthor"
        ));
    }
}
