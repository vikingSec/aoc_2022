
const filename : &str = "./input.txt";

fn chartonum(input : char) -> u32 {
    let lowercaseOff : u32 = 96;
    let uppercaseOff : u32 = 38; 
     
    if(input.is_uppercase()){

        println!("Letter {} is uppercase, so score should be {}", input, input as u32-uppercaseOff);
        input as u32 - uppercaseOff
    }else{

        println!("Letter {} is lowercase, so score should be {}", input, input as u32-lowercaseOff);
        input as u32 - lowercaseOff
    }
}

pub fn solve1(){
    
    let input_raw : String = std::fs::read_to_string("./input.txt").unwrap();
    let lines : Vec<&str> = input_raw.split("\n").collect();
    let mut sum = 0;
    for mut line in &lines{
        let line = &line.trim();
        let half = line.len() / 2;
        let chars : Vec<char> = line.chars().collect();
        let comp1 = &chars[0..half];
        let comp2 = &chars[half..chars.len()];
        let shared : Vec<char> = comp1.into_iter().filter(|&a| comp2.contains(a)).cloned().collect::<Vec<char>>();
        if shared.len() > 0 {
            sum+=chartonum(shared[0]);
        }else{

            println!("comp1: {:?}\ncomp2: {:?}",comp1,comp2);
        }

        
    }

    println!("Sum : {}",sum);
}

pub fn solve2(){
    let input_raw : String = std::fs::read_to_string("./input.txt").unwrap();
    let lines : Vec<&str> = input_raw.split("\n").collect();

    let mut sum = 0;
    let mut index = 0;
    while index < lines.len()-3{
        
        let comp1 : Vec<char> = lines[index].trim().chars().collect();
        let comp2 : Vec<char> = lines[index+1].trim().chars().collect();
        let comp3 : Vec<char> = lines[index+2].trim().chars().collect();
        let shared: Vec<char> = comp1.into_iter().filter(|&a| comp2.contains(&a) && comp3.contains(&a)).collect::<Vec<char>>();
        if shared.len() > 0 {
            sum+=chartonum(shared[0]);
        }

        index+=3;
    }
    println!("Sum : {}",sum);

    println!("{}",lines.len());
}
