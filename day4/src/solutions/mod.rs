
const filename : &str = "./input.txt";

pub fn solve1(){
    
    let input_raw : String = std::fs::read_to_string("./input.txt").unwrap();
    let lines : Vec<&str> = input_raw.split("\n").collect();
    
    let mut sum : u32 = 0;

    for mut line in &lines{
        let line = line.trim();
        let spl : Vec<&str>= line.split(",").collect();
        let first : String= spl[0].to_string();
        let second : String = spl[1].to_string();
        let first_spl : Vec<&str> = first.split("-").collect();
        let sec_spl : Vec<&str> = second.split("-").collect();
        let first_beg : u32 = first_spl[0].to_string().parse::<u32>().unwrap();
        let first_end : u32 = first_spl[1].to_string().parse::<u32>().unwrap();
        let sec_beg : u32 = sec_spl[0].to_string().parse::<u32>().unwrap();
        let sec_end : u32 = sec_spl[1].to_string().parse::<u32>().unwrap();

        if((first_beg <= sec_beg && first_end >= sec_end) || (sec_beg <= first_beg && sec_end >= first_end)){
            sum+=1;
            println!("Sum: {}",sum); 
        }

    }

    println!("Total of {} encapsulating assignments", sum);
}

pub fn solve2(){


    let input_raw : String = std::fs::read_to_string("./input.txt").unwrap();
    let lines : Vec<&str> = input_raw.split("\n").collect();
    
    let mut sum : u32 = 0;

    for mut line in &lines{
        let line = line.trim();
        let spl : Vec<&str>= line.split(",").collect();
        let first : String= spl[0].to_string();
        let second : String = spl[1].to_string();
        let first_spl : Vec<&str> = first.split("-").collect();
        let sec_spl : Vec<&str> = second.split("-").collect();
        let first_beg : u32 = first_spl[0].to_string().parse::<u32>().unwrap();
        let first_end : u32 = first_spl[1].to_string().parse::<u32>().unwrap();
        let sec_beg : u32 = sec_spl[0].to_string().parse::<u32>().unwrap();
        let sec_end : u32 = sec_spl[1].to_string().parse::<u32>().unwrap();
        if(&first_beg <= &sec_end && &sec_beg <= &first_end){
            sum+=1;
            
            println!("[-] Sum: {}",sum);
        } 
        

    }

    println!("Total of {} encapsulating assignments", sum);

    println!("{}",lines.len());
}
