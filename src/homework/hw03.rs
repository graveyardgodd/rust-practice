const A: usize = 20;
const B: usize = 10;
fn get_chr(sh: usize, vs: usize, i: usize, j: usize) -> char 
{  // если мы на границе (верх/низ/лево/право) или на одной из диагоналей, то рисуем "#"
    if i == 0 || i == vs - 1 || j == 0 || j == sh - 1 ||j == i * sh / vs || j == sh - 1 - i * sh / vs 
    {
        '#' 
    } else {
        ' ' 
    }
}
fn main()
 {
    let mut v = String::new();

    for i in 0..B
     {
        for j in 0..A 
        {
            v.push(get_chr(A, B, i, j));
        }
        v.push('\n');
    }

    print!("{}", v);
    println!(""); // разрешили использовать один раз, так что я использую
}
