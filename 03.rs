/*
 * Copyright (C) 2019  CERN for the benefit of the LHCb collaboration
 * Author: Paul Seyfert <pseyfert@cern.ch>
 *
 * This software is distributed under the terms of the GNU General Public
 * Licence version 3 (GPL Version 3), copied verbatim in the file "LICENSE".
 */

struct Point {
    x: isize,
    y: isize
}

struct LineWithIt {
    on_the_section_iter: usize, // how many steps in the current direction have already been made
    which_section_iter: usize, // pointing at the instruction behind the last that has been fully processed
    x: isize,
    y: isize,
    section_lengths: Vec<usize>,
    section_directions: Vec<char>
}

fn step_success(line: &mut LineWithIt) -> bool {
    if line.which_section_iter == line.section_directions.len() {
        return false;
    }

    let current_dir: char = line.section_directions[line.which_section_iter];
    match current_dir {
        'R' => {
            line.x += 1;
        },
        'L' => {
            line.x -= 1;
        },
        'U' => {
            line.y += 1;
        },
        'D' => {
            line.y -= 1;
        },
        _ => println!("panic"),
    }
    line.on_the_section_iter += 1;

    // the last step finished the segment
    if line.on_the_section_iter == line.section_lengths[line.which_section_iter] {
        line.on_the_section_iter = 0;
        line.which_section_iter += 1;
    }
    return true;
}

fn reset_line(line: &mut LineWithIt) {
    line.on_the_section_iter = 0;
    line.which_section_iter = 0;
    line.x = 0;
    line.y = 0;
}

fn check_intersect(x: isize, y: isize, line: &mut LineWithIt) -> bool {
    reset_line(line);
    while step_success(line) {
        if line.x == x && line.y == y {
            return true
        }
    }
    return false;
}

fn meet(line1: &mut LineWithIt, line2: &mut LineWithIt) -> Vec<Point> {
    let mut points: Vec<Point> = Vec::new();

    while step_success(line1) {
        if check_intersect(line1.x, line1.y, line2) {
            points.push(Point{x: line1.x, y: line1.y});
        }
    }

    points
}

fn abs(input: isize) -> isize {
    if input > 0 {
        return input;
    } else {
        return - input;
    }
}

fn manhattan(point: &Point) -> isize {
    abs(point.x) + abs(point.y)
}

fn min_manhattan(points: &Vec<Point>) -> isize {
    let mut min = manhattan(&points[0]);
    for point in points {
        let cur = manhattan(point);
        if cur < min {
            min = cur;
        }
    }
    min
}

fn main() {
    let mut line1 = LineWithIt{
        section_lengths: vec![8,5,5,3],
        section_directions: vec!['R','U','L','D'],
        x: 0,
        y: 0,
        on_the_section_iter: 0,
        which_section_iter: 0
    };
    let mut line2 = LineWithIt{
        section_lengths: vec![7,6,4,4],
        section_directions: vec!['U','R','D','L'],
        x: 0,
        y: 0,
        on_the_section_iter: 0,
        which_section_iter: 0
    };

    let ps = meet(&mut line1, &mut line2);
    // let matches = check_intersect(4, -1, &mut line);

    for point in ps.iter() {
        println!("meeting in x {}, y {} at distance {}", (*point).x, (*point).y, manhattan(point));
    }
    println!("closest match is at {}", min_manhattan(&ps));
}
