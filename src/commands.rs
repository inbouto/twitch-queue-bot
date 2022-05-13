

#[derive(Clone)]
pub struct Command {
    keyword: String,
    args: Vec<String>,
    usage: Option<String>,
    executor: fn (&[String]) -> Result<String, String>,
}

impl Command {
    pub fn new(keyword: &str, args: Vec<String>, usage: Option<String>, executor: fn (&[String]) -> Result<String, String>) -> Self{
        Command{
            keyword: keyword.to_string(),
            args: args,
            usage: usage,
            executor: executor,
        }
    }
    pub fn execute(&self, args: &Vec<String>) -> Result<String, String> {
        if args.len() < self.args.len() {
            return Err(format!("Needs {} arguments ; {} provided", self.args.len(), args.len()));
        }
        (self.executor)(&args[..self.args.len()])
    }
    pub fn get_keyword(self) -> String {
        self.keyword.clone()
    }
}

pub fn ping(args: &[String]) -> Result<String, String> {
    Ok("Pong!".to_string())
}