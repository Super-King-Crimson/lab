pub mod hashmaps {
    pub use std::collections::HashMap;
    
    /// Counts characters up to the 65535th character of `str`, and returns a HashMap<char, u32> containing the results.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use std::collections::HashMap;
    /// use lab::showcase::hashmaps::count_chars;
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

pub mod iterators {
    pub fn every_other_item_in_first_ten_times_five(slice: &[i32]) -> Vec<i32> {
        slice.iter().take(10).step_by(2).map(|v,| *v * 5).collect()
    }

    pub fn flatten_find_indices(pat: &str, slice: &[&str]) -> Vec<(usize, usize)> {
        //or zip(1usize..)
        slice.iter().enumerate() //create an (i, str) iterator over the collection of strs
            .flat_map(|(outer_count, s)| //for each (i, str)
                s.match_indices(pat).map(move |(inner_count, _)| //create an iterator based on the index of matches of pat in str (j, match) pairs
                    (outer_count, inner_count) //and turn it into one that returns (i, j)
                )
            ) //flatten those iterators: iterator over iterators over (i, j) => iterators over (i, j)
        .collect() //collect into a vec and return it
    }

    pub fn parse(slice: &mut [&str]) -> Vec<u64> {
        slice.iter().filter_map(|s| s.parse().ok()).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hashmaps::*;
    use iterators::*;

    #[test]
    fn to_hash_test() {
        let str = "abcbdaded";

        assert_eq!(count_chars(str), HashMap::from([
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

        assert_eq!(count_chars(str), HashMap::new());
    }    

    #[test]
    fn every_other_item_in_first_ten_times_five_test1() {
        let vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        assert_eq!(every_other_item_in_first_ten_times_five(&vec), vec![5, 15, 25, 35, 45i32]);
    }

    #[test]
    fn every_other_item_in_first_ten_times_five_test2() {
        let vec = vec![0, 2, 40, 600, 8000, 100000];
        assert_eq!(every_other_item_in_first_ten_times_five(&vec[1..]), vec![10, 3000, 500000]);
    }
}