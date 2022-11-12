use std::{fmt::{Display}};
use serde::{Serialize, Deserialize};

// Global constants, used to scale maze size easier for testing.
const MAZE_WIDTH: usize = 8;
const MAZE_HEIGHT: usize = 8;

// Single node, represents one space of the maze
#[derive(Clone, Copy)]
struct Node{
    passable: bool,
    in_path: bool,
    searched: bool,
    is_start: bool,
    is_end: bool
}

// Represents entire maze.
#[derive(Clone, Copy)]
struct Maze{
    hallways: [[Node; MAZE_WIDTH] ; MAZE_HEIGHT],
    start: (usize, usize),
    end: (usize, usize)
}

// Template, used by serde to deserialize file input easier
// Exclusively used in load_maze to transform this into a maze object
#[derive(Serialize, Deserialize)]
struct MazeTemplate{
    walls: Vec<(usize, usize)>,
    start: (usize, usize),
    end: (usize, usize)
}

// By default, all nodes are passable
const DEFAULT: Node = Node{
    passable: true,
    in_path: false,
    searched: false,
    is_start: false,
    is_end: false
};


fn main() {
    // Used for loading maze from file
    let in_path = std::env::args().nth(1);
    let mut loaded_maze: MazeTemplate = MazeTemplate { walls: vec![], start: (0, 0), end: (1, 1) };

    // Code to write template to test_maze.json. Uncomment if you want to write a new one easier.
    // let json_maze = serde_json::to_string(&loaded_maze).unwrap();
    // std::fs::write("test_maze.json", json_maze).expect("Err writing maze template");

    // If we've been given a file to load a maze from
    if in_path != None{
        // Get the string
        let incoming = std::fs::read_to_string(in_path.unwrap()).expect("cannot read file");

        // Deserialize it to a template
        loaded_maze = serde_json::from_str(incoming.as_str()).unwrap()
    }

    // Default empty maze
    let mut maze = Maze{
        hallways: [[DEFAULT; MAZE_WIDTH]; MAZE_HEIGHT],
        start: (0, 0),
        end: (0, 0)
    };

    maze = load_maze(maze, loaded_maze);

    solve_maze(maze);
}

fn solve_maze(mut maze: Maze){
    println!("Unsolved Maze: ");
    println!("{maze}");

    let row = maze.start.0;
    let col = maze.start.1;
    fn _solve_maze(row: usize, col: usize, maze: &mut Maze) -> bool{
        // Base Case, we found the end.
        if row == maze.end.0{
            if col == maze.end.1{
                println!("Found the end!");
                return true;
            }
        }

        // If we're inside the bounds of the maze
        // No need to compare if r/c > 0 b/c of type limits
        if row < MAZE_HEIGHT{
            if col < MAZE_WIDTH{
                // If we're in a valid, passable spot
                if maze.hallways[row][col].passable{
                    // Mark we've been here
                    maze.hallways[row][col].searched = true;

                    // If we looped on ourselves, get out
                    if maze.hallways[row][col].in_path {return false}
                    
                    // Mark us as in the path
                    maze.hallways[row][col].in_path = true;
                    
                    // If we aren't on the roof
                    if row < MAZE_HEIGHT{
                        // Try going up
                        if _solve_maze(row + 1, col, maze){
                            return true;
                        }
                    }

                    // If we aren't on the floor
                    if row > 0{
                        // Try going down
                        if _solve_maze(row - 1, col, maze){
                            return true;
                        }
                    }
                    
                    // If we aren't on the left wall
                    if col > 0{
                        // Try going left
                        if _solve_maze(row, col - 1, maze){
                            return true;
                        }
                    }

                    // If we aren't on the right wall
                    if col < MAZE_WIDTH{
                        // Try going right 
                        if _solve_maze(row, col + 1, maze){
                            return true;
                        }
                    }
                
                    // We hit a dead end going all directions from us, backstep
                    maze.hallways[row][col].in_path = false;
                    return false;
                }
                else {
                    // We hit a wall, get out
                    return false;
                }
            }
        }
        
        // Space is invalid, get out of here.
        return false;
    }

     if _solve_maze(row, col, &mut maze){
        println!("Solved Maze: ");
        println!("{}", maze);
     } else {
        println!("Maze is unsolvable.");
     }
}

fn load_maze(mut maze: Maze, save: MazeTemplate) -> Maze{
    for coords in save.walls{
        if coords.0 < maze.hallways.len().try_into().unwrap(){
            if coords.1 < maze.hallways[coords.0].len(){
                maze.hallways[coords.0][coords.1].passable = false;
            }
        }
    }
    maze.start = save.start;
    maze.hallways[maze.start.0][maze.start.1].is_start = true;
    maze.end = save.end;
    maze.hallways[maze.end.0][maze.end.1].is_end = true;
    return maze;
}

impl Display for Maze{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Top Border
        let mut t_border: String = "".to_string();
        for i in 0..MAZE_HEIGHT + 2{
            if i == 0{ t_border += "┌"}
            else if i == MAZE_HEIGHT + 1 { t_border += "┐"}
            else {t_border += "─"};
        }
        writeln!(f, "{t_border}").expect("How");

        // Maze, including side borders
        for row in self.hallways{
            // Left Border
            write!(f, "│").expect("How");

            // Maze Interior
            for square in row{
                square.fmt(f).expect("Error formatting node");
                //write!(f, "█").expect("Square moment");
            }

            // Right Border
            write!(f, "│").expect("How");
            write!(f, "\n").expect("I have no idea how you messed up printing a newline, but congrats I guess");
        }

        // Bottom Border
        let mut b_border = "".to_string();
        for i in 0..MAZE_HEIGHT + 2{
            if i == 0{ b_border += "└"}
            else if i == MAZE_HEIGHT + 1 {b_border += "┘"}
            else {b_border += "─"};
        }
        write!(f, "{b_border}").expect("How");
        write!(f, "")
    }
}

impl Display for Node{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let symbol: &str;
        if self.is_start{
            symbol = "S"
        } else if self.is_end {
            symbol = "E"
        } else if self.in_path{
            symbol = "*"
        } else if self.passable{
            if self.searched{
                symbol = "x"
            } else {
                symbol = " "
            }
        } else {
            symbol = "█"
        }
        write!(f, "{symbol}")
    }
}
