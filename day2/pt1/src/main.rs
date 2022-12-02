const win : i32 = 6;
const draw: i32 = 3;
const loss: i32 = 0;

const rock : i32 = 1;
const paper: i32 = 2;
const scissors: i32 = 3;

fn main() {
    
    let input_raw = std::fs::read_to_string("./input.txt").unwrap();
    let lines : Vec<&str> = input_raw.split("\n").collect();
    let mut score = 0;
    for mut line in lines{
        line = line.trim();
        let spl : Vec<&str> = line.split(" ").collect();
        let opp : String = String::from(spl[0]);
        let us : String = String::from(spl[1]);
        // Winning conditions
        println!("[-] Opp: {}\tUs: {}", opp, us);
        if(opp == "A" && us == "Y"){
            println!("opp chooses a");
            score+=win+paper;
        }
        if(opp == "B" && us == "Z"){
            score+=win+scissors;
        }
        if(opp == "C" && us == "X"){
            score+=win+rock;
        }
        // Drawing conditions
        if(opp == "A" && us == "X"){
            score+=draw+rock;
        }
        if(opp == "B" && us == "Y"){
            score+=draw+paper;
        }
        if(opp == "C" && us == "Z"){
            score+=draw+scissors;
        }
        // Losing conditions
        if(opp == "A" && us == "Z"){
            score+=loss+scissors;
        }
        if(opp == "B" && us == "X"){
            score+=loss+rock;
        }
        if(opp == "C" && us == "Y"){
            score+=loss+paper;
        }
    }
    println!("[-] Our total score is {}",score)

}
