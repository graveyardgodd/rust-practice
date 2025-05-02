fn rotate2(s: &str, n: &isize) -> String 
  {
    let len = s.len() as isize;
    let mut real_shift = *n;
    if real_shift >= len || real_shift <= -len
    {
        real_shift = real_shift % len;
    }
    if real_shift < 0
    {
        real_shift = len + real_shift;
    }
    let mut i = 0;
    let mut chars: Vec<char> = Vec::new();
    for c in s.chars()
    {
        chars.push(c);
    }

    let mut result = String::new();
    while i < len 
      {
        let idx = (i + len - real_shift) % len;
        let mut j = 0;
        let mut ch = ' ';
        while j < len
          {
            if j == idx
            {
                ch = chars[j as usize];
            }
            j += 1;
        }
        result.push(ch);
        i += 1;
    }
    result
}
fn test()
  {
    let s = "abcdefgh";
    let shifts = 
      [
        (0,  "abcdefgh"),
        (8,  "abcdefgh"),
        (-8, "abcdefgh"),
        (1,  "habcdefg"),
        (2,  "ghabcdef"),
        (10, "ghabcdef"),
        (-1, "bcdefgha"),
        (-2, "cdefghab"),
        (-10,"cdefghab"),
    ];
    for (n, expected) in shifts.iter()
      {
        assert_eq!(rotate2(s, n), expected.to_string());
    }
}
fn main()
{
    let s = String::from("abcdefgh");
    let rotated = rotate2(&s, &2);
    println!("{}", rotated);
}
