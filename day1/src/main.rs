use std::fs;
const filename : &str = "./input.txt";   

fn main() {
    
    let input_raw = fs::read_to_string(filename).unwrap();
    let lines : Vec<&str> = input_raw.split("\n").collect();
    println!("[-] Lines in input file: {}",lines.len());
    let mut totals : Vec<i32> = Vec::new();
    let mut curElf : Vec<i32> = Vec::new();
    for line in lines{
        if (line.len() < 1){
            let mut sum = curElf.iter().sum();
            totals.push(sum);
            curElf = Vec::new();
        }else{
            curElf.push(line.parse::<i32>().unwrap());
        }
    }
    totals.sort();
    let relsum : i32 = totals[totals.len()-3..totals.len()].iter().sum();
    println!("The sum of the top 3 elves is {}", relsum);
}
