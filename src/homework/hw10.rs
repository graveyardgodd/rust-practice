fn is_palindrome(x: u32) -> bool 
{
    let s = x.to_string();
    s.chars().eq(s.chars().rev())
}
fn test()
  {
    let data =
    [
        (123, false),
        (121, true),
        (1221, true),
    ];
    for (n, expected) in data.iter() 
    {
        assert_eq!(is_palindrome(*n), *expected);
    }
}
fn main()
{
    let n = 1221;
    println!("{} is {}palindrome", n, if is_palindrome(n) { "" } else { "not " });
} 

