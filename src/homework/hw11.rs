use rand::Rng;

fn gen_random_vector(n: usize) -> Vec<i32>
{
    let mut rng = rand::thread_rng();
    (0..n).map(|_| rng.gen_range(10..100)).collect()
}

fn min_adjacent_sum(data: &[i32]) -> (usize, i32, i32)
{
    let mut min_sum = i32::MAX;
    let mut idx = 0;
    for i in 0..data.len() - 1
    {
        let sum = data[i] + data[i + 1];
        if sum < min_sum
        {
            min_sum = sum;
            idx = i;
        }
    }
    (idx, data[idx], data[idx + 1])
}

fn print_vector_info(data: &[i32])
{
    println!("indexes: {}", (0..data.len()).map(|i| format!("{:>3}.", i)).collect::<Vec<_>>().join(" "));
    println!("data:    [{}]", data.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(", "));

    let (idx, a, b) = min_adjacent_sum(data);
    let mut arrow = vec!["   "; data.len()];
    arrow[idx] = "\\__";
    arrow[idx + 1] = "__/";
    println!("indexes: {}", arrow.join(" "));
    println!("min adjacent sum={}+{}={} at indexes:{},{}", a, b, a + b, idx, idx + 1);
}

fn main()
{
    let v = gen_random_vector(20);
    print_vector_info(&v);
}
