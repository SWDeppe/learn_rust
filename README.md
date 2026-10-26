# learn_rust
Ia driven rust learning with a learning structure similare to 42's piscine ! 

## How i work

I read a chapter of https://doc.rust-lang.org/book/ and ask IA to generate me some exercice function

## Chapters

Chapter 1 -> 3 ignored not interesting for real funtions 

### [Chapter 4](https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html)

#### TODO
- [ X ] my_strdup
- [ X ] my_trim
- [ ] my_find
- [ ] my_reverse
- [ ] my_split_at


#### my_strdup

Returns a new possesed string, that returns a exact copie of s. The result must stay valide even if the original one disapears


- **Definition**:
    ``` RUST
    pub fn my_strdup(s: &str) -> String
    ```
- **Types**: &str, String, char
- **autorised**: String::new, String::with_capacity, String::push, str::chars, Iterator::next

#### my_trim

Returns a slice of s without heading/tailing ASCII spaces. No "allocation" the result is a vue of s

- **Definition**: 
    ``` RUST
    pub fn my_trim(s: &str) -> &str
    ```
- **Types**: str::len, str::as_bytes, u8::is_ascii_whitespace
- **autorised**: String::new, String::with_capacity, String::push, str::chars, Iterator::next

#### my_find

returns in bytes the first occurence of neadle in haystack or None if there isn't. An empty neadle is allways found at position 0.

- **Definition**:
    ``` RUST
    pub fn my_find(haystack: &str, needle: &str) -> Option<usize>
    ``` 
- **Types**: &str, &[u8], usize, Option<usize>
- **autorised**: str::len, str::as_bytes

#### my_reverse

Returns a slice of s without heading/tailing ASCII spaces. No "allocation" the result is a vue of s

- **Definition**:
    ``` RUST
    pub fn my_reverse(slice: &mut [i32])
    ```
- **Types**: str::len, str::as_bytes, u8::is_ascii_whitespace
- **autorised**: String::new, String::with_capacity, String::push, str::chars, Iterator::next

#### my_split_at

cuts s at index mid and returns the two slices (before, after). Reruns None if mid is higher than the length of s or dosen't falls into a charater frontier. 

- **Definition**:
    ``` RUST
    pub fn my_split_at(s: &str, mid: usize) -> Option<(&str, &str)>
    ``` 
- **Types**: &str, usize, Option<(&str, &str)>
- **autorised**: str::len, str::is_char_boundary, découpe &s[..mid] et &s[mid..], Some, None
