
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

// ex2
pub fn my_find(haystack: &str, needle: &str) -> Option<usize> {
    let mut res: usize = 0;
    let needle_siz = needle.len();

    if needle == "" {
        return Some(res);
    }

    while res + needle_siz <= haystack.as_bytes().len() {
        if &haystack.as_bytes()[res..res + needle_siz] == needle.as_bytes()
        {
            return Some(res);
        }
        res += 1;
    }

    return None;
}


pub fn my_reverse(slice: &mut [i32]) {
    let mut i = 0;
    let mut len = slice.len();

   while len - i > 2 {
        slice.swap(i, len - 1);
        i += 1;
        len -= 1;
   };
}

// doing the same is_char_bundary (juste trying to know how utf8 is doing)
fn is_char_first(char: u8) -> bool{
    // so the idee is that an ASCII character is from 0 -> 127
    // so the last byte is set to one to indicate its gonna be out of the ascii table
    // 10 -> continuation byte (middle of a character)
    // 110 -> First byte of a 2 bytes character
    // 1110 -> First byte of a 3 bytes character
    // 11110 -> First byte of a 4 bytes character
    if char >> 7 & 1 == 1 && (char >> 6) & 1 == 0 {
        return  false;
    }
    else {
        return  true;
    }
}

pub fn my_split_at(s: &str, mid: usize) -> Option<(&str, &str)> {
    if s.len() < mid || is_char_first(s.as_bytes()[mid]) == false{
        return None;
    }
    else {
        Some((&s[..mid], &s[mid..]))
    }
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



        assert_eq!(my_find("hello world", ""), Some(0));
        assert_eq!(my_find("hello world", "world"), Some(6));
        assert_eq!(my_find("hello world", "hello"), Some(0));
        assert_eq!(my_find("hello world", " "), Some(5));
        assert_eq!(my_find("hello world", "jj"), None);
        
        let mut lst: [i32; 5] = [1, 2, 12, 55, 33];
        my_reverse(&mut lst);
        assert_eq!(lst, [33, 55, 12, 2, 1]);

        let s = "hello world 🦀";

        assert_eq!(my_split_at(s, 1), Some(("h", "ello world 🦀")));
        assert_eq!(my_split_at(s, 155), None);
        assert_eq!(my_split_at(s, 13), None);

        let s = "🦀hello world";
        assert_eq!(my_split_at(s, "🦀".len()), Some(("🦀", "hello world")));

    }
}
