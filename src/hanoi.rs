fn is_valid_move(hanoi: &[Vec<u8>; 3], from_index: usize, to_index: usize) -> bool {    
    let from = &hanoi[from_index];
    let to = &hanoi[to_index];

    //Valid move if:
    //from has at least one ring and to has no rings 
    //the top ring (last ring) of from is smaller than the top ring of to
    !from.is_empty() && ( to.is_empty() || from.last() < to.last() )
}

fn move_one(hanoi: &mut [Vec<u8>; 3], from_index: usize, to_index: usize) {
    if is_valid_move(hanoi, from_index, to_index) {
        let ring = hanoi[from_index].pop().expect("Move is valid so ring must not be empty");
        hanoi[to_index].push(ring);
    } else {
        panic!("Invalid move attempted");
    }
}

fn perform_valid_move(hanoi: &mut [Vec<u8>; 3], from_index: usize, to_index: usize) {
    if is_valid_move(hanoi, from_index, to_index) {
        move_one(hanoi, from_index, to_index)
    } else if is_valid_move(hanoi, to_index, from_index) {
        move_one(hanoi, to_index, from_index)
    } else {
        panic!("to_index and from_index are both empty!");
    }
}

pub fn solve_recur(hanoi: &mut [Vec<u8>; 3], start_index: usize, goal_index: usize, spare_index: usize, n: u32) {
    if n == 0 {
        panic!("Attempted to solve a hanoi with empty start tower: `n` must be equal to the number of disks on top of hanoi[start_index]");
    }

    if n == 1 {
        move_one(hanoi, start_index, goal_index);        
    } else {
        //move n-1 rings to spare
        solve_recur(hanoi, start_index, spare_index, goal_index, n-1);
        //move the final ring on the bottom to the goal
        move_one(hanoi, start_index, goal_index);        
        //move the n-1 rings back onto the goal
        solve_recur(hanoi, spare_index, goal_index, start_index, n-1);
    }
}

pub fn solve_iter(hanoi: &mut [Vec<u8>; 3], start_index: usize, goal_index: usize, spare_index: usize) {
    let n = hanoi[start_index].len();

    if n == 0 {
        panic!("Attempted to solve a hanoi with empty start tower");
    }

    if n == 1 {
        move_one(hanoi, start_index, goal_index);
    }      

    let iterations = 2u32.checked_pow(u32::try_from(n).expect("Value is too large")).expect("Value is too large") - 1;
    
    //if n is even, swap goal and spare (they'll still end up on the right pole don't worry)
    let (goal_index, spare_index) = {
        let (mut new_goal, mut new_spare) = (goal_index, spare_index);

        if n % 2 == 0 {
            (new_goal, new_spare) = (new_spare, new_goal);            
        }        

        (new_goal, new_spare)
    };        

    println!("{goal_index}, {spare_index}");

    for i in 1..=iterations {
        if i % 3 == 0 {
            perform_valid_move(hanoi, spare_index, goal_index);
        } else if i % 3 == 1  {
            perform_valid_move(hanoi, start_index, goal_index);
        } else {
            perform_valid_move(hanoi, start_index, spare_index);
        }
    
        println!("start: {:?}, goal: {:?}, spare: {:?}", hanoi[start_index], hanoi[goal_index], hanoi[spare_index]);
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use super::*;

    #[test]
    fn hanoi_recur_test1() {
        let mut hanoi = [
            vec![5u8, 4, 3, 2, 1],
            vec![],
            vec![],
        ];

        solve_recur(&mut hanoi, 0, 1, 2, 5);

        assert_eq!(hanoi, [
            vec![],
            vec![5, 4, 3, 2, 1u8],
            vec![],
        ]);

        solve_recur(&mut hanoi, 1, 2, 0, 5);

        assert_eq!(hanoi, [
            vec![],
            vec![],
            vec![5, 4, 3, 2, 1u8],
        ]);
    }

    #[test]
    #[should_panic(expected = "empty start tower")]
    fn hanoi_recur_test2() {
        let mut hanoi: [Vec<u8>; 3] = [
            vec![],
            vec![],
            vec![],
        ];

        solve_recur(&mut hanoi, 0, 1, 2, 0);
    }

    #[test]
    #[should_panic(expected = "nvalid move")]
    fn hanoi_recur_test3() {
        let mut hanoi: [Vec<u8>; 3] = [
            vec![1, 2, 3],
            vec![],
            vec![],
        ];

        solve_recur(&mut hanoi, 0, 1, 2, 3);
    }

    #[test]    
    fn hanoi_recur_test4() {
        let mut hanoi: [Vec<u8>; 3] = [
            vec![1],
            vec![],
            vec![],
        ];

        solve_recur(&mut hanoi, 0, 1, 2, 1);

        assert_eq!(
            hanoi, [
                vec![],
                vec![1],
                vec![],
            ]
        );
    }

    #[test]
    fn hanoi_iter_test1() {
        let mut hanoi = [
            vec![6, 5, 4, 3, 2, 1u8],
            vec![],
            vec![],
        ];

        solve_iter(&mut hanoi, 0, 1, 2);

        assert_eq!(hanoi, [
            vec![],
            vec![6, 5, 4, 3, 2, 1u8],
            vec![],
        ]);

        solve_iter(&mut hanoi, 1, 2, 0);        

        assert_eq!(hanoi, [
            vec![],
            vec![],
            vec![6, 5, 4, 3, 2, 1u8],
        ]);
    }

    #[test]
    #[should_panic(expected = "empty start tower")]
    fn hanoi_iter_test2() {
        let mut hanoi: [Vec<u8>; 3] = [
            vec![],
            vec![],
            vec![],
        ];

        solve_iter(&mut hanoi, 0, 1, 2);
    }

    #[test]
    #[should_panic(expected = "too")] //too large, too big
    fn hanoi_iter_test3() {
        let mut hanoi = [
            vec![0u8; 256].iter_mut().zip(0u8..255).map(|(_, i)| 255 - i).collect::<Vec<u8>>(),            
            vec![],
            vec![],
        ];

        solve_iter(&mut hanoi, 0, 1, 2);

        assert_eq!(hanoi, [
            vec![],
            vec![6, 5, 4, 3, 2, 1u8],
            vec![],
        ]);

        solve_iter(&mut hanoi, 1, 2, 0);        

        assert_eq!(hanoi, [
            vec![],
            vec![],
            vec![6, 5, 4, 3, 2, 1u8],
        ]);
    }
}