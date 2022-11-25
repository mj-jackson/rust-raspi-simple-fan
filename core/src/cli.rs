use std::env;

pub fn get_argument_value(arg: &str) -> Option<String> {
    let args_vec: Vec<String> = env::args().collect();
    let pos = args_vec.iter().position(|value| value == arg);

    pos.map(|p| {
        let i = p + 1;
        args_vec.get(i).map(|val| val.to_owned())
    })?
}
