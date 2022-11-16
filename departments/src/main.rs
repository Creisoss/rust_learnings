use std::collections::HashMap;
use std::io;

fn main() {
    let mut hash: HashMap<String, Vec<String>> = HashMap::new();    
    let mut input_text: String;
    loop {
        println!("1 - Add user to a department\n2 - Read users from departments\n3 - Read all users from all departments\n4 - Add department\n5 - Exit\n");
        input_text = return_string();
        match input_text.as_str() {
            "1\n" => add_user_to_department(&mut hash),
            "2\n" => print_department(&mut hash),
            "3\n" => print_all_departmens(&mut hash),
            "4\n" => add_department(&mut hash),
            "5\n" => break,
            _ => continue,
        }
    };
}

fn clear_screen(){
       print!("\x1B[2J\x1B[1;1H");
}

fn add_user_to_department(hash: &mut HashMap<String, Vec<String>>){
    let department_name: String = write_the_department_name();
    if check_if_the_department_exists(&department_name, hash){
        println!("Type the name of the new employee: ");
        let mut employee_name: String = return_string();
        employee_name.remove(employee_name.len()-1);
        let v = hash.get_mut(&department_name).unwrap();   
        v.push(employee_name);
        v.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));
    }else{
        println!("This department does not exists");
    }
}

fn print_department(hash: &mut HashMap<String, Vec<String>>){
    clear_screen();
    let department_name: String = write_the_department_name();
    if check_if_the_department_exists(&department_name, hash){
        println!("{department_name} department:");
        for name in hash.get(&department_name).unwrap().iter(){
            println!("\t{}", name);
        }
    }else{
        println!("This department does not exists");
    }
}

fn write_the_department_name() -> String{
    println!("Type the name of the department: ");
    let mut department_name: String = return_string();
    department_name.remove(department_name.len()-1);
    department_name
}

fn check_if_the_department_exists(department_name: &String,hash: &mut HashMap<String, Vec<String>>) -> bool{
    hash.contains_key(department_name)
}

fn add_department(hash: &mut HashMap<String, Vec<String>>){
    let department_name: String = write_the_department_name();
    if !check_if_the_department_exists(&department_name, hash){
        hash.entry(department_name).or_insert(Vec::new());
    }else{
        println!("This department already exists");
    }
}

fn print_all_departmens(hash: &mut HashMap<String, Vec<String>>){
    clear_screen();
    for (key, value) in hash{
        println!("{key} department:");
        for name in value.iter(){
            println!("\t{name}");
        }
    }
}

fn return_string() -> String{
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input
}

