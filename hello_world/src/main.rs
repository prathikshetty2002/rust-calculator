fn main() {
    let x=10;
    let y=20;
    let z=add_val(x,y);
    println!("{}",z)
}

fn add_val(x: u8, y: u8)-> u8{
    return x+y;
}
