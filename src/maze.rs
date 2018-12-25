use rand::seq::SliceRandom;

pub fn gen_maze(w: u8, h: u8) -> Vec<Vec<u8>> {
    let mut v = vec![vec![1; w as usize]; h as usize];
    let mut room_count: usize = 0;
    let mut walls: Vec<(usize, usize)> = vec::new();
    for i in 1..h - 1 {
        for j in 1..w - 1 {
            if j % 2 == 1 && i % 2 == 1 {
                v[i as usize][j as usize] = 0;
                room_count += 1;
            }
            if (j % 2 == 1 && i % 2 == 0) || (j % 2 == 0 && i % 2 == 1) {
                walls.push((j as usize, i as usize));
            }
        }
    }
    let mut uf = UnionFind::new(room_count);
    v
}
#[derive(Debug)]
struct UnionFind {
    par: Vec<usize>,
    rank: Vec<usize>,
}
impl UnionFind {
    fn new(size: usize) -> UnionFind {
        let mut par = Vec::with_capacity(size);
        let mut rank = Vec::with_capacity(size);
        for i in 0..size {
            par.push(i);
            rank.push(0);
        }
        UnionFind {
            par: par,
            rank: rank,
        }
    }
    fn root(&mut self, x: usize) -> usize {
        if self.par[x] == x {
            x
        } else {
            let par = self.par[x];
            let ppar = self.root(par);
            self.par[x] = ppar;
            ppar
        }
    }
    fn same(&mut self, x: usize, y: usize) -> bool {
        return self.root(x) == self.root(y);
    }
    fn unite(&mut self, x: usize, y: usize) {
        let px = self.root(x);
        let py = self.root(y);
        if px == py {
            return;
        }
        if self.rank[px] < self.rank[py] {
            self.par[px] = py;
        } else {
            self.par[py] = px;
            if self.rank[px] == self.rank[py] {
                self.rank[px] += 1;
            }
        }
    }
}
