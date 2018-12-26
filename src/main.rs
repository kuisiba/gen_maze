extern crate gif;
extern crate rand;
extern crate structopt;
mod gen_gif;
mod maze;
use structopt::StructOpt;

const FOURK: (usize, usize) = (3840, 2160);

#[derive(StructOpt, Debug)]
struct Opt {
    /// if on, output gif.
    /// its size is n-power px of width and height
    /// ex) -i 3 -w 17 -h 17 -> 51px * 51px gif file
    #[structopt(short = "i", long = "img", default_value = "0")]
    image: u8,

    /// must be Odd number & 5 <= width <= 255
    #[structopt(short = "w", long = "width", parse(from_str = "parse"))]
    width: u8,

    /// must be Odd number & 5 <= height <= 255
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
    if FOURK.0 < opt.width as usize * opt.image as usize
        || FOURK.1 < opt.height as usize * opt.image as usize
    {
        eprintln!("too big!");
        std::process::exit(1);
    }
    let maze = maze::gen_maze(opt.width, opt.height);
    if opt.image == 0 {
        for i in maze.iter() {
            for j in i.iter() {
                print!("{:?}", j);
            }
            println!("");
        }
    } else {
        let mut image = std::fs::File::create("./maze.gif").unwrap();
        let scale = opt.image;
        gen_gif::gen_gif(&mut image, &maze, scale);
    }
}
