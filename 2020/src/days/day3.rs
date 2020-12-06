const TREE_MAP: [&str; 323] = [
    ".#......##..#.....#....#.#.#...",
    ".#.#...#.##.#..........#...##..",
    ".........#.....#.####........#.",
    ".......#.#...#.#...............",
    "..#....#...#.#...#.#...#.#.....",
    "...#...........#..#.........#.#",
    "....#..#....#..#..#.#...#..##..",
    "#...........#..#.....#.......#.",
    "#..#...#...#.###...#...#.#...#.",
    "#...#.#.......#...#...#...##.##",
    "..#..................#.#.#....#",
    "..#.##....#........##..........",
    ".....#....#....#.#.......#.....",
    "##.#..##.#.....###.......#.....",
    "......#...###....#..#.#...#....",
    "..............#.........#.##...",
    "#......#.............#....#...#",
    ".#..#......#.###....#...#.....#",
    "..#........#.....#.....#...#..#",
    ".......#...#..............#..#.",
    "..#...#........#...##........#.",
    ".#........#....#......#......#.",
    "....#..#.###.......##....#.#..#",
    "..#..###..#....................",
    "......#...#....#.........#.#...",
    "....#.##................#..#...",
    "....#......######.....#........",
    ".#......##.......#....#..##.###",
    "..#...##.###..#.......#....#...",
    "....#.###...#.#.#........#.....",
    "...###...#.......#..........#.#",
    "..........#...#..........##.#..",
    "..#....#........#.....#....#..#",
    "..#...#.#....##..#...##....#...",
    "........##...#..##.....#.......",
    "###.......#.#...#...#.......#.#",
    "....#.#....##.###........#.....",
    ".....#..............#....##..##",
    "#......#.#....#.#......#.....##",
    ".....#....#..#......#...#......",
    "..#.##..#.....#..#....#......#.",
    ".....#.#.#..........##....#....",
    ".........#..#..........#.#.....",
    ".##..#...#......#.#..#....#....",
    "#.#..##.......#.#......##......",
    "..#.#....#.#.....#.............",
    ".#.........#.......#..#.#......",
    "##.........#..##.#......#......",
    "#..#.....#...#.....#.........#.",
    "..........#..##..##.#..##...###",
    "..##.....#...#..##...##.#.#....",
    "..#..........#.#.....##.#....#.",
    ".##..#..#.........###.......#..",
    "......##....#...##....##.......",
    ".....#.#.##...............#....",
    "#..#......#.....#..#..#.#.....#",
    ".....##.#....#.#.....#.#.#.....",
    "....#..#.#..##....#.....#....#.",
    "#...#.....#....#....#.#.#......",
    ".....#................#.......#",
    ".......#..#.#...#.#......#..#.#",
    "...........#....#....###...#.#.",
    "#.##....##..###.#.#......#.##.#",
    "..##...#.#..#..#...#.....#.#.#.",
    "#.....###.#..#.#...#.#......#.#",
    "..##.#...#...#.#.....#.#.......",
    "#....#...#.##......#.#......#..",
    "..#.....##.....#...............",
    ".....###...##.#...........#....",
    "...#..##.....##....#...........",
    ".....#..#......#..........#....",
    "....##..##.#...#...#.#.....#.##",
    ".#.....###..###.#...#.#..#....#",
    ".#..........#...#..#.#.#..#...#",
    ".##.##..#..#....#....####......",
    "....#..#.#..........#..........",
    "###...#.#..#..#...#..###.......",
    "####.#...#....#..#...#..#......",
    ".....##....#.#...#....##....##.",
    "....#.#.##....#.##..#....#.#.#.",
    "#......#..#.###....#####.##....",
    "..##..#.#.#..#........##.##..##",
    "#.#...#..#..#......#..#.....#..",
    ".###.....#.#....#.#..##.....#.#",
    "....#......#.#...#...#.#....#.#",
    ".....#.###.##..................",
    ".#..........#........#.#...##.#",
    ".##......#.#.#..#....##.###..#.",
    "..#.##....#....#.........#.#..#",
    "........#..#..#.#.####.....##..",
    "#..#.##.#......#.#..##.#...#..#",
    "..#.#.##..#.##..........#......",
    "##.#.....#.#.##..#..##.....##.#",
    ".##........#..#.....#...#.##.##",
    "...#....#.#.#.........##.....#.",
    "...#....#.#....#...#..#........",
    ".....#...#..#...#.##......##...",
    "##.........#......#..........##",
    ".#......#.....##....#.#.#.....#",
    "..#.###......#..#.#....#.....#.",
    ".#.......#...#...#.#.#.#..##...",
    "...#..............#...###.....#",
    "...##......#.#..#.#........#.#.",
    "..##.#....#..........##...#.#..",
    "..#...#.....#.######...##...#..",
    "#...#...#............#.....#...",
    ".###..###.##..#.........#......",
    ".#........##..#....#...#.#..##.",
    "#.#.##.#.#...###...............",
    "..#.#.#......#.#.#....#.....#.#",
    ".#...........#.##.#..#.###.....",
    ".###.#....#...........##.#.#...",
    ".#...#...........#..##.........",
    ".#...#.#...........#..###....#.",
    ".##.......#.....#.....##....#..",
    "#.......#........#...##.##..#.#",
    "....#..###..#.....##.......#...",
    "......###.#...#..#....#.#...#..",
    "..#..#.......##...#.#.#...#....",
    "......#..#.......#.......##.#..",
    "#.#....###.....#...#..#...#....",
    "#...#.##.#........#..........##",
    ".....#.#.##.#.#..#..##.......##",
    ".#.#.......##....#.#...........",
    "#..##.............##...#.#..#..",
    "#...........#.#......#.##.##..#",
    "...#...#...........#....###.#.#",
    ".##..#.#.#....#....#####.......",
    "..#...#.....#.#....#...........",
    ".#..#........#.....#.#......#..",
    ".#.........#...#...#.#.#..#....",
    ".##.##......#.#...#.......#...#",
    ".##...#..#..........#...#.....#",
    "#..........#..#...#.#......#...",
    "....##......#...##..##..#....#.",
    ".##.......#...#.#..##..#..#....",
    ".#.#................#....#.....",
    "..#..#..###.......#............",
    "...##.....#..#......#....#.....",
    "....#...###...#....#..##...#.#.",
    "#.........#.......#...#....#...",
    ".#.#...#.#....##....#.#..##.#..",
    "...#..#..#....#..#.#..##.....##",
    "..#..#.#.#....#...#....#..#....",
    "......###.....#...##.#..#.#...#",
    ".#.#.#..#.##........#.#....#...",
    ".#..........#....#.#.......#...",
    "#.....#........#........#....#.",
    ".#.#..#...#...................#",
    "....####..#..#..#..#....#..#.#.",
    "..##.#..........#.##..#.....##.",
    "..................##..........#",
    "....##....###.....#..#...#.#...",
    ".##.........#..#...............",
    "....##..###....#.##............",
    "#.#...###.#..##...#...........#",
    ".....#..#......#.....#.........",
    "..#..##...#.....#.....#.#......",
    "......#....###.#..#.#.#....#..#",
    "#...#.......#.##.....#.........",
    ".#.#..#...#.............##.....",
    "......#..............#.....#..#",
    "......#......###....#...#......",
    ".....#.....#...#.......###.....",
    "#..........##......##.#.#.....#",
    "....#.......#..#......#.......#",
    "..#...#.###...........#..#.###.",
    ".....#...#.#...........#.#...##",
    "........#.#.#........#.#.....#.",
    "....##..##.#.#..#.#....#.#.##..",
    "..#.#.#......##.....#...#.#...#",
    "##...#..#......#.#.#..#...#....",
    "....#..##...........#..#..#..#.",
    ".#..##...#...#...##.#..#.#....#",
    ".#.....####.#..#..#....##..#.#.",
    ".#....#..#......#.....#.#.#....",
    "....#..#.....#......#..........",
    "..#.#..###.....#...#...#.....##",
    "..#.#...##..#...........####...",
    ".#.##....##.#......#.....##.#..",
    "#.##..#....#.###..........##...",
    ".###...#......#.#....##........",
    "...................#..#.....#..",
    "#.#...#.#..#.....#...#..####.##",
    "....#.##..##...##.##.....#.....",
    ".#...#.##...........#.......##.",
    "###..#.....##...#.........##...",
    ".###....##...###...............",
    ".#....#####........#.#.#.##....",
    ".#.#....####.##........#.......",
    ".....#......#..................",
    "......###.....##......#..##.#..",
    "....#.#...........##.#....##.#.",
    "...................#.#.#.......",
    "#.#.#........#..#.......##.....",
    "..#...#...#....#......#....##.#",
    "#..#..............#......#....#",
    "......#.........##.............",
    ".....#.#....##..#.......#......",
    "......#.......#...........#....",
    "....#....#.#..##.#....#...#....",
    "#.#.#..#..#.#.#.#...#....#....#",
    ".#.#....#...#.#..#......#.....#",
    ".#...........#.#....##.....#...",
    "........#...#....#....##.....##",
    "#..#..........#..#..#.....#....",
    "#.#.###..........#.##....#...##",
    "..#................#.##.##.....",
    "..#...#.##...##...#.........#..",
    "#....#......#......#.........#.",
    "##...#...##.#.........#......#.",
    ".......#.....#.................",
    "...#...#.....##.........#.#..#.",
    "..#......#...#.......#......#.#",
    "#.......#...#.##.#..##..#......",
    ".#.#............#...###..#.....",
    "...#.......##.......#....#..#..",
    ".....#..#.#....#.#.............",
    "#....#...##.##....#....##......",
    "........#......#.......#....#..",
    "..#..#..##......##.#..#.#..##..",
    "....##......#.##.##......#.....",
    "........##.#...#.....#.......#.",
    "..##.#....#..#......#.##.......",
    "..##.####.#...#.#....#.........",
    ".#........#.....#..#....#...#.#",
    "###....##......#..#..#.##..#...",
    "..........###.#..#..#....#.....",
    "..#.........#....#.....#....#.#",
    ".#...#.#.....##.#...#...#.#..#.",
    "....##......##.##.#.....#..#...",
    "....#.##...##.......#..##......",
    "#..........#..#....#.......#.#.",
    "..#.....#.................#....",
    "..........#.#.#.....#.#....#..#",
    ".......#..........#.##....#....",
    "#..#.....#.......#........#....",
    "#.....##..#.........##..#..#.#.",
    ".##.#...#..........#....#......",
    "....#..#.#......#.##..#..#.##..",
    "...##.####....#.....#.#...##...",
    "..#.#....#.#........#..........",
    "#...#.#.##.##....##..#...#...#.",
    "...#.#.......#..#...#..#..##..#",
    ".....#....#........###.....#...",
    ".......#..#.##....#.#.....#....",
    "....##....#....#.......#.....#.",
    ".........#........###...##.....",
    "#.#..#...##.........#.#..#....#",
    "...##...........#.........#...#",
    "......#.#.#.........#..#.#.#...",
    "........##.###....#..#.......#.",
    "....#.#...#......#..#........##",
    ".#....##....#...#.##.........#.",
    "####.#..#...........##.#.#.....",
    "...#....#..#.....#..##.####.#..",
    ".##...#...........#.#.........#",
    "#.#..#..#...#.#.#.........#..#.",
    "#......###............#...#....",
    "..#.......#....#...#...#..#...#",
    "#.#.#...##..#...#...#.......##.",
    "......#.#.......#..........#.#.",
    "...............#...#..#...#.#..",
    ".#.#...##.####..##.##....#..##.",
    "#..####.......##.#........#...#",
    "......###....##...#.#..#.##....",
    ".##.....###..#...#.###.###.....",
    "..#...#.....#...#..#..##..#....",
    "...#...##.....##........#.#.##.",
    ".#...#..#....#....#..###....#.#",
    "..#.#.#.#.#..........#.#..#..##",
    ".......###.....................",
    "##.#......#.##.....#.........#.",
    "......................#.#.....#",
    "#..#........##.......#..##..#.#",
    "#.#.#.....##.#.##.##.#....##...",
    ".#...#.....#.........#.....#...",
    "..#.........#.##.#.###.#......#",
    ".........#..#.##...#.......###.",
    ".....##........#......#........",
    "...#.#...##...#........#.##....",
    ".........##............#.####..",
    "#....#...#...#..#....#..#.#.#.#",
    "..#.........#......#.##........",
    "....#.....#........#........#.#",
    ".##.#..#.#..#..###......###....",
    "#.###.....#.#.#.##........#..##",
    "#.#..#...##.....#....#...#.#...",
    "......#....#.....#...#.........",
    "...#........##.......#.##..####",
    "..#..#....#....#..#..#...#.##..",
    ".##.....#............#...#.....",
    "......#.......#.....#...#.#.#..",
    ".........#.....#...##..........",
    ".....#........##...........#...",
    "#.#..##.#...#....#....#........",
    "#.##..#.#.......#...#......#...",
    "...........#.#..#..#.....##.#..",
    "#....#.##.......#......#.##..#.",
    ".....#........#.##.#...#.....#.",
    ".....###..#.......##...........",
    ".........#.#.#.....#.##.......#",
    ".......#....#......#.#.....#...",
    "##........#...#..#.#.........#.",
    "##...........#.##...##......#..",
    "..#.###.#.#.#...####..#....###.",
    ".........#...#.....##....#.#.##",
    ".###..###.#.#.....#.##.........",
    "#..#...#.#.................##.#",
    "##.........#.#....#.#...#.###..",
    "#.#....#..............#.##.#...",
    "...#..#....##.#..#.......#..##.",
    ".#..#.###......##..........#..#",
    ".##....#.#....#....#.#..#......",
    ".......#.....#..#....#.##...#..",
    "#.#.#.........###..#..#.....#..",
    "...##..##...##....#..#......#..",
    "..........#....#..........#....",
    "#..##..#...#......#.....#.#....",
    "#..##..#....#.#.#...#..........",
    "......##..#.........#........#.",
    ".##..#..#......###.....#..#....",
    ".....#..#.##..........#.#..#..."
];

const TREE: &str = "#";

pub fn run() {
    run_part1();

    run_part2();
}

fn run_part1() {
    println!("Day 3 - Part 1: {} trees hit", traverse_map(3, 1));
}

fn run_part2() {
    let r1_d1 = traverse_map(1, 1);
    let r3_d1 = traverse_map(3, 1);
    let r5_d1 = traverse_map(5, 1);
    let r7_d1 = traverse_map(7, 1);
    let r1_d2 = traverse_map(1, 2);

    let product = r1_d1 * r3_d1 * r5_d1 * r7_d1 * r1_d2;

    println!("Day 3 - Part 2: {}, {}, {}, {}, and {} trees hit. The product is {}", r1_d1, r3_d1, r5_d1, r7_d1, r1_d2, product);
}

fn traverse_map(right: usize, down: usize) -> u32 {
    let mut num_trees = 0;

    let mut current_idx = 0;
    for i in (0..TREE_MAP.len()).step_by(down) {
        let tree_line = TREE_MAP[i];
        current_idx %= tree_line.len();

        if tree_line[current_idx..(current_idx + 1)] == *TREE {
            num_trees += 1;
        }

        current_idx += right;
    }

    num_trees
}
