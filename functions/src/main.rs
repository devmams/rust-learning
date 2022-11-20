fn main() {
    printer(10);
}

fn printer(nb: u32){
    for n in 1..nb {
        println!("{} : {}", n, is_even(n));
    }
}

fn is_even(nb: u32) -> bool {
    return nb % 2 == 0;
}