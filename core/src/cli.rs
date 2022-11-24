use std::env;

pub fn get_argument_value(arg: &str) -> Option<String> {
    let args_vec: Vec<String> = env::args().collect();
    let pos = args_vec.iter().position(|value| value == arg);

    match pos {
        Some(position) => {
            let index = position + 1;
            let result = args_vec.get(index);
            
            result.map(|value| value.to_owned())
        },
        None => None
    }
}