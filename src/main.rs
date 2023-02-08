use std::io;
use std::time::{Instant, Duration};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Problem {
    question: String,
    answer: String,
}

fn main() {
    let file_path = "problems.csv";
    let mut reader = csv::Reader::from_path(file_path).expect("Couldn't parse the given csv file");

    let mut correct = 0;
    let timer = Instant::now();
    for result in reader.deserialize() {
        let problem : Problem= match result {
            Ok(res) => {
                let problem: Problem = res;
                problem
            }
            Err(e) => panic!("Couldn't deserialize:{e}")
        };
        println!("{} =",problem.question);
        let mut user_answer = String::new();
        io::stdin()
        .read_line(&mut user_answer)
        .expect("Failed to read input from the user");

        if timer.elapsed() >= Duration::from_secs(20) {
            break;
        }
        if user_answer.trim() == problem.answer.trim() {
             correct = correct + 1;
        }

    }
    println!("You scored {correct} out of 7");

}
