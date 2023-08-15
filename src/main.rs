use std::time::Instant;
use std::time::Duration;
use CellState::*;

fn main() {
    
    let initial_state = vec![
        vec![Dead,  Alive, Dead], 
        vec![Dead,  Dead,  Alive], 
        vec![Alive, Alive, Alive], 
        vec![Dead,  Dead,  Dead]
    ];

    let mut durations: Vec<Duration> = Vec::new();
    let duration_iterator = 5000;
        
    for _ in 0..duration_iterator {
        
        let start = Instant::now();
        calc_next_board_state(&initial_state);
        let end = Instant::now();
        let duration = end - start;
        durations.push(duration);
        println!("Time taken1: {:?}", duration);
    }

    let total_duration = durations.iter().fold(Duration::new(0, 0), |acc, &duration| acc + duration);
    let average_duration = total_duration / durations.len() as u32;
    println!("Average Run Duration: {:?}", average_duration);

}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum CellState {
    Dead = 0,
    Alive = 1,
}

fn calc_next_board_state(initial_state: &Vec<Vec<CellState>>) -> Vec<Vec<CellState>> {
    let mut final_state: Vec<Vec<CellState>> = initial_state.clone(); // can make an initial state with just Dead values so only the success state is changed
    
    let rows = initial_state.len();
    let columns = initial_state.get(0).expect("No value").len();
    
    for row in 0..rows {
        for col in 0..columns {
        //collect all the positions of the values that are supposed to be true and change them 
                let mut live_neighbours = 0;
                
                if row+1 < rows && initial_state[row+1][col] == Alive {live_neighbours+=1;}
                if row > 0 && initial_state[row-1][col] == Alive {live_neighbours+=1;}
                if col+1 < columns && initial_state[row][col+1] == Alive {live_neighbours+=1;}
                if col > 0 && initial_state[row][col-1] == Alive {live_neighbours+=1;}
                
                //horizontal
                if row>0 && col>0 && initial_state[row-1][col-1] == Alive {live_neighbours+=1;}
                if row+1 < rows && col+1 < columns && initial_state[row+1][col+1] == Alive {live_neighbours+=1;}
                if row > 0 && col+1 < columns && initial_state[row-1][col+1] == Alive {live_neighbours+=1;}
                if row+1 < rows && col > 0 && initial_state[row+1][col-1] == Alive {live_neighbours+=1;}
                
                
                if (live_neighbours == 2 || live_neighbours == 3) && initial_state[row][col] == Alive {final_state[row][col] = Alive;}
                else if live_neighbours < 2 && initial_state[row][col] == Alive {final_state[row][col] = Dead;}
                else if live_neighbours > 3 && initial_state[row][col] == Alive {final_state[row][col] = Dead;}
                else if live_neighbours == 3 && initial_state[row][col] == Dead {final_state[row][col] = Alive;}
        }
    }
    final_state
}

#[cfg(test)]
mod tests {
use super::calc_next_board_state;
use std::time::Instant;
use std::time::Duration;
use crate::CellState::*;


    #[test]
    fn test_2() {
        let initial_state = vec![
            vec![Dead,  Alive, Dead], 
            vec![Dead,  Dead,  Alive], 
            vec![Alive, Alive, Alive], 
            vec![Dead,  Dead,  Dead]
        ];
    
    
        let final_state = vec![
            vec![Dead,  Dead,  Dead], 
            vec![Alive, Dead,  Alive], 
            vec![Dead,  Alive, Alive], 
            vec![Dead,  Alive, Dead]
        ];

        let calc_next_state = calc_next_board_state(&initial_state);
        assert_eq!(&final_state, &calc_next_state);
    }
}





