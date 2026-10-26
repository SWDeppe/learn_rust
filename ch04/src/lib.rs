
// ex0
pub fn my_strdup(s: &str) -> String {
    let mut new = String::with_capacity(s.len());

    for c in s.chars() {
        new.push(c);
    }

    new
}

// ex1
pub fn my_trim(s: &str) -> &str {
    let mut heading: usize = 0;
    let mut tailing: usize = s.as_bytes().len();

    if s == "" { return &s }

    while s.as_bytes()[tailing - 1].is_ascii_whitespace() || s.as_bytes()[heading].is_ascii_whitespace()
    {
        if tailing - 1 <= heading { return &s[0..0]; }
        if s.as_bytes()[tailing - 1].is_ascii_whitespace() { tailing -= 1; }
        if s.as_bytes()[heading].is_ascii_whitespace() { heading += 1; }
    }


    &s[heading..tailing]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let s1 = "Hello world";
        let new = my_strdup(s1);
        assert_eq!(new, s1);

        let s1 = "héllo 🦀";
        let new = my_strdup(s1);
        assert_eq!(new, s1);

        let s1 = " hello ";
        assert_eq!(my_trim(s1), s1.trim());

        let s1 = "     héllo 🦀  ";
        assert_eq!(my_trim(s1), s1.trim());

        let s1 = "      ";
        assert_eq!(my_trim(s1), s1.trim());
    
        let s1 = " ";
        assert_eq!(my_trim(s1), s1.trim());
    
        let s1 = "";
        assert_eq!(my_trim(s1), s1.trim());
    }
}
