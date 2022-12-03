
const filename : &str = "./input.txt";

pub fn solve1(){
    
    let input_raw : String = std::fs::read_to_string("./input.txt").unwrap();
    let lines : Vec<&str> = input_raw.split("\n").collect();




    println!("{}",lines.len());
}

pub fn solve2(){
    let input_raw : String = std::fs::read_to_string("./input.txt").unwrap();
    let lines : Vec<&str> = input_raw.split("\n").collect();



    println!("{}",lines.len());
}
