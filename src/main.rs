fn main() {
    let x = 5;

    let x = x + 1;
    // x = 5+1 = 6, so x value is 6
    // it shadows the previous value of x
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
        // the x value is shadowed to 6*2 = 12
    }
    // here the x value is back to 6 as the scope of x*2 is over
    println!("The value of x is: {}", x);
    
}