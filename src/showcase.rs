pub mod iterators {
    pub use std::collections::HashMap;
    
    /// Counts characters up to the 65535th character of `str`, and returns a HashMap<char, u32> containing the results.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use std::collections::HashMap;
    /// use lab::showcase::iterators::count_chars;
    /// 
    /// let str = "abcbdaded";
    /// 
    /// assert_eq!(count_chars(str), HashMap::from([
    ///       ('a', 2),
    ///       ('b', 2),
    ///       ('c', 1),
    ///       ('d', 3),
    ///       ('e', 1),
    /// ]));
    /// ```
    pub fn count_chars(str: &str) -> HashMap<char, u32> {        
        let mut counter = HashMap::new();
        
        for c in str.chars() {  
            counter.entry(c).and_modify(|repeats| *repeats += 1).or_insert(1u32);       
        }

        counter
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use iterators::*;

    #[test]
    fn to_hash_test() {
        let str = "abcbdaded";

        assert_eq!(iterators::count_chars(str), HashMap::from([
            ('a', 2),
            ('b', 2),
            ('c', 1),
            ('d', 3),
            ('e', 1),
        ]));
    }

    #[test]
    fn to_hash_test2() {
        let str = "";

        assert_eq!(iterators::count_chars(str), HashMap::new());
    }    
}