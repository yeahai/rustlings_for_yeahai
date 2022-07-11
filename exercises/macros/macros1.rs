// macros1.rs
// Make me compile! Execute `rustlings hint macros1` for hints :)

#[warn(unused_macros)]
macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
}

fn main() {
    my_macro!();
}
