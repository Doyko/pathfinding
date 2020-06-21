use std::collections::VecDeque;
use std::env;
use std::fs::File;
use std::io::Read;

use termion::color;

#[derive(Debug, Copy, Clone)]
struct Pos {
    x: usize,
    y: usize,
}

impl PartialEq for Pos {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}
#[derive(Debug, Copy, Clone, PartialEq)]
enum Elem {
    Uncheck(u32),
    Check(u32),
    Start,
    End,
    Wall,
}

#[derive(Debug, Clone)]
struct Map {
    width: usize,
    height: usize,
    start: Pos,
    end: Pos,
    map: Vec<Elem>,
}

fn draw_map(map: &Map) {
    for i in 0..map.height {
        for j in 0..map.width {
            match map.map.get(i * map.width + j).unwrap() {
                Elem::Uncheck(0) => print!(
                    "{}0  {}",
                    color::Fg(color::LightBlack),
                    color::Fg(color::Reset)
                ),
                Elem::Uncheck(n) => print!(
                    "{}{:3}{}",
                    color::Fg(color::Green),
                    n.to_string(),
                    color::Fg(color::Reset)
                ),
                Elem::Check(n) => print!(
                    "{}{:3}{}",
                    color::Fg(color::LightGreen),
                    n.to_string(),
                    color::Fg(color::Reset)
                ),
                Elem::Start => print!(
                    "{}S  {}",
                    color::Fg(color::LightYellow),
                    color::Fg(color::Reset)
                ),
                Elem::End => print!("{}E  {}", color::Fg(color::Cyan), color::Fg(color::Reset)),
                Elem::Wall => print!(
                    "{}X  {}",
                    color::Fg(color::LightRed),
                    color::Fg(color::Reset)
                ),
            }
        }
        print!("\n");
    }
}

fn dist(pos1: Pos, pos2: Pos) -> u32 {
    ((pos1.x as i32 - pos2.x as i32).abs() + (pos1.y as i32 - pos2.y as i32).abs()) as u32
}

fn read_map(file_name: String) -> Map {
    let mut file = File::open(file_name).unwrap();
    let mut file_str = String::new();
    file.read_to_string(&mut file_str).unwrap();

    let mut lines: Vec<&str> = file_str.lines().collect();

    let mut it_size = lines
        .remove(0)
        .split_whitespace()
        .map(|c| c.parse::<usize>().unwrap());
    let (width, height) = (it_size.next().unwrap(), it_size.next().unwrap());

    let mut it_target = lines
        .remove(0)
        .split_whitespace()
        .map(|c| c.parse::<usize>().unwrap());
    let start = Pos {
        x: it_target.next().unwrap(),
        y: it_target.next().unwrap(),
    };
    let end = Pos {
        x: it_target.next().unwrap(),
        y: it_target.next().unwrap(),
    };

    let mut map: Vec<Elem> = lines
        .iter()
        .map(|line| {
            line.split_whitespace().map(|c| {
                if c.chars().next().unwrap() == '0' {
                    Elem::Uncheck(0)
                } else {
                    Elem::Wall
                }
            })
        })
        .flatten()
        .collect();

    assert!(
        start.x < width && start.y < height,
        "error with the start !"
    );
    assert!(end.x < width && end.y < height, "error with the end !");
    assert!(map.len() == height * width, "error with the map size !");

    map[start.y * width + start.x] = Elem::Start;
    map[end.y * width + end.x] = Elem::End;

    Map {
        width,
        height,
        start,
        end,
        map,
    }
}

const CARDINALS: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

fn get_neighbors(pos: &Pos, map: &Map) -> Vec<Pos> {
    CARDINALS
        .iter()
        .filter(|(x, y)| {
            (x + pos.x as i32) >= 0
                && (x + pos.x as i32) < map.width as i32
                && (y + pos.y as i32) >= 0
                && (y + pos.y as i32) < map.height as i32
        })
        .map(|(x, y)| Pos {
            x: (pos.x as i32 + x) as usize,
            y: (pos.y as i32 + y) as usize,
        })
        .filter(|pos| {
            match map
                .map
                .get(pos.y * map.width + pos.x)
                .unwrap_or(&Elem::Wall)
            {
                Elem::Uncheck(_n) => true,
                Elem::End => true,
                _ => false,
            }
        })
        .collect()
}

fn breath(map: &mut Map) -> (u32, u32) {
    let mut step = 0;
    let mut len = 0;
    let mut queue: VecDeque<Pos> = VecDeque::new();

    queue.push_back(map.start);

    while !queue.is_empty() {
        step += 1;
        let current = queue.pop_front().unwrap();

        if current == map.end {
            match map.map.get(current.x + map.width * current.y).unwrap() {
                Elem::Uncheck(n) => len = *n,
                _ => (),
            };
            map.map[current.x + map.width * current.y] = Elem::End;
            break;
        }

        match map.map.get(current.x + map.width * current.y).unwrap() {
            Elem::Uncheck(n) => map.map[current.x + map.width * current.y] = Elem::Check(*n),
            _ => (),
        };

        for n in get_neighbors(&current, map) {
            let elem = map.map.get(n.x + map.width * n.y).unwrap();

            if *elem != Elem::Uncheck(0) && *elem != Elem::End {
                continue;
            }

            map.map[n.x + map.width * n.y] = Elem::Uncheck(
                match map.map.get(current.x + map.width * current.y).unwrap() {
                    Elem::Check(n) => n + 1,
                    _ => 1,
                },
            );
            queue.push_back(n);
        }
    }

    println!("+----------------------+");
    println!("| Breadth first search |");
    println!("+----------------------+");

    draw_map(&map);

    println!("\nstep : {}\nlength : {}\n", step, len);

    (step, len)
}

fn heuristic(map: &mut Map) -> (u32, u32) {
    let mut step = 0;
    let mut len = 0;
    let mut vec: Vec<Pos> = Vec::new();

    vec.push(map.start);

    while !vec.is_empty() {
        step += 1;
        vec.sort_by(|a, b| dist(*b, map.end).partial_cmp(&dist(*a, map.end)).unwrap());
        let current = vec.pop().unwrap();

        if current == map.end {
            match map.map.get(current.x + map.width * current.y).unwrap() {
                Elem::Uncheck(n) => len = *n,
                _ => (),
            };
            map.map[current.x + map.width * current.y] = Elem::End;
            break;
        }

        match map.map.get(current.x + map.width * current.y).unwrap() {
            Elem::Uncheck(n) => map.map[current.x + map.width * current.y] = Elem::Check(*n),
            _ => (),
        };

        for n in get_neighbors(&current, map) {
            let elem = map.map.get(n.x + map.width * n.y).unwrap();

            if *elem != Elem::Uncheck(0) && *elem != Elem::End {
                continue;
            }

            map.map[n.x + map.width * n.y] = Elem::Uncheck(
                match map.map.get(current.x + map.width * current.y).unwrap() {
                    Elem::Check(n) => n + 1,
                    _ => 1,
                },
            );
            vec.push(n);
        }
    }
    println!("+------------------+");
    println!("| Heuristic search |");
    println!("+------------------+");

    draw_map(&map);

    println!("\nstep : {}\nlength : {}\n", step, len);

    (step, len)
}

fn astar(map: &mut Map) -> (u32, u32) {
    let mut step = 0;
    let mut len = 0;
    let mut vec: Vec<Pos> = Vec::new();

    vec.push(map.start);

    while !vec.is_empty() {
        step += 1;
        vec.sort_by(|a, b| {
            (match map.map.get(b.x + map.width * b.y).unwrap() {
                Elem::Uncheck(n) => *n,
                _ => 0,
            } + dist(*b, map.end))
            .partial_cmp(
                &(match map.map.get(a.x + map.width * a.y).unwrap() {
                    Elem::Uncheck(n) => *n,
                    _ => 0,
                } + &dist(*a, map.end)),
            )
            .unwrap()
        });
        let current = vec.pop().unwrap();

        if current == map.end {
            match map.map.get(current.x + map.width * current.y).unwrap() {
                Elem::Uncheck(n) => len = *n,
                _ => (),
            };
            map.map[current.x + map.width * current.y] = Elem::End;
            break;
        }

        match map.map.get(current.x + map.width * current.y).unwrap() {
            Elem::Uncheck(n) => map.map[current.x + map.width * current.y] = Elem::Check(*n),
            _ => (),
        };

        for n in get_neighbors(&current, map) {
            let elem = map.map.get(n.x + map.width * n.y).unwrap();

            if match &elem {
                Elem::Uncheck(0) => false,
                Elem::Uncheck(cn) => {
                    match map.map.get(current.x + map.width * current.y).unwrap() {
                        Elem::Check(cc) => *cn < cc + 1,
                        _ => true,
                    }
                }
                Elem::End => false,
                _ => true,
            } {
                continue;
            }

            map.map[n.x + map.width * n.y] = Elem::Uncheck(
                match map.map.get(current.x + map.width * current.y).unwrap() {
                    Elem::Check(n) => n + 1,
                    _ => 1,
                },
            );
            vec.retain(|&e| e != n);
            vec.push(n);
        }
    }
    println!("+--------+");
    println!("| A-star |");
    println!("+--------+");

    draw_map(&map);

    println!("\nstep : {}\nlength : {}\n", step, len);

    (step, len)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    assert!(args.len() > 1, "Need the name of the map as an argument!");

    let map = read_map(format!("map/{}", &args[1]));

    println!("+-----+");
    println!("| MAP |");
    println!("+-----+");

    draw_map(&map);

    let b = breath(&mut map.clone());
    let h = heuristic(&mut map.clone());
    let a = astar(&mut map.clone());

    println!("+--------+");
    println!("| RESULT |");
    println!("+--------+\n");

    println!("+----------------------+--------+--------+");
    println!("| Algorithm            | steps  | length |");
    println!("+----------------------+--------+--------+");
    println!("| Breadth first search | {:6} | {:6} |", b.0, b.1);
    println!("+----------------------+--------+--------+");
    println!("| Heuristic search     | {:6} | {:6} |", h.0, h.1);
    println!("+----------------------+--------+--------+");
    println!("| A-star               | {:6} | {:6} |", a.0, a.1);
    println!("+----------------------+--------+--------+");
}
