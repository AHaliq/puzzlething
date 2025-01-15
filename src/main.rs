use std::collections::HashSet;
use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Tile {
    Blocked,
    Filled,
    Empty,
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let symbol = match self {
            Tile::Blocked => ' ',
            Tile::Filled => 'O',
            Tile::Empty => '-',
        };
        write!(f, "{}", symbol)
    }
}

#[derive(Clone, Debug, Copy, PartialEq, Eq, Hash)]
pub struct Grid {
    grid: [[Tile; 7]; 7],
    filled_count: u32,
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in 0..7 {
            for x in 0..7 {
                write!(f, "{}", self.grid[x][y])?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Action {
    x: usize,
    y: usize,
    dir: Direction,
}

impl fmt::Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let dir = match self.dir {
            Direction::Up => "↑",
            Direction::Down => "↓",
            Direction::Left => "←",
            Direction::Right => "→",
        };
        write!(f, "({}, {}) {}", self.x, self.y, dir)
    }
}

impl Default for Grid {
    fn default() -> Self {
        Self::new()
    }
}

impl Grid {
    pub fn new() -> Grid {
        let mut grid = [[Tile::Filled; 7]; 7];
        grid[3][3] = Tile::Empty;
        grid[0][0] = Tile::Blocked;
        grid[0][1] = Tile::Blocked;
        grid[1][0] = Tile::Blocked;
        grid[1][1] = Tile::Blocked;
        grid[5][0] = Tile::Blocked;
        grid[5][1] = Tile::Blocked;
        grid[6][0] = Tile::Blocked;
        grid[6][1] = Tile::Blocked;
        grid[0][5] = Tile::Blocked;
        grid[0][6] = Tile::Blocked;
        grid[1][5] = Tile::Blocked;
        grid[1][6] = Tile::Blocked;
        grid[5][5] = Tile::Blocked;
        grid[5][6] = Tile::Blocked;
        grid[6][5] = Tile::Blocked;
        grid[6][6] = Tile::Blocked;
        Grid {
            grid,
            filled_count: 32,
        }
    }

    pub fn tile_actions(&self, x: usize, y: usize) -> Vec<Action> {
        if self.grid[x][y] != Tile::Filled {
            return Vec::new();
        }
        let mut actions = Vec::new();
        if x > 1 && self.grid[x - 1][y] == Tile::Filled && self.grid[x - 2][y] == Tile::Empty {
            actions.push(Action {
                x,
                y,
                dir: Direction::Left,
            });
        }
        if x < 5 && self.grid[x + 1][y] == Tile::Filled && self.grid[x + 2][y] == Tile::Empty {
            actions.push(Action {
                x,
                y,
                dir: Direction::Right,
            });
        }
        if y > 1 && self.grid[x][y - 1] == Tile::Filled && self.grid[x][y - 2] == Tile::Empty {
            actions.push(Action {
                x,
                y,
                dir: Direction::Up,
            });
        }
        if y < 5 && self.grid[x][y + 1] == Tile::Filled && self.grid[x][y + 2] == Tile::Empty {
            actions.push(Action {
                x,
                y,
                dir: Direction::Down,
            });
        }
        actions
    }

    pub fn valid_actions(&self) -> Vec<Action> {
        let mut actions = Vec::new();
        for x in 0..7 {
            for y in 0..7 {
                actions.append(&mut self.tile_actions(x, y));
            }
        }
        actions
    }

    pub fn verify_action(&self, action: Action) -> bool {
        match action.dir {
            Direction::Up => {
                if action.y < 2 {
                    return false;
                }
                if self.grid[action.x][action.y - 1] != Tile::Filled {
                    return false;
                }
                if self.grid[action.x][action.y - 2] != Tile::Empty {
                    return false;
                }
            }
            Direction::Down => {
                if action.y > 4 {
                    return false;
                }
                if self.grid[action.x][action.y + 1] != Tile::Filled {
                    return false;
                }
                if self.grid[action.x][action.y + 2] != Tile::Empty {
                    return false;
                }
            }
            Direction::Left => {
                if action.x < 2 {
                    return false;
                }
                if self.grid[action.x - 1][action.y] != Tile::Filled {
                    return false;
                }
                if self.grid[action.x - 2][action.y] != Tile::Empty {
                    return false;
                }
            }
            Direction::Right => {
                if action.x > 4 {
                    return false;
                }
                if self.grid[action.x + 1][action.y] != Tile::Filled {
                    return false;
                }
                if self.grid[action.x + 2][action.y] != Tile::Empty {
                    return false;
                }
            }
        }
        true
    }

    pub fn perform_action(&self, action: Action) -> Self {
        assert!(self.verify_action(action));
        let mut new_grid = self.grid;
        new_grid[action.x][action.y] = Tile::Empty;
        match action.dir {
            Direction::Up => {
                new_grid[action.x][action.y - 1] = Tile::Empty;
                new_grid[action.x][action.y - 2] = Tile::Filled;
            }
            Direction::Down => {
                new_grid[action.x][action.y + 1] = Tile::Empty;
                new_grid[action.x][action.y + 2] = Tile::Filled;
            }
            Direction::Left => {
                new_grid[action.x - 1][action.y] = Tile::Empty;
                new_grid[action.x - 2][action.y] = Tile::Filled;
            }
            Direction::Right => {
                new_grid[action.x + 1][action.y] = Tile::Empty;
                new_grid[action.x + 2][action.y] = Tile::Filled;
            }
        }
        Grid {
            grid: new_grid,
            filled_count: self.filled_count - 1,
        }
    }

    pub fn filled_count(&self) -> u32 {
        let mut count = 0;
        for x in 0..7 {
            for y in 0..7 {
                if self.grid[x][y] == Tile::Filled {
                    count += 1;
                }
            }
        }
        count
    }
}

#[derive(Clone, Debug)]
pub struct GameTree {
    state: Grid,
    history: Vec<Action>,
}

impl Default for GameTree {
    fn default() -> Self {
        GameTree::new(Grid::new(), Vec::new())
    }
}

impl GameTree {
    pub fn new(state: Grid, history: Vec<Action>) -> GameTree {
        GameTree { state, history }
    }

    pub fn search(&self) -> Option<GameTree> {
        let term_x = 3;
        let term_y = 3;
        let mut memo = HashSet::new();
        let mut queue: Vec<GameTree> = Vec::new();
        memo.insert(self.state);
        queue.push(self.clone());
        loop {
            match queue.pop() {
                None => return None,
                Some(cur) => {
                    let actions = cur.state.valid_actions();
                    for action in actions {
                        let new_state = &cur.state.perform_action(action);
                        if !memo.contains(new_state) {
                            memo.insert(*new_state);
                            let mut new_history = cur.history.clone();
                            new_history.push(action);
                            queue.push(GameTree::new(*new_state, new_history));
                        }
                    }
                    if cur.state.filled_count <= 1 && cur.state.grid[term_x][term_y] == Tile::Filled
                    {
                        return Some(cur);
                    }
                }
            }
        }
    }
}

fn main() {
    let g = GameTree::default().search().unwrap();
    println!("{}", g.state);
    println!("Finished in {} moves\n", g.history.len());
    println!("(x, y) direction");
    for action in g.history {
        println!("{}", action);
    }
}
