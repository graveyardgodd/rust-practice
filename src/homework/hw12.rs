fn count_permutation(shipments: &Vec<u32>) -> Option<usize>
{
    let total: u32 = shipments.iter().sum();
    let n = shipments.len() as u32;
    if total % n != 0
    {
        return None;
    }

    let avg = total / n;
    let mut moves = 0;

    for &s in shipments
    {
        if s > avg
        {
            moves += (s - avg) as usize;
        }
    }

    Some(moves)
}

fn gen_shipments(n: usize) -> Vec<u32>
{
    let base = 100;
    let mut shipments = vec![base; n];
    for i in 0..n / 2
    {
        shipments[i] += i as u32;
        shipments[n - 1 - i] -= i as u32;
    }
    shipments
}

fn main()
{
    let ships = gen_shipments(6);
    println!("Shipments: {:?}", ships);

    match count_permutation(&ships)
    {
        Some(moves) => println!("Min moves to balance: {}", moves),
        None => println!("Not possible to balance shipments equally."),
    }
}
