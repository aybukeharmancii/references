fn main() {
    let mut x = 10;
    //let xr = &x; immutable
    {
        let dom = &mut x;

    *dom += 1;


    }


    println!("x is {}", x);
}
