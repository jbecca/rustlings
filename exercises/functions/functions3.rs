// functions3.rs
//
// Execute `rustlings hint functions3` or use the `hint` watch subcommand for a
// hint.


fn main() {
    let mut x: u32 = 28;
    call_me(&mut x);
    println!("new val of x {}", x)
}

fn call_me(num: &mut u32) {
    *num = 74;
    for i in 0..*num {
        println!("Ring! Call number {}", i + 1);
    }
}
