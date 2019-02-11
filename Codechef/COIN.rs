use std::vec::{self};
use std::io::{self};
use std::string::{self};
use std::collections::{HashMap};

//macro_rules! input{
//   () => {
//       let mut line = String::new();
//       io::stdin().read_line(&mut line);
//       let mut inp_ = line.split_whitespace();
//       macro_rules! inp{
//           ($t:ty) => {inp_.next().unwrap().parse::<$t>().unwrap();}
//       }
//   }
//}


fn solve(test: i64, memo: &mut HashMap<i64,i64>) -> i64 {
    if memo.contains_key(&test){
        return memo[&test];
    }

    if test == 0{
        return 0;
    }

    let mut ret : i64 = 0;
    let test1 = solve(test/2, memo) + solve(test/3, memo) + solve(test/4, memo);
    if test1 > test{
        ret = test1;
    }
    else{
        ret = test;
    }

    memo.insert(test, ret);
    return ret;
}

fn main(){
    //input!();
    let mut answers : Vec<i64> = Vec::new();
    while true{
       let mut line = String::new();
       io::stdin().read_line(&mut line);
       if line.len() == 0 {
           break;
       }
       let mut inp_ = line.split_whitespace();
       let test = inp_.next().unwrap().parse::<i64>().unwrap();
       let mut memo : HashMap<i64,i64> = HashMap::new();
       answers.push(solve(test, &mut memo)); 
    }

    for i in answers{
        println!("{}", i);
    }
}

