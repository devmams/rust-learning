fn main() {
    
    let mut x = 10;

    {
        let xr = &mut x;
        *xr += 2;
        println!("{}", xr);
    }

    println!("{}", x);
}
