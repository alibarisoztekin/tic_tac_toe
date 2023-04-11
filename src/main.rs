use std::io;
use std::fmt;

#[derive(Clone, PartialEq)]
enum Player {
    X,
    O,
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Player::X => write!(f, "X"),
            Player::O => write!(f, "O"),
        }
    }
}

struct Board {
    cells: [[Option<Player>; 3]; 3],
}

impl Board {
    fn new() -> Board {
        Board {
            cells: [[None, None, None], [None, None, None], [None, None, None]],
        }
    }

    fn print(&self) {
        for row in &self.cells {
            for cell in row {
                print!(
                    "{} ",
                    match cell {
                        Some(player) => player.to_string(),
                        None => ".".to_string(),
                    }
                );
            }
            println!();
        }
    }

    fn set_cell(&mut self, x: usize, y: usize, player: Player) -> bool {
        if self.cells[y][x].is_none() {
            self.cells[y][x] = Some(player);
            true
        } else {
            false
        }
    }

    fn check_win(&self, player: &Player) -> bool {
        let check_line = |cells: &Vec<&Option<Player>>| {
            cells.iter().all(|cell| match cell {
                Some(cell_player) => cell_player == player,
                None => false,
            })
        };
    
        // Check rows and columns
        for i in 0..3 {
            if check_line(&self.cells[i].iter().collect::<Vec<_>>())
                || check_line(&(0..3).map(|j| &self.cells[j][i]).collect::<Vec<_>>())
            {
                return true;
            }
        }
    
        // Check diagonals
        check_line(&(0..3).map(|i| &self.cells[i][i]).collect::<Vec<_>>())
            || check_line(&(0..3).map(|i| &self.cells[i][2 - i]).collect::<Vec<_>>())
    }
    
    

    fn is_full(&self) -> bool {
        self.cells.iter().all(|row| row.iter().all(|cell| cell.is_some()))
    }
}

fn main() {
    let mut board = Board::new();
    let mut current_player = Player::X;

    loop {
        println!("Player {}'s turn", current_player);
        board.print();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let coords: Vec<usize> = input
            .trim()
            .split(' ')
            .map(|x| x.parse().expect("Invalid input"))
            .collect();

        if coords.len() == 2 && coords[0] < 3 && coords[1] < 3 {
            if board.set_cell(coords[0], coords[1], current_player.clone()) {
                if board.check_win(&current_player) {
                    println!("Player {} wins!", current_player);
                    board.print();
                    break;
                } else if board.is_full() {
                    println!("It's a draw!");
                    board.print();
                    break;
                } else {
                    current_player = match current_player {
                        Player::X => Player::O,
                        Player::O => Player::X,
                    };
                }
            } else {
                println!("Cell already occupied. Try again.");
            }
        } else {
            println!("Invalid input. Try again.");
        }
    }
}
