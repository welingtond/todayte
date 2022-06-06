extern crate todayte ;
use todayte::todayte::Todayte ;
use std::env ;
use std::time::SystemTime ;
use std::time::UNIX_EPOCH ;

fn main() {
    let gmt_value: i8 ;
    match env::var("GMT") {
        Ok(value) => {
            gmt_value = value.parse().unwrap()
        } ,
        Err(_) => {
            println!("The Environment variable GMT is not set!\nUsing GMT=0") ;
            gmt_value = 0
        }
    }

    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(secs_ago) => {
            println!("{}", Todayte::new(secs_ago.as_secs(), gmt_value)) ;
        }
        Err(_) => {
            println!("Error calculation date!") ;
        }
    }
}