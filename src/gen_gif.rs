use gif::*;

pub fn gen_gif(image: &mut std::fs::File, maze: &Vec<Vec<u8>>, scale: u8) {
    let color_map = &[0xFF, 0xFF, 0xFF, 0xB0, 0xC4, 0xDE];
    let width = (maze[0].len() * scale as usize) as u16;
    let height = (maze.len() * scale as usize) as u16;
    let state = make_state(maze, scale);
    let mut encoder = Encoder::new(image, width, height, color_map).unwrap();
    encoder.set(Repeat::Infinite).unwrap();
    let mut frame = Frame::default();
    frame.width = width;
    frame.height = height;
    frame.buffer = std::borrow::Cow::Borrowed(&state[..]);
    encoder.write_frame(&frame).unwrap();
}

fn make_state(maze: &Vec<Vec<u8>>, scale: u8) -> Vec<u8> {
    let mut powered_v = vec![vec![0; maze[0].len() * scale as usize]; maze.len() * scale as usize];
    for i in 0..maze.len() {
        for j in 0..maze[0].len() {
            for k in 0..scale {
                for l in 0..scale {
                    if maze[i][j] == 0 {
                        powered_v[i * scale as usize + k as usize]
                            [j * scale as usize + l as usize] = maze[i][j];
                    } else {
                        powered_v[i * scale as usize + k as usize]
                            [j * scale as usize + l as usize] = maze[i][j];
                    }
                }
            }
        }
    }
    let mut v = Vec::new();
    for i in 0..powered_v.len() {
        for j in 0..powered_v[0].len() {
            v.push(powered_v[i][j]);
        }
    }
    v
}
