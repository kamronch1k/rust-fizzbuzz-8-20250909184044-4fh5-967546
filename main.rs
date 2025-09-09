fn main(){for i in 1..=8{let mut s=String::new(); if i%3==0{s.push_str("Fizz");} if i%5==0{s.push_str("Buzz");} if s.is_empty(){println!("{}",i)} else {println!("{}",s)} }}
