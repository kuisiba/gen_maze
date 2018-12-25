extern crate rand;
extern crate structopt;
mod maze;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Opt {
    //#[structopt(short = "i", long = "img")]
    //image: bool,
    #[structopt(short = "w", long = "width")]
    width: u8,

    #[structopt(short = "h", long = "height")]
    height: u8,
}

fn main() {
    let opt = Opt::from_args();
    //eprintln!("{:?}", opt);
    //hとwは奇数、かつ、5以上
    let maze = maze::gen_maze(opt.width, opt.height);
    for i in maze.iter() {
        for j in i.iter() {
            print!("{:?}", j);
        }
        println!("");
    }
}
