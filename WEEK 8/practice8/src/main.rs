fn main() {
    let b:(i32,bool,f64) = (120,true,10.9);
    print(b);
}

fn print(x:(i32,bool,f64)){
    println!("Inside the print method");
    println!("{:?}",x );
}
