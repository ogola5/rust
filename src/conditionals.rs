pub fn run(){
    let age: u8= 22;
    let check_id:bool = false;
    let knows_person_of_age = true;

    //if else
    if age >= 21 && check_id || knows_person_of_age{
        println!("bartnder: what would you like to drink");
    }else if age < 21 && check_id {
        println!("bartender: sorry you have to leave");
    }else{
        println!("bartender: i lll need to check your id");
    }
    //
    let is_of_age=if age>21 {true} else{false};
    println!("Is Of Age:{}",is_of_age)
}
