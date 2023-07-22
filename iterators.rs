pub mod showcase {
    fn showcase_zip() {
        let a = vec![1, 2, 3];
        let b = vec![5, 6, 7, 8, 9];
    
        let zip = a.iter().zip(
            b.iter().skip(b.len().checked_sub(a.len()).unwrap_or(0))
        );
    
        for (v1, v2) in zip {
            println!("({v1}, {v2})");
        }
    }
}