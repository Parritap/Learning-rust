fn main() {
    let num: i32 = find_max(&[1, 100, 4, 1, 30]);
    println!("{}", num)
}

pub fn find_max(slice: &[i32]) -> i32 {
    let mut max = slice[0];
    for &value in slice.iter().skip(1) {
        if value > max {
            max = value;
        }
    }
    max
}

