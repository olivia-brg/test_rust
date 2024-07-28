fn main() {
    
    let arr: [i32; 10] = [15, 76, 74, 64, 23, 95, 06, 81, 46, 52];

    // let i: i32;
    // let j: i32;
    let _swapped : bool;

    for i in 1..101 {
        if i % 15 == 0 {
            println!("fizzbuzz");
        } else if i % 3 == 0 {
            println!("fizz");
        } else if i % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", i);
        }
    }

    println!("{:?}", arr)

}
