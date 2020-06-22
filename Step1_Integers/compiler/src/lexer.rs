use std::fs;
use std::error::Error;
use regex::Regex;

pub fn tokenize( path: String ) -> Result<Vec<String>, Box<dyn Error>>{
    let content = fs::read_to_string(path)?;
    let re = Regex::new("\\{|\\}|\\(|\\)|;|int|return|[a-zA-z]\\w*|[0-9]+")?;
    let vec = re.find_iter(&content).map(|mat| mat.as_str().to_string()).collect();
    Ok(vec)
}