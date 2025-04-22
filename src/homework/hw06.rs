fn draw_tree(levels: usize) {
    let total_levels = levels + 1;
 
// max bottom stroke lenght
    let max_width = (1..=total_levels).map(|lvl| lvl * 2 - 1).last().unwrap();

 // rendering all lvls
    (1..=total_levels).for_each(|level| {
        (1..=(2 * level - 1))
            .step_by(2)
            .for_each(|stars| {
                let padding = (max_width - stars) / 2;
                let line = " ".repeat(padding) + &"*".repeat(stars);
                println!("{}", line);
            });
    });
}

fn main() {
    draw_tree(4);
}
