use std::fs;

struct Vertex {
    x: i32,
    y: i32,
    value: char,
    distance: i32,
}

struct Edge {
    start: Vertex,
    end: Vertex,
}

struct Graph {
    vertices: Vec<Vertex>,
    edges: Vec<Edge>,
}

// impl PartialEq for Coords {
//     fn eq(&self, other: &Self) -> bool {
//         self.x == other.x && self.y == other.y
//     }
// }

fn parse_char(c: char) -> i32 {
    return c as i32;
}

fn get_char(c: char) -> char {
    if c == 'S' {
        return 'a';
    }
    if c == 'E' {
        return 'z';
    }
    return c;
}

fn is_within_one_letter(current: char, next: char) -> bool {
    if current == 'z' && next == 'E' {
        return true;
    }
    if current == 'S' && next == 'a' {
        return true;
    }
    let c = parse_char(get_char(current));
    let n = parse_char(get_char(next));
    return c - n >= -1 && c - n <= 1;
}

fn part1() {
    println!("Day 12 Part 1");
    let file_path = "./src/input.txt";
    println!("In file {}", file_path);

    let contents: String =
        fs::read_to_string(file_path).expect("Should have been able to read the file");
    let lines: Vec<&str> = contents.split("\n").filter(|s| !s.is_empty()).collect();

    let mut graph = Graph {
        vertices: Vec::new(),
        edges: Vec::new(),
    };

    let mut map: Vec<Vec<char>> = Vec::new();

    for y in 0..lines.len() {
        map.push(Vec::new());
        for c in lines[y].chars() {
            map[y].push(c);
        }
    }

    // Build Vertices and edges
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            graph.vertices.push(Vertex {
                x: x as i32,
                y: x as i32,
                value: map[y][x],
                distance: 100,
            });
            // // Down
            // if y > 0 && is_within_one_letter(map[y][x], map[y - 1][x]) {
            //     graph.edges.push(Edge {
            //         sx: x as i32,
            //         sy: y as i32,
            //         ex: x as i32,
            //         ey: (y as i32) - 1,
            //         weight: 1,
            //     });
            // }
            // // Up
            // if y < map.len() - 1 && is_within_one_letter(map[y][x], map[y + 1][x]) {
            //     graph.edges.push(Edge {
            //         sx: x as i32,
            //         sy: y as i32,
            //         ex: x as i32,
            //         ey: (y as i32) + 1,
            //         weight: 1,
            //     });
            // }
            // // Right
            // if x < map[y].len() - 1 && is_within_one_letter(map[y][x], map[y][x + 1]) {
            //     graph.edges.push(Edge {
            //         sx: x as i32,
            //         sy: y as i32,
            //         ex: (x as i32) + 1,
            //         ey: y as i32,
            //         weight: 1,
            //     });
            // }
            // // Left
            // if x > 0 && is_within_one_letter(map[y][x], map[y][x - 1]) {
            //     graph.edges.push(Edge {
            //         sx: x as i32,
            //         sy: y as i32,
            //         ex: (x as i32) - 1,
            //         ey: y as i32,
            //         weight: 1,
            //     });
            // }
        }
    }

    for i in 0..graph.vertices.len() {
        for j in i..graph.vertices.len() {
            // Down
            if graph.vertices[i] && is_within_one_letter(graph.vertices[i].symbol, map[y - 1][x]) {
                graph.edges.push(Edge {
                    sx: x as i32,
                    sy: y as i32,
                    ex: x as i32,
                    ey: (y as i32) - 1,
                    weight: 1,
                });
            }
            // Up
            if y < map.len() - 1 && is_within_one_letter(map[y][x], map[y + 1][x]) {
                graph.edges.push(Edge {
                    sx: x as i32,
                    sy: y as i32,
                    ex: x as i32,
                    ey: (y as i32) + 1,
                    weight: 1,
                });
            }
            // Right
            if x < map[y].len() - 1 && is_within_one_letter(map[y][x], map[y][x + 1]) {
                graph.edges.push(Edge {
                    sx: x as i32,
                    sy: y as i32,
                    ex: (x as i32) + 1,
                    ey: y as i32,
                    weight: 1,
                });
            }
            // Left
            if x > 0 && is_within_one_letter(map[y][x], map[y][x - 1]) {
                graph.edges.push(Edge {
                    sx: x as i32,
                    sy: y as i32,
                    ex: (x as i32) - 1,
                    ey: y as i32,
                    weight: 1,
                });
            }
        }
    }

    for y in 0..map.len() {
        for x in 0..map[y].len() {
            print!("{}", map[y][x]);
        }
        print!("\n",);
    }
}

fn main() {
    part1();
}
