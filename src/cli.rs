use std::env;
pub fn run(){
    let args:Vec<String> = env::args().collect();
    let command = args[1].clone();
    let name= "ogola";
    let status= "100%";

    //println!("command:{}",command);
    if command=="hello"{
        println!("Hi {},how are you?",name);

    }else if  command == status {
        println!("Status is {}",status);
        
    }else{
        println!("thats not a valid command");
    }
}