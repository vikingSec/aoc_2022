
const filename : &str = "./input.txt";

fn repeatedChars(input : &str) -> bool {
     
    let mut chars : Vec<u8> = input.as_bytes().to_vec();
    while chars.len() > 0{
        let mut popped : u8 = chars.pop().unwrap();
        if chars.contains(&popped){
            return false
        }  
    }
    return true 

}

pub fn solve1() -> usize{
    
    let input_raw : String = std::fs::read_to_string("./input.txt").unwrap();
    let lines : Vec<&str> = input_raw.split("\n").collect();
    
    let input_chars : Vec<char> = input_raw.chars().collect();
    let mut index : usize = 0;
    while index+4 < input_chars.len(){
        let chunk : String = input_chars[index..index+4].into_iter().collect();
        if repeatedChars(chunk.as_str()){
            return index+4;
        }
        index+=1;

    }
    return 0;
}

pub fn solve2() -> usize{
    let input_raw : String = std::fs::read_to_string("./input.txt").unwrap();
    let lines : Vec<&str> = input_raw.split("\n").collect();

 
    let input_chars : Vec<char> = input_raw.chars().collect();
    let mut index : usize = 0;
    while index+4 < input_chars.len(){
        let chunk : String = input_chars[index..index+14].into_iter().collect();
        if repeatedChars(chunk.as_str()){
            return index+14;
        }
        index+=1;

    }
    return 0;

}
