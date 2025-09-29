mod generated;
use generated::settings::*;

fn main() {
    let present: &str = "I should be present in the binary";
    let absent: &str = "I should be absent in the binary";

    if FOO {
        // Should be kept during compiler optimizations
        println!("{}", present);
    } else {
        // Should be removed by compiler optimizations
        println!("{}", absent);
    }
    if BAR == 1 {
        // Should be kept during compiler optimizations
        println!("{}", present);
    } else {
        // Should be removed by compiler optimizations
        println!("{}", absent);
    }
}
