////////// DO NOT CHANGE BELOW HERE /////////
// This function should be called by the `show_output!()` macro
#[derive(Debug)]
struct Coordinate {
    x: i32,
    y: i32,
}

impl Coordinate {
    fn show(&self) {
        println!("({}, {})", self.x, self.y);
    }
}

////////// DO NOT CHANGE ABOVE HERE /////////

// TODO: Create `for_2d!` macro here.
macro_rules! for_2d {
    ($row:ident <$type_row:ty> in $iter1:expr, $col:ident <$type_col:ty> in $iter2:expr, $blk:block) => {
        for $row in $iter1 {
            let $row: $type_row = $row;
            for $col in $iter2 {
                let $col: $type_col = $col;
                $blk
            }
        }
    }
}
////////// DO NOT CHANGE BELOW HERE /////////

fn main() {
    for_2d!(row <i32> in 1..5, col <i32> in 2..7, {
        (Coordinate {x: col, y: row}).show()
    });

    let values = [1, 3, 5];

    for_2d!(x <u16> in values, y <u16> in values, {
        (Coordinate {x: x.into(), y: y.into()}).show()
    });
}
