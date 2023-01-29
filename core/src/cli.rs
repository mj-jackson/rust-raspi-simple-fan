pub fn get_argument_value<T>(arg: &str, args: T) -> Option<String>
where
    T: IntoIterator<Item = String>,
{
    let args_vec: Vec<String> = args.into_iter().collect();
    let pos = args_vec.iter().position(|value| value == arg);

    pos.map(|p| {
        let i = p + 1;
        args_vec.get(i).map(|val| val.to_owned())
    })?
}
