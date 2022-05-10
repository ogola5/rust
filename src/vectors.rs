//vectors  are resizable array

use std::mem;

pub fn run(){
    let mut numbers:Vec<i32> = vec![1,2,3,4];

    //reassign value
    numbers[2]= 20;
    println!("{:?}",numbers);

    //add on to vector
    numbers.push(5);
    numbers.push(6);

    //pop the last value
    numbers.pop();

    //get single val
    println!("single Value:{}",numbers[0]);

    //get array length
    println!("Vector  Length:{}",numbers.len());

    //Vectors are stack allocated
    println!("Vector occupies {} bytes",mem::size_of_val(&numbers));

    //Get Slice
    let slice:&[i32] = &&numbers[1..3];
    println!("slice :{:?}",slice);

    //loop through vector values

    for x in numbers.iter() {
        println!("numbers:{}",x);
    }
    // loop &mut ate valus
    for x in numbers.iter_mut() {
        *x *= 2;
    }

    println!("numbers vec:{:?}",numbers);
}
