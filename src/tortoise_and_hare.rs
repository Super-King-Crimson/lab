#![allow(dead_code)]
//There is a uint array of length n + 1 that has values [1, n]
//Given that there is only one duplicate in this list, find it.

//YOU MUST ONLY USE CONSTANT EXTRA SPACE O(1)
//THE ARRAY CANNOT BE MODIFIED
//THE RUNTIME COMPLEXITY MUST BE O(n) OR LOWER
use rand::Rng;

///Generates a random array (vector) with length `size` with values from 1 to `size - 1`
fn gen_challenge(n: usize) -> (Vec<usize>, usize) {
    let mut vec: Vec<usize> = Vec::new();
    //0 is init value of repeated value since we 100% know 0 cannot be in array
    let mut repeated_value: usize = 0;
    let mut rng;

    for _ in 0..n {
        loop {
            rng = rand::thread_rng().gen_range(1..n);

            if vec.iter().find(|x| x == &&rng).is_some() {
                if repeated_value == 0 {
                    repeated_value = rng;
                } else if repeated_value != rng {
                    continue;
                }
            }

            break;
        }

        vec.push(rng);
    }

    (vec, repeated_value)
}

pub fn solution(arr: &[usize]) -> Option<usize> {
    let mut tortoise = Some(&arr[0]);
    let mut hare = Some(&arr[0]);

    while hare.is_some() {
        tortoise = arr.get(*tortoise.expect("tortoise should be some as it was just checked"));
        hare = arr.get(
            *arr.get(
                *hare.expect("hare should be some as it was just checked")
            ).unwrap_or_else(|| hare.unwrap()) //use unwrap_or_else here for lazy evaluation of fn call
        );

        if hare.is_some() && tortoise == hare {
            let mut from_head = arr[0];
            let mut from_meeting = *hare.unwrap();            

            while from_head != from_meeting {
                from_head = arr[from_head];
                from_meeting = arr[from_meeting];                
            }

            return Some(from_head);
        }
    }
    
    Option::None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let v1 = vec![1, 2, 3, 4, 1];        
        assert_eq!(Some(1), solution(&v1));
    }

    #[test]
    fn test2() {
        let v2 = vec![1, 2];
        assert_eq!(None, solution(&v2))
    }

    #[test]
    fn test3() {
        let (v3, repeated_value1) = gen_challenge(10);        
        assert_eq!(Some(repeated_value1), solution(&v3));        
    }   

    #[test]    
    fn test4() {
        let (v4, repeated_value2) = gen_challenge(100);
        assert_eq!(Some(repeated_value2), solution(&v4));
    }

    #[test]    
    fn test5() {
        let (v5, repeated_value3) = gen_challenge(1000);
        assert_eq!(Some(repeated_value3), solution(&v5));
    } 

    #[test]    
    fn test6() {
        let vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 3];
        assert_eq!(Some(3), solution(&vec));
    }
}