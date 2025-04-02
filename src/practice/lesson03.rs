/// console - envelope
// ******************************
// *  *                      *  *
// *    *                  *    *
// *      *              *      *
// *        *          *        *
// *          *      *          *
// *            *  *            *
// *             **             *
// *           *    *           *
// *         *        *         *
// *       *            *       *
// *     *                *     *
// *   *                    *   *
// * *                        * *
// ******************************
fn envelope(width: u32, height: u32) {
    let range_y = 1..=height;
    for y in range_y {
        for x in 1..=width {
            let is_hor = y == 1 || y == height;
            let is_ver = x == 1 || x == width;

            let sym = if is_hor || is_ver { '*' } else { ' ' };
            print!("{sym}");
        }
        print!("\n");
    }
}

#[test]
fn test1() {
    envelope(25, 10);
}
