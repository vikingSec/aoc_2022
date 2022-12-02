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
        let res : String = String::from(spl[1]);

        // x == win
        // y == draw
        // z == loss
        // Winning conditions
        if(res == "X"){
            score+=loss;
            if(opp == "A"){
                score+=scissors;
            }
            if(opp == "B"){
                score+=rock;

            }
            if(opp == "C"){
                score+=paper;
            }
        }
        if(res == "Y"){
            score+=draw;
            if(opp == "A"){
                score+=rock;
            }
            if(opp == "B"){
                score+=paper;
            }
            if(opp == "C"){
                score+=scissors;
            }
        }
        if(res == "Z"){
            score+=win;
            if(opp == "A"){
                score+=paper;
            }
            if(opp == "B"){
                score+=scissors;
            }
            if(opp == "C"){
               score+=rock; 
            }
        }
    }
    println!("[-] Our total score is {}",score)

}
