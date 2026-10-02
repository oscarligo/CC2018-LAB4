/// Un patrón se escribe como arte ASCII:
/// `#` representa una célula viva y cualquier otro carácter una célula muerta.
pub type Pattern = &'static [&'static str];

#[rustfmt::skip]
pub const GLIDER: Pattern = &[
    ".#.",
    "..#",
    "###",
];

#[rustfmt::skip]
pub const BLINKER: Pattern = &[
    "###",
];

#[rustfmt::skip]
pub const TOAD: Pattern = &[
    ".###",
    "###.",
];

#[rustfmt::skip]
pub const BEACON: Pattern = &[
    "##..",
    "##..",
    "..##",
    "..##",
];


#[rustfmt::skip]
pub const PULSAR: Pattern = &[
    ".................",
    "...###.....###...",
    ".................",
    ".#....#...#....#.",
    ".#....#...#....#.",
    ".#....#...#....#.",
    "...###.....###...",
    ".................",
    "...###.....###...",
    ".#....#...#....#.",
    ".#....#...#....#.",
    ".#....#...#....#.",
    ".................",
    "...###.....###...",
];


#[rustfmt::skip]
pub const GOSPER_GLIDER_GUN: Pattern = &[
    "........................#...........",
    "......................#.#...........",
    "............##......##............##",
    "...........#...#....##............##",
    "##........#.....#...##..............",
    "##........#...#.##....#.#...........",
    "..........#.....#.......#...........",
    "...........#...#....................",
    "............##......................",
];

#[rustfmt::skip]
pub const ACORN: Pattern = &[
    ".#.....",
    "...#...",
    "##..###",
];

pub struct GameOfLife {
    width: usize,
    height: usize,
    cells: Vec<u8>,
    next_cells: Vec<u8>,
}

impl GameOfLife {
    pub fn new(width: usize, height: usize) -> Self {
        let cell_count = width * height;

        Self {
            width,
            height,
            cells: vec![0; cell_count],
            next_cells: vec![0; cell_count],
        }
    }

    pub fn place_pattern(&mut self, pattern: Pattern, x: isize, y: isize) {
        for (pattern_y, row) in pattern.iter().enumerate() {
            for (pattern_x, cell) in row.bytes().enumerate() {
                if cell == b'#' {
                    self.set_alive(x + pattern_x as isize, y + pattern_y as isize);
                }
            }
        }
    }

    pub fn step(&mut self) {
        for y in 0..self.height {
            for x in 0..self.width {
                let index = self.index(x, y);
                let neighbors = self.live_neighbor_count(x, y);

                self.next_cells[index] = match (self.cells[index], neighbors) {
                    (1, 2 | 3) | (0, 3) => 1,
                    _ => 0,
                };
            }
        }

        std::mem::swap(&mut self.cells, &mut self.next_cells);
    }

    pub fn live_cells(&self) -> impl Iterator<Item = (usize, usize)> + '_ {
        self.cells
            .iter()
            .enumerate()
            .filter(|(_, cell)| **cell == 1)
            .map(|(index, _)| (index % self.width, index / self.width))
    }

    fn set_alive(&mut self, x: isize, y: isize) {
        if x >= 0 && y >= 0 && x < self.width as isize && y < self.height as isize {
            let index = self.index(x as usize, y as usize);
            self.cells[index] = 1;
        }
    }

    fn live_neighbor_count(&self, x: usize, y: usize) -> u8 {
        let mut neighbors = 0;

        for offset_y in -1..=1 {
            for offset_x in -1..=1 {
                if offset_x == 0 && offset_y == 0 {
                    continue;
                }

                let neighbor_x = x as isize + offset_x;
                let neighbor_y = y as isize + offset_y;

                if neighbor_x >= 0
                    && neighbor_y >= 0
                    && neighbor_x < self.width as isize
                    && neighbor_y < self.height as isize
                {
                    neighbors += self.cells[self.index(neighbor_x as usize, neighbor_y as usize)];
                }
            }
        }

        neighbors
    }

    fn index(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }
}


