use std::io::{self};
use std::vec::{self};

//Problem: https://www.hackerrank.com/contests/deep-whale-reef/challenges/pwsh-grids/submissions/code/1312832506

fn sum_divisors(n : i32) -> i32{
    let sqrt = (n as f32).sqrt().floor() as i32;
    let mut sum = 0;
    for i in 1..sqrt + 1{
        sum = sum + ((n as f32)/(i as f32)).floor() as i32;
    }
    return 2*sum - sqrt*sqrt;
}

fn main(){
    let mut line = String::new();
    io::stdin().read_line(&mut line);
    let M = line.split_whitespace().next().unwrap().parse::<i32>().unwrap();
    let mut answers = Vec::new();
    for i in 0..M{
        let mut line = String::new();
        io::stdin().read_line(&mut line);
        let N = line.split_whitespace().next().unwrap().parse::<i32>().unwrap();
        answers.push(sum_divisors(N));
    }
    for answer in answers{
        println!("{}", answer);
    }    
}
