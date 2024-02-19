use neovim_lib::Value;

pub fn parse_string(values: &Vec<Value>) -> Vec<String> {
    values.iter().map(|value| value.to_string()).collect()
}

pub fn parse_string_first(values: &Vec<Value>) -> String {
    parse_string(values).first().unwrap().to_string()
}
