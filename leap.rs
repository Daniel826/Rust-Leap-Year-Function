fn leap(x: i32) {
    if x%100 == 0 {
        if x%400 == 0 {
            println!("{} is a leap year.", x);
        } else {
            println!("{} is not a leap year.", x);
        }
    } else {
        if x%4 == 0 {
            println!("{} is a leap year.", x);
        } else {
            println!("{} is not a leap year.", x)
        }
    }
}

fn main() {
    //Should return is a leap year
    leap(1992);
    //Should return is not a leap year
    leap(1990);
    //Should return is a leap year
    leap(2000);
    //Should return is not a leap year
    leap(1800);
}