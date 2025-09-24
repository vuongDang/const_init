fn main() {
    let foo = env!("TEMP_FOO");
    let bar = env!("TEMP_BAR");
    println!("foo: {foo}");
    println!("bar: {bar}");

    if foo == "true" {
        println!("I should be print this");
    } else {
        println!("I should not print this, and I should not even appear in the binary");
    }
}
