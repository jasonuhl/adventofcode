#[derive(Eq)]
#[derive(PartialEq)]
#[derive(Hash)]
#[derive(Debug)]
#[derive(Clone)]
struct Block {
    y: usize,
    x: usize,
}

#[derive(Eq)]
#[derive(PartialEq)]
#[derive(Hash)]
#[derive(Debug)]
#[derive(Clone)]
enum Direction {
    D = 0,
    U = 1,
    R = 2,
    L = 3,
}

#[derive(Eq, Hash, PartialEq)]
#[derive(Debug)]
#[derive(Clone)]
struct State {
    block: Block,
    direction: Direction,
    prior: Block,
    prior_state_dir: Direction,
    prior_state_dist: u8,
    straight_steps: u8,
}

fn get_direction(block: &Block, prior: &Block) -> Direction {
    if block.y > prior.y {
        return Direction::D;
    } else if block.y < prior.y {
        return Direction::U;
    } else if block.x > prior.x {
        return Direction::R;
    } else if block.x < prior.x {
        return Direction::L;
    } else {
        panic!("no direction");
    }
}

fn reverse_step(state: &State, visited: &Vec<Vec<Vec<Vec<Option<State>>>>>) -> State {
    let newstate = visited[state.prior.y][state.prior.x][state.prior_state_dir.clone() as usize][state.prior_state_dist as usize].clone().unwrap();
    //println!("reverse_step: {:?}", newstate.block);
    println!("reverse_step: {:?}", newstate);

    return newstate;
}

fn reverse_walk(state: &State, visited: &Vec<Vec<Vec<Vec<Option<State>>>>>) {
    let mut newstate = state.clone();
    loop {
        if newstate.block.y == 0 && newstate.block.x == 0 {
            return;
        }
        newstate = reverse_step(&newstate, visited);
    }
}

fn main() {
    use std::fs::File;
    use std::io::{prelude::*, BufReader};
    use priority_queue::PriorityQueue;

    let f = File::open("input.txt").unwrap();
    let reader = BufReader::new(f);

    let mut a: Vec<Vec<u8>> = vec![];
    let mut pq = PriorityQueue::new();

    for line in reader.lines() {
        let s = line.unwrap();
        let costs: Vec<u8> = s.bytes().map(|b| b - 48).collect();
        a.push(costs);
    }

    let mut visited: Vec<Vec<Vec<Vec<Option<State>>>>> = vec![vec![vec![vec![None; 11]; 4]; a[0].len()]; a.len()];

    let mut current = State{
        block: Block{y: 0, x: 0},
        direction: Direction::D,
        prior: Block{y: 0, x: 0},
        prior_state_dir: Direction::D,
        prior_state_dist: 0,
        straight_steps: 0,
    };
    let mut current_cum_cost = 0;

    visited[current.block.y][current.block.x][current.direction.clone() as usize][current.straight_steps as usize] = Some(current.clone());
    let mut total_pq_pushes = 0;
    loop {
        if current.block.y == a.len() - 1 && current.block.x == a[0].len() - 1 {
            println!("Done: {}", current_cum_cost);
            println!("Current: {:?}", current);
            println!("Final size of pq={}", pq.len());
            println!("Total pq pushes is {}", total_pq_pushes);
            reverse_walk(&current, &visited);
            return;
        }

        let mut neighbors: Vec<Block> = vec![];

        let mut gobble: usize;

        if current.block.y == 0 && current.block.x == 0 {
            gobble = 4;
        } else if current.straight_steps >= 4 && current.block.x == current.prior.x {
            gobble = 1;
        } else {
            gobble = 4;
        }
        if ((current.block.y == 0 && current.block.x == 0) || current.direction != Direction::U) && current.block.y + gobble < a.len() {
            let candidate = Block{y: current.block.y + gobble, x: current.block.x};
            if candidate != current.prior {
                neighbors.push(candidate);
            }
        }

        if current.block.y == 0 && current.block.x == 0 {
            gobble = 4;
        } else if current.straight_steps >= 4 && current.block.y == current.prior.y {
            gobble = 1;
        } else {
            gobble = 4;
        }
        if ((current.block.y == 0 && current.block.x == 0) || current.direction != Direction::L) && current.block.x + gobble < a[0].len() {
            let candidate = Block{y: current.block.y, x: current.block.x + gobble};
            if candidate != current.prior {
                neighbors.push(candidate);
            }
        }

        if current.block.y == 0 && current.block.x == 0 {
            gobble = 4;
        } else if current.straight_steps >= 4 && current.block.x == current.prior.x {
            gobble = 1;
        } else {
            gobble = 4;
        }
        if ((current.block.y == 0 && current.block.x == 0) || current.direction != Direction::D) && current.block.y >= 4 {
            let candidate = Block{y: current.block.y - gobble, x: current.block.x};
            if candidate != current.prior {
                neighbors.push(candidate);
            }
        }

        if current.block.y == 0 && current.block.x == 0 {
            gobble = 4;
        } else if current.straight_steps >= 4 && current.block.y == current.prior.y {
            gobble = 1;
        } else {
            gobble = 4;
        }
        if ((current.block.y == 0 && current.block.x == 0) || current.direction != Direction::R) && current.block.x >= 4 {
            let candidate = Block{y: current.block.y, x: current.block.x - gobble};
            if candidate != current.prior {
                neighbors.push(candidate);
            }
        }

        for neigh in neighbors {
            //println!("At current {:?} looking at neighbor {:?}", current, neigh);
            //let cell_cost: u32 = u32::try_from(a[neigh.y][neigh.x]).unwrap();
            let mut straight_steps = 4;
            if current.block.x == 0 && current.block.y == 0 {
                straight_steps = 4;
            } else if (neigh.x == current.block.x && current.block.x == current.prior.x) || (neigh.y == current.block.y && current.block.y == current.prior.y) {
                straight_steps = current.straight_steps + 1;
            }

            if straight_steps > 10 {
                continue;
            }

            let mut cum_cost = current_cum_cost;
            if neigh.y == current.block.y {
                if neigh.x > current.block.x {
                    //if neigh.x - current.block.x != straight_steps as usize {
                    if false {
                        panic!("wrong distance: {} vs. {} for neigh {:?} and current {:?}", neigh.y - current.block.y, straight_steps, neigh, current);
                    } else {
                        for x in current.block.x+1..=neigh.x {
                            let cell_cost: u32 = u32::try_from(a[neigh.y][x]).unwrap();
                            cum_cost += cell_cost;
                        }
                    }
                } else {
                    //if current.block.x - neigh.x != straight_steps as usize {
                    if false {
                        panic!("wrong distance: {} vs. {} for neigh {:?} and current {:?}", neigh.y - current.block.y, straight_steps, neigh, current);
                    } else {
                        for x in neigh.x..current.block.x {
                            let cell_cost: u32 = u32::try_from(a[neigh.y][x]).unwrap();
                            cum_cost += cell_cost;
                        }
                    }
                }
            } else if neigh.x == current.block.x {
                if neigh.y > current.block.y {
//                    if neigh.y - current.block.y != straight_steps as usize {
                    if false {
                        panic!("wrong distance: {} vs. {} for neigh {:?} and current {:?}", neigh.y - current.block.y, straight_steps, neigh, current);
                    } else {
                        for y in current.block.y+1..=neigh.y {
                            let cell_cost: u32 = u32::try_from(a[y][neigh.x]).unwrap();
                            cum_cost += cell_cost;
                        }
                    }
                } else {
                    //if current.block.y - neigh.y != straight_steps as usize {
                    if false {
                        panic!("wrong distance: {} vs. {} for neigh {:?} and current {:?}", neigh.y - current.block.y, straight_steps, neigh, current);
                    } else {
                        for y in neigh.y..current.block.y {
                            let cell_cost: u32 = u32::try_from(a[y][neigh.x]).unwrap();
                            cum_cost += cell_cost;
                        }
                    }
                }
            } else {
                panic!("not colinear");
            }

            let dir = get_direction(&neigh, &current.block);
            let neigh_state = State{
                block: neigh.clone(),
                direction: dir.clone(),
                prior: current.block.clone(),
                prior_state_dir: current.direction.clone(),
                prior_state_dist: current.straight_steps,
                straight_steps: straight_steps,
            };
            //println!("pq len is {}, New neighbor: {:?}", pq.len(), neigh_state);
            if pq.len() > 1000000 {
                println!("neigh_state {:?}", neigh_state);
                panic!("too big");
            }

            if visited[neigh_state.block.y][neigh_state.block.x][dir as usize][neigh_state.straight_steps as usize].is_some() {
                //println!("Already visited: {:?}", neigh_state);
                continue;
            }
            let old_neigh_state = pq.get(&neigh_state);
            if old_neigh_state.is_some() {
                let (_, priority) = old_neigh_state.unwrap();
                let old_cum_cost = u32::MAX - priority;
                if cum_cost < old_cum_cost {
                    println!("Found a lower cost for {:?}: {} < {}", neigh_state, cum_cost, old_cum_cost);
                    pq.push(neigh_state.clone(), u32::MAX - cum_cost);
                    total_pq_pushes += 1;
                }
            } else {
                pq.push(neigh_state.clone(), u32::MAX - cum_cost);
                total_pq_pushes += 1;
            }
        }

        let prio: u32;
        (current, prio) = pq.pop().unwrap();
        visited[current.block.y][current.block.x][current.direction.clone() as usize][current.straight_steps as usize] = Some(current.clone());
        current_cum_cost = u32::MAX - prio;
    }
}
