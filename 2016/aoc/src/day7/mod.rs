use regex::Regex;

pub fn do_day7(input: &str) {
    let mut tls_lines = 0;
    let mut ssl_lines = 0;

    for line in input.lines() {
        if supports_tls(line) {
            tls_lines += 1;
        }
        if supports_ssl(line) {
            ssl_lines += 1;
        }
    }

    println!("{} support TLS", tls_lines);
    println!("{} support SSL", ssl_lines);
}

fn supports_tls(ip: &str) -> bool {
    contains_abba(ip) && extract_hypernet_sequences(ip).iter().map(|h| !contains_abba(h)).all(|b| b)
}

fn supports_ssl(ip: &str) -> bool {
    let nonhypernet = extract_nonhypernet(ip);

    match contains_aba(&nonhypernet) {
        None => false,
        Some(abas) => {
            let hypernets = extract_hypernet_sequences(ip);
            hypernets.iter().any(|h| abas.iter().any(|&(a, b)| contains_xyx(h, &(a, b))))
        }
    }
}

fn extract_hypernet_sequences(source: &str) -> Vec<&str> {
    lazy_static! {
        static ref MATCH_BRACKETED: Regex = Regex::new(r"(\[(.*?)\])").unwrap();
    }

    MATCH_BRACKETED.captures_iter(source).map(|c| c.at(2).unwrap()).collect()
}

fn extract_nonhypernet(source: &str) -> String {
    lazy_static! {
        static ref MATCH_BRACKETED: Regex = Regex::new(r"(\[.*?\])").unwrap();
    }

    MATCH_BRACKETED.replace_all(source, "")
}

fn contains_abba(haystack: &str) -> bool {
    for i in 0..haystack.len() - 2 {
        // not enough room left in the string
        if i + 3 >= haystack.len() {
            return false;
        }

        let c0 = haystack.chars().nth(i).unwrap();
        let c1 = haystack.chars().nth(i + 1).unwrap();

        if c0 == c1 {
            // can't possibly be an abba
            continue;
        }

        let c2 = haystack.chars().nth(i + 2).unwrap();
        if c2 != c1 {
            // can't be an abba
            continue;
        }

        let c3 = haystack.chars().nth(i + 3).unwrap();
        if c3 == c0 {
            return true;
        }
    }

    false
}

fn contains_aba(haystack: &str) -> Option<Vec<(char, char)>> {
    let mut abas = Vec::new();

    for i in 0..haystack.len() - 2 {
        // not enough room left in the string
        if i + 2 >= haystack.len() {
            return None;
        }

        let c0 = haystack.chars().nth(i).unwrap();
        let c1 = haystack.chars().nth(i + 1).unwrap();

        if c0 == c1 {
            // can't possibly be an abba
            continue;
        }

        let c2 = haystack.chars().nth(i + 2).unwrap();
        if c2 == c0 {
            abas.push((c0, c1));
        }
    }

    if abas.len() > 0 { Some(abas) } else { None }
}

fn contains_xyx(haystack: &str, cs: &(char, char)) -> bool {
    for i in 0..haystack.len() - 2 {
        if i + 2 >= haystack.len() {
            return false;
        }

        let c0 = haystack.chars().nth(i).unwrap();
        if c0 != cs.1 {
            continue;
        }

        let c1 = haystack.chars().nth(i + 1).unwrap();
        if c1 != cs.0 {
            continue;
        }

        let c2 = haystack.chars().nth(i + 2).unwrap();
        if c2 == cs.1 {
            return true;
        }
    }

    false
}

#[test]
fn test_contains_abba() {
    assert!(contains_abba("abba"));
    assert!(!contains_abba("aaaa"));
}

#[test]
fn test_contains_aba() {
    assert_eq!(contains_aba("aba"), Some(vec![('a', 'b')]));
    assert_eq!(contains_aba("aaaa"), None);
    assert_eq!(contains_aba("zazbz"), Some(vec![('z', 'a'), ('z', 'b')]));
}

#[test]
fn test_contains_xyx() {
    assert_eq!(contains_xyx("aba", &('b', 'a')), true);
    assert_eq!(contains_xyx("aaaa", &('b', 'a')), false);
}

#[test]
fn test_extract_hypernet() {
    assert_eq!(extract_hypernet_sequences("aaa").len(), 0);
    assert_eq!(extract_hypernet_sequences("a[aa").len(), 0);
    assert_eq!(extract_hypernet_sequences("a[a]a"), ["a"]);
    assert_eq!(extract_hypernet_sequences("a[a][b]a"), ["a", "b"]);
    assert_eq!(extract_hypernet_sequences("a[a]yyys[b]a"), ["a", "b"]);
}

#[test]
fn test_extract_nonhypernet() {
    assert_eq!(extract_nonhypernet("aaa"), "aaa");
    assert_eq!(extract_nonhypernet("a[aa"), "a[aa");
    assert_eq!(extract_nonhypernet("a[a]a"), "aa");
    assert_eq!(extract_nonhypernet("a[a][b]a"), "aa");
    assert_eq!(extract_nonhypernet("a[a]yyys[b]a"), "ayyysa");
}

#[test]
fn test_supports_tls() {
    assert!(supports_tls("abba[mnop]qrst"));
    assert!(!supports_tls("abcd[bddb]xyyx"));
    assert!(!supports_tls("aaaa[qwer]tyui"));
    assert!(supports_tls("ioxxoj[asdfgh]zxcvbn"));
}

#[test]
fn test_supports_ssl() {
    assert!(supports_ssl("aba[bab]xyz"));
    assert!(!supports_ssl("sys[sys]sys"));
    assert!(supports_ssl("aaa[kek]eke"));
    assert!(supports_ssl("zazbz[bzb]cdb"));
}
