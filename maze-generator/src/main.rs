enum Element {
    Wall,
    Passage,
}

struct Maze {
    width: usize,
    height: usize,
    data: Vec<Element>,
}

impl Maze {
    fn init(height: usize, width: usize) -> Maze {
        let mut maze = Maze {
            width: width,
            height: height,
            data: Vec::new(),
        };
        for y in 0..height {
            for x in 0..width {
                let val =
                    if y == 0 || y == height - 1{
                        Element::Passage
                    } else if x == 0 || x == width - 1 {
                        Element::Passage
                    } else {
                        Element::Wall
                    };
                maze.data.push(val);
            }
        }
        maze
    }

    fn show(&self) {
        for y in 0 .. self.height {
            for x in 0 .. self.width {
                match self.data[y * self.width + x] {
                    Element::Wall => print!("[]"),
                    Element::Passage => print!("  "),
                }
            }
            println!("")
        }
    }

    fn is_wall(&self, x: isize, y: isize) -> bool {
        let (ux, uy) = (x as usize, y as usize);
        match self.data[(uy * self.width + ux) as usize] {
            Element::Wall => true,
            Element::Passage => false,
        }
    }

    fn carve<R: rand::Rng>(&mut self, rng: &mut R, x: usize, y: usize) {
        let x_dirs = [1, -1, 0, 0];
        let y_dirs = [0, 0, 1, -1];
        self.data[y * self.width + x] = Element::Passage;
        let d = rng.gen::<usize>() % 4;
        for i in 0..4 {
            let dx: isize = x_dirs[(d + i) % 4];
            let dy: isize = y_dirs[(d + i) % 4];
            let x2 = (x as isize) + dx;
            let y2 = (y as isize) + dy;
            if self.is_wall(x2, y2) {
                let nx = x2 + dx;
                let ny = y2 + dy;
                if self.is_wall(nx, ny) {
                    let index = (y2 as usize) * self.width + (x2 as usize);
                    self.data[index] = Element::Passage;
                    self.carve(rng, nx as usize, ny as usize);
                }
            }
        }
    }

    fn generate(width: i64, height: i64) -> Maze {
        let width = width as usize;
        let height = height as usize;
        let mut maze = Maze::init(width, height);
        let mut rng = rand::thread_rng();
        maze.carve(&mut rng, 2, 2);
        maze.data[1 * width + 2] = Element::Passage;
        maze.data[(height - 2) * width + (width - 3)] = Element::Passage;
        maze
    }
}

fn main() {
    let width: i64 = 39;
    let height: i64 = 31;
    let maze: Maze = Maze::generate(width, height);
    maze.show();
}
