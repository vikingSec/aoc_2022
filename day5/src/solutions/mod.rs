
const filename : &str = "./input.txt";


fn constructStacks(lines : &Vec<&str>){
    // Get line number for stack base
    let mut stackbase = 0;
    let mut line = lines[stackbase];
    let mut ret : Vec<String> = vec![]; 
    while line.len() > 2{
        stackbase+=1;
        line = lines[stackbase];
    }
    stackbase-=1;
    println!("Stack base is at line {}",stackbase);
    // Initialize stacks
    let baseSpl = lines[stackbase].split("  ");
    for item in baseSpl{
        ret.push(
            "".to_string() 
        ); 

    }
    println!("Initialized a total of {} new stacks!",ret.len());
    // Construct stacks by working backwards from the stackbase
    let mut curLine = stackbase as i32-1 as i32;
    while curLine >= 0{
        let linevec : Vec<u8> = lines[curLine as usize].as_bytes().to_vec();
        let mut curItem : usize = 0;
        let mut curStack : usize = 0;
        while curItem < linevec.len()-1{
            let curChunk : Vec<u8> = linevec[curItem..curItem+3].to_vec();
            if curChunk[0] as char == '['{
                ret[curStack].push(curChunk[1] as char);
            }
            curStack+=1;
            curItem+=4;
        }
        curLine -= 1;
         
    }
    let moves = &lines[stackbase+1..];
    let mut moveIndex = 1;
    println!("First move: {}",moves[1]);
    for curMove in moves{
        moveIndex+=1;
        let moveSpl : Vec<&str> = curMove.split(" ").collect();
        if moveSpl.len() > 2{
            let amt : u32 = moveSpl[1].parse::<u32>().unwrap();
            let from : u32 = moveSpl[3].parse::<u32>().unwrap()-1;
            let to : u32 = moveSpl[5].parse::<u32>().unwrap()-1;
            println!("Stacks: {:?}",ret); 
            println!("Move: {}\n\tFrom: {}\n\tTo: {}\n\tAmt: {}", curMove, ret[from as usize], ret[to as usize], amt);


            let mut reps = 0;
            while reps < amt{
                let popped = ret[from as usize].pop().unwrap();
                ret[to as usize].push(popped);
                reps+=1;
            }
            println!("After operations:");
            println!("From: {}\nTo: {}",ret[from as usize], ret[to as usize]);
            println!("Stacks: {:?}", ret);
                
        }
    }

    for mut stack in ret{
        println!("{}",&stack.pop().unwrap());
    } 
}


fn constructStacks2(lines : &Vec<&str>){
    // Get line number for stack base
    let mut stackbase = 0;
    let mut line = lines[stackbase];
    let mut ret : Vec<String> = vec![]; 
    while line.len() > 2{
        stackbase+=1;
        line = lines[stackbase];
    }
    stackbase-=1;
    println!("Stack base is at line {}",stackbase);
    // Initialize stacks
    let baseSpl = lines[stackbase].split("  ");
    for item in baseSpl{
        ret.push(
            "".to_string() 
        ); 

    }
    println!("Initialized a total of {} new stacks!",ret.len());
    // Construct stacks by working backwards from the stackbase
    let mut curLine = stackbase as i32-1 as i32;
    while curLine >= 0{
        let linevec : Vec<u8> = lines[curLine as usize].as_bytes().to_vec();
        let mut curItem : usize = 0;
        let mut curStack : usize = 0;
        while curItem < linevec.len()-1{
            let curChunk : Vec<u8> = linevec[curItem..curItem+3].to_vec();
            if curChunk[0] as char == '['{
                ret[curStack].push(curChunk[1] as char);
            }
            curStack+=1;
            curItem+=4;
        }
        curLine -= 1;
         
    }
    let moves = &lines[stackbase+1..];
    let mut moveIndex = 1;
    println!("First move: {}",moves[1]);
    for curMove in moves{
        moveIndex+=1;
        let moveSpl : Vec<&str> = curMove.split(" ").collect();
        if moveSpl.len() > 2{
            let amt : u32 = moveSpl[1].parse::<u32>().unwrap();
            let from : u32 = moveSpl[3].parse::<u32>().unwrap()-1;
            let to : u32 = moveSpl[5].parse::<u32>().unwrap()-1;
            println!("Stacks: {:?}",ret); 
            println!("Move: {}\n\tFrom: {}\n\tTo: {}\n\tAmt: {}", curMove, ret[from as usize], ret[to as usize], amt);

            let fromchars : Vec<u8> = ret[from as usize].as_bytes().to_vec();
            let mut chunk : Vec<u8> = fromchars[fromchars.len()-amt as usize..fromchars.len() as usize].to_vec();
            chunk.reverse();
            let mut reps = 0; 
            while reps < amt{
                let popped = chunk.pop().unwrap();
                ret[to as usize].push(popped as char);
                reps+=1;
                ret[from as usize].pop();
            }
            println!("After operations:");
            println!("From: {}\nTo: {}",ret[from as usize], ret[to as usize]);
            println!("Stacks: {:?}", ret);
                
        }
    }

    for mut stack in ret{
        println!("{}",&stack.pop().unwrap());
    } 
}




pub fn solve1(){
    
    let input_raw : String = std::fs::read_to_string("./input.txt").unwrap();
    let lines : Vec<&str> = input_raw.split("\n").collect();

    constructStacks(&lines);


    println!("{}",lines.len());
}

pub fn solve2(){
    let input_raw : String = std::fs::read_to_string("./input.txt").unwrap();
    let lines : Vec<&str> = input_raw.split("\n").collect();

    constructStacks2(&lines);

    println!("{}",lines.len());
}
