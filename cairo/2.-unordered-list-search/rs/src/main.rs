use std::convert::TryFrom;

fn main() {
    let list: [usize; 7] = [6, 3, 45, 5, 79, 13, 0];

    let x1 = 5;
    let x2 = 10;
    let mut found_x1: (isize, bool) = (-1, false);
    let mut found_x2: (isize, bool) = (-1, false);
    for (i, item) in list.iter().enumerate() {
        if !found_x1.1 {
            found_x1.1 = &x1 == item;
            found_x1.0 = isize::try_from(i).ok().unwrap();
        }
        if !found_x2.1 {
            found_x2.1 = &x2 == item;
            found_x2.0 = isize::try_from(i).ok().unwrap();
        }
    }

    if found_x1.1 {
        println!("Found x1 on pos: {}", found_x1.0);
    }

    if !found_x1.1 {
        println!("x1 not found...");
    }

    if found_x2.1 {
        println!("Found x2 on pos: {}", found_x2.0);
    }

    if !found_x2.1 {
        println!("x2 not found...");
    }
}
