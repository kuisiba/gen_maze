extern crate rand;
extern crate structopt;
mod maze;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Opt {
    //#[structopt(short = "i", long = "img")]
    //image: bool,
    ///must be Odd number & 5 <= width <= 255
    #[structopt(short = "w", long = "width", parse(from_str = "parse"))]
    width: u8,

    ///must be Odd number & 5 <= height <= 255
    #[structopt(short = "h", long = "height", parse(from_str = "parse"))]
    height: u8,
}
fn parse(src: &str) -> u8 {
    match src.parse::<u8>() {
        Ok(n) => {
            if n % 2 == 1 {
                if n < 5 {
                    eprintln!("error! number must be greater than 4");
                    eprintln!("try --help");
                    std::process::exit(1);
                } else {
                    n
                }
            } else {
                eprintln!("error! width and height must be Odd number");
                eprintln!("try --help");
                std::process::exit(1);
            }
        }
        Err(e) => {
            eprintln!("error! {}", e);
            eprintln!("try --help");
            std::process::exit(1)
        }
    }
}

fn main() {
    let opt = Opt::from_args();
    eprintln!("opt.width: {:?}, opt.height: {:?}", opt.width, opt.height);
    let maze = maze::gen_maze(opt.width, opt.height);
    for i in maze.iter() {
        for j in i.iter() {
            print!("{:?}", j);
        }
        println!("");
    }
}
