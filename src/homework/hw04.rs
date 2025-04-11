const SIZE: usize = 10; // размерная константа является определением ДЛИННЫ каждой из четырех граней ромба. Не высота и не ширина самого ромба.

fn top_left() -> Vec<Vec<char>>
 {
    let mut v = vec![vec![' '; SIZE]; SIZE];
    for i in 0..SIZE 
    {
        for j in 0..=i 
        {
            v[i][SIZE - 1 - j] = '*'; // рисуем слева направо
        }
    }
    v
}

fn top_right() -> Vec<Vec<char>>
 {
    let mut v = vec![vec![' '; SIZE]; SIZE];
    for i in 0..SIZE
     {
        for j in 0..=i
         {
            v[i][j] = '*'; // рисуем справа налево
        }
    }
    v
}

fn bottom_left() -> Vec<Vec<char>> 
{
    let mut v = vec![vec![' '; SIZE]; SIZE];
    for i in 0..SIZE 
    {
        for j in 0..(SIZE - i) 
        {
            v[i][SIZE - 1 - j] = '*'; // убывающая левая половина
        }
    }
    v
}

fn bottom_right() -> Vec<Vec<char>> 
{
    let mut v = vec![vec![' '; SIZE]; SIZE];
    for i in 0..SIZE 
    {
        for j in 0..(SIZE - i)
         {
            v[i][j] = '*'; // убывающая правая половина
        }
    }
    v
}

fn main() {
    let mut output = String::new();

    // верх ромба
    let left = top_left();
    let right = top_right();
    for i in 0..SIZE
     {
        for j in 0..SIZE
         {
            output.push(left[i][j]);
        }
        for j in 1..SIZE 
        {
            output.push(right[i][j]);
        }
        output.push('\n');
    }

    // низ ромба
    let left_b = bottom_left();
    let right_b = bottom_right();
    for i in 1..SIZE
     {
        for j in 0..SIZE
        {
            output.push(left_b[i][j]);
        }
        for j in 1..SIZE 
        {
            output.push(right_b[i][j]);
        }
        output.push('\n');
    }

    print!("{}", output); // один print
    println!(""); // один println ^^
}
