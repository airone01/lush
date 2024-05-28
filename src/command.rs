use std::fmt::{self, Debug, Formatter};

pub struct Command {
    pub keyword: String,
    pub args: Vec<String>,
}

impl Debug for Command {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "Command {{ keyword: {}, args: {:?} }}",
            self.keyword, self.args
        )
    }
}
