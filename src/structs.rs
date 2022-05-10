// Struct - Used to create custom data types

//traditional struct
// struct Color{
//     red:u8,
//     green:u8,
//     blue:u8
// }    
//Tuple struct
//struct Color(u8,u8,u8);

struct Person{
    first_name:String,
    second_name:String
}
impl Person {
    //Construct a person
    fn new(first:&str,second:&str) -> Person{
        Person { first_name:first.to_string(), second_name:second.to_string () }
    }
    //get full name
    fn full_name(&self) ->String{
        format!("{} {}",self.first_name,self.second_name)
    }
    //set last name
    fn set_second_name(&mut self,second:&str) {
        self.second_name = second.to_string();
    }
    //name to tuple
    fn to_tuple(self) ->(String,String){
        (self.first_name,self.second_name)
    }
}
pub fn run(){
//     let mut c = Color{
//         red:255,
//         green:0,
//         blue:0
//     };

//     c.red=200;
//     println!("color:{} {} {}",c.red,c.green,c.blue);

// let mut c = Color(255,0,0);

// c.0=200;

// println!("color: {} {} {}",c.0,c.1,c.2)
let mut p = Person:: new("Mary","Doe");
println!("Person {}",p.full_name());
p.set_second_name("evance");

println!("Person Tuple {:?}",p.to_tuple());
}