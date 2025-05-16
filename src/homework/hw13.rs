use std::collections::HashSet;

struct Point  
{
    x: i32,
    y: i32,
}
struct Rectangle
{
    a: Point,
    b: Point,
}
fn area_occupied(xs: &Vec<Rectangle>) -> i32 
{
    let mut points = HashSet::new();
    for r in xs 
    {
        let x1 = r.a.x.min(r.b.x);
        let x2 = r.a.x.max(r.b.x);
        let y1 = r.a.y.min(r.b.y);
        let y2 = r.a.y.max(r.b.y);
        for x in x1..x2 
        {
            for y in y1..y2 
            {
                points.insert((x, y));
            }
        }
    }
    points.len() as i32
}
fn test_data() -> Vec<Rectangle> 
{
    vec!
    [
        Rectangle { a: Point { x: 2, y: 9 }, b: Point { x: 5, y: 3 } },
        Rectangle { a: Point { x: 1, y: 8 }, b: Point { x: 11, y: 6 } },
        Rectangle { a: Point { x: 9, y: 10 }, b: Point { x: 13, y: 2 } },
    ]
}

fn area_occupied_test() 
{
    let data = test_data();
    let occupied = area_occupied(&data);
    assert_eq!(occupied, 60);
}
