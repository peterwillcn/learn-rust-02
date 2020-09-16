// target/release/hello-command even 1 2 3 4
use std::fmt::{self, Display, Formatter};
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "app")]
pub struct Args {
    #[structopt(subcommand)]
    pub command: Command,
}

#[derive(StructOpt)]
pub struct Elements {
    pub elements: Vec<i32>,
}

#[derive(StructOpt)]
pub enum Command {
    //偶数
    #[structopt(name = "even")]
    Even(Elements),
    //奇数
    #[structopt(name = "odd")]
    Odd(Elements),
}

//显示
impl Display for Elements {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "[{:#?}]", self.elements)
    }
}

fn main() {
    let arg = Args::from_args();
    let arg2 = arg;
    match arg2.command {
        Command::Even(e) => {
            print!("Operants: {}", e);
            print!("Result: ");
            //迭代器访问
            for i in &e.elements {
                if i % 2 == 0 {
                    print!(" {} ", i);
                }
            }
        }
        Command::Odd(e) => {
            print!("Operants: {}", e);
            print!("Result: ");
            for i in &e.elements {
                if i % 2 == 1 {
                    print!(" {} ", i);
                }
            }
        }
    }
}
