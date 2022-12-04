use advent_of_code::*;
use std::env;

macro_rules! dispatch_to {
    ($val:expr => {$($func:ident), *}) => {
        match $val {
            $(
                stringify!($func) => $func(),
            )*
            _ => {}
        }
    };
}

fn dispatch(s: &str) {
    dispatch_to!( s => {
        advent_1,
        advent_2,
        advent_3,
        advent_4
    });
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let advent_number: String = format!("advent_{}", args.get(1).unwrap_or(&"1".to_string()));

    dispatch(&advent_number);
}
