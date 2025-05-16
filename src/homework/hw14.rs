fn gray(n: u8) -> Vec<String>  
{
    static mut MEMO: Option<Vec<Vec<String>>> = None;
    unsafe 
    {
        let memo = MEMO.get_or_insert_with(|| vec![vec!["".to_string()]]);
        while memo.len() <= n as usize
        {
            let prev = &memo[memo.len() - 1];
            let mut next = vec![];
            next.extend(prev.iter().map(|s| "0".to_string() + s));
            next.extend(prev.iter().rev().map(|s| "1".to_string() + s));
            memo.push(next);
        }
        memo[n as usize].clone()
    }
}
