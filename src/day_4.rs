use crate::util::read_lines;


const BOARD_DIMENSIONS: u32 = 5;

#[derive(Debug, Default)]
struct BingoState {
    number_picks: Vec<u32>,
    boards: Vec<Board>
}

impl BingoState {
    fn pick_all(&mut self) -> u32 {
        let len_picks = self.number_picks.len();
        for idx in 0..len_picks {
            let num = self.number_picks[idx];
            if let Some(res) = self.check_number(num) {
                return res
            }
        }
        
        panic!("found no winning board")
    }
    fn check_number(&mut self, num: u32) -> Option<u32> {
        for board in &mut self.boards {
            if board.check_number(num) {
                return Some(board.get_unmarked_sum() * num);
            }
        }
        
        None
    }
}

#[derive(Debug, Default)]
struct Board {
    numbers: Vec<u32>,
    selected_numbers: Vec<bool>
}

impl Board {
    fn check_number(&mut self, num: u32) -> bool {
        for (idx, val) in self.numbers.iter().enumerate() {
            if *val == num {
                self.selected_numbers[idx] = true;
                let x = idx as u32 % BOARD_DIMENSIONS;
                let y = idx as u32 / BOARD_DIMENSIONS;
                if self.check_bingo(x, y) {
                    return true
                }
            }
        }
        
        false
    }
    fn select(&mut self, x: u32, y: u32) {
        self.selected_numbers[(x + (y * BOARD_DIMENSIONS)) as usize] = true;
    }
    fn get(&self, x: u32, y: u32) -> u32 {
        self.numbers[(x + (y * BOARD_DIMENSIONS)) as usize]
    }
    fn is_selected(&self, x: u32, y: u32) -> bool {
        self.selected_numbers[(x + (y * BOARD_DIMENSIONS)) as usize]
    }
    fn check_bingo(&self, x: u32, y: u32) -> bool {
        self.check_bingo_x(x) || self.check_bingo_y(y)
    }
    fn check_bingo_x(&self, x: u32) -> bool {
        for i in 0..BOARD_DIMENSIONS {
            if !self.selected_numbers[(x + (i * BOARD_DIMENSIONS)) as usize] {
                return false
            }
        }
        true
    }
    fn check_bingo_y(&self, y: u32) -> bool {
        for i in 0..BOARD_DIMENSIONS {
            if !self.selected_numbers[(i + (y * BOARD_DIMENSIONS)) as usize] {
                return false
            }
        }
        true
    }
    fn get_unmarked_sum(&self) -> u32 {
        let mut sum: u32 = 0;
        for (idx, selected) in  self.selected_numbers.iter().enumerate() {
            if !*selected {
                sum += self.numbers[idx]
            }
        }
        
        sum
    }
    fn for_numbers(nums: Vec<u32>) -> Self {
        let mut board = Board::default();
        board.numbers = nums;
        board.selected_numbers = vec![false; board.numbers.len()];

        board
    }
}

fn read_from_file(file_name: &str) -> BingoState {
    let mut state = BingoState::default();
    let lines: Vec<String> = read_lines(file_name)
        .expect("couldn't read file")
        .filter_map(|x| x.ok())
        .collect();
    let picks: Vec<u32> = lines[0].split(",").filter_map(|n| n.parse::<u32>().ok()).collect();

    state.number_picks = picks;

    state.boards = lines[1..].chunks((BOARD_DIMENSIONS + 1) as usize)
    .map(|chunk| {
        let numbers = chunk[1..].iter().flat_map(|s| s.split(" "))
        .filter_map(|s| s.parse::<u32>().ok())
        .collect();

        Board::for_numbers(numbers)
    })
    .collect();


    state
}

pub fn execute_part_1() {
   let mut state = read_from_file("./day_4_input.txt");

   println!("Day 4 Part 1: {}", state.pick_all());
}