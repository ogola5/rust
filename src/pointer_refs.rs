// Reference pointers point to a resource in memory
pub fn run(){
    //Primitive Array
    let arr1 = [1,2,3];
    let arr2= arr1;

    println!("values of {:?}",(arr1,arr2));

    //with non-primitives ,if you assign another variable to a pice of data
    //the first variable will nolonger hold that value. You will need
    //to use reference (&) to point to the resource
    //vector
    let vec1=vec![1,2,3];
    let vec2=&vec1;

    println!("Values :{:?}",(&vec1,vec2));

   
}