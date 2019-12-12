/*
 * Copyright (C) 2019  CERN for the benefit of the LHCb collaboration
 * Author: Paul Seyfert <pseyfert@cern.ch>
 *
 * This software is distributed under the terms of the GNU General Public
 * Licence version 3 (GPL Version 3), copied verbatim in the file "LICENSE".
 */

struct Point {
    x: isize,
    y: isize,
}

struct Line {
    section_lengths: Vec<usize>,
    section_directions: Vec<char>,
}

struct LineWithIt<'a> {
    on_the_section_iter: usize, // how many steps in the current direction have already been made
    which_section_iter: usize, // pointing at the instruction behind the last that has been fully processed
    p: Point,
    line: &'a Line,
    steps_taken: isize,
}

fn step_success(line: &mut LineWithIt) -> bool {
    if line.which_section_iter == (*line.line).section_directions.len() {
        return false;
    }

    let current_dir: char = (*line.line).section_directions[line.which_section_iter];
    match current_dir {
        'R' => {
            line.p.x += 1;
        }
        'L' => {
            line.p.x -= 1;
        }
        'U' => {
            line.p.y += 1;
        }
        'D' => {
            line.p.y -= 1;
        }
        _ => println!("panic"),
    }
    line.steps_taken += 1;
    line.on_the_section_iter += 1;

    // the last step finished the segment
    if line.on_the_section_iter == (*line.line).section_lengths[line.which_section_iter] {
        line.on_the_section_iter = 0;
        line.which_section_iter += 1;
    }
    return true;
}

fn reset_line(line: &mut LineWithIt) {
    line.on_the_section_iter = 0;
    line.which_section_iter = 0;
    line.p.x = 0;
    line.p.y = 0;
    line.steps_taken = 0;
}

fn create_iterable_line<'a>(line: &'a Line) -> LineWithIt {
    LineWithIt {
        on_the_section_iter: 0,
        which_section_iter: 0,
        p: Point { x: 0, y: 0 },
        line: line,
        steps_taken: 0,
    }
}

fn check_intersect(x: isize, y: isize, iterable_line: &mut LineWithIt, abort: isize) -> bool {
    while step_success(iterable_line) {
        // not strictly checking an intersection
        if iterable_line.steps_taken > abort {
            return false;
        }
        if iterable_line.p.x == x && iterable_line.p.y == y {
            return true;
        }
    }
    return false;
}

fn meet(line1: &Line, line2: &Line) -> isize {
    let mut iterable_line1 = create_iterable_line(line1);
    let mut best: isize = 200000; // guess
                                  // let mut points: Vec<Point> = Vec::new();

    while step_success(&mut iterable_line1) {
        if iterable_line1.steps_taken > best {
            break;
        }
        if iterable_line1.steps_taken + manhattan(&iterable_line1.p) > best {
            // no need to check, too far out
            continue;
        }
        let mut iterable_line2 = create_iterable_line(line2);
        if check_intersect(
            iterable_line1.p.x,
            iterable_line1.p.y,
            &mut iterable_line2,
            best - iterable_line1.steps_taken,
        ) {
            let signal = iterable_line1.steps_taken + iterable_line2.steps_taken;
            if signal < best {
                best = signal;
            }
        }
    }
    return best;
}

fn abs(input: isize) -> isize {
    if input > 0 {
        return input;
    } else {
        return -input;
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
    let line1 = parse("R990,U944,L921,U993,L64,U29,R899,D406,R841,U716,L32,U658,L830,D481,L441,U491,L687,D847,L459,U920,R165,U224,L896,D868,L191,U877,L909,U467,R798,U132,R234,U49,R484,U911,R108,D308,R867,U350,L404,U107,L84,U668,R850,U470,L182,U93,R284,U999,L664,U110,R650,D189,R540,D112,R794,U999,R871,D829,L549,U988,R654,D411,R323,D774,R529,U275,L909,U936,R122,D922,L331,U813,L748,U770,R511,D892,L770,U318,R661,U823,R210,D393,L694,U929,L76,D619,R395,U651,R526,U145,R851,U112,R73,D89,R17,U929,R807,U87,R764,D158,L820,U803,L785,D205,L828,D271,L839,D482,L797,U338,R322,D633,L292,U16,R627,U19,R548,U516,L384,U83,R256,U937,R404,U322,R671,D543,L412,U446,R238,U246,L794,D750,L34,U317,L994,U874,L247,D20,R491,U834,L498,D987,R2,U175,R452,U168,R495,D183,R201,U532,L192,U984,L566,D471,L704,D2,L776,U5,R911,U308,R763,D555,R458,D439,L968,D78,R549,D583,R289,D355,L503,D871,L881,U516,L507,D551,R711,U702,L308,D905,L408,U932,L884,U218,L158,D562,L200,D114,R673,U448,R887,U181,R247,U329,L965,U495,L329,D162,L265,D389,R419,U435,R258,U146,R208,D184,R730,D19,L78,D886,R472,D350,R484,U392,L542,U601,L202,U974,L310,U52,L537,U597,L163,D655,R928,U269,L926,D790,L513,U441,L90,U581,L211,U871,R603,D130,L220,U459,L933,U648,R721,U642,R301,U537,L858,D777,R823,U14,R820,D218,L96,D318,L206,U280,R887,D241,L752,U828,R354,U864,R844,D872,L728,D298,L234,U695,R434,D94,R905,D592,L32,D860,R680,D197,R886,U760,L232,U916,L452,U248,R715,D773,R867,U77,R751,D36,R565,U897,R782,U931,R546,U261,R920,D296,R451,U258,L394,U965,R912,D593,L990");
    let line2 = parse("L994,U515,R163,U863,L343,U162,L875,D92,L483,D601,R79,D761,L389,U167,L145,U145,L247,U886,R61,U820,L584,D239,R402,U805,R956,U126,R615,D322,R431,D460,R397,D511,R805,D177,L778,U296,R599,U759,R40,U1,L422,U751,R94,U401,R504,U940,L564,U24,R595,U944,R815,U672,R787,D854,R579,D604,L62,U670,L516,U199,L639,D919,L485,U655,R391,U669,R772,D34,R868,D12,L108,U295,R701,D603,R493,U927,R29,D34,R499,U111,L87,U190,R884,D658,R474,D166,R921,U698,R592,U25,R710,D398,L26,U696,L432,D887,R469,U656,L428,D188,L543,D150,R160,U543,R743,U692,R618,D148,R956,U753,L175,D789,R897,U305,L137,D914,R330,D780,R744,D473,L754,U482,L975,D413,L698,U656,L177,U419,R13,D827,L67,D800,R369,U97,L34,D588,L41,D760,L164,U224,L921,D311,R489,U956,R277,U180,R724,U748,R785,U826,L426,D957,R303,U16,L729,U224,L712,U43,L280,D648,R987,D941,R154,D581,R876,U615,L480,D103,R636,D276,R948,U89,R434,D212,R837,D295,R532,D390,R374,D926,R911,D110,R258,U83,L955,U747,L925,D366,R571,U241,R628,D344,R919,U117,R337,D683,L720,U261,L124,D545,R979,D601,L906,D324,R441,U678,L978,U744,L472,D217,R799,U740,L77,U964,L278,U497,R441,U21,L37,U319,L24,D211,L44,U459,R35,D609,R900,D538,R397,D776,R629,D860,R519,D340,R168,U603,R46,U889,R897,D442,R997,U705,L82,D963,R941,U701,L347,D824,R269,U891,L569,D558,L867,U145,R121,D369,R542,U227,L198,U863,L755,U273,L734,D233,R578,U67,L821,U600,L203,D234,R695,U819,L639,D700,R295,D129,L612,U157,R212,U536,L968,U562,L999,D391,L231,U262,R334,D967,R463,U748,R842,D500,R860,U856,R263,D633,R460,D337,L880,U146,R910");
    // let line1 = parse("R75,D30,R83,U83,L12,D49,R71,U7,L72");
    // let line2 = parse("U62,R66,U55,R34,D71,R55,D58,R83");

    let minimal_distance = meet(&line1, &line2);

    println!("closest match is at {}", minimal_distance);
}

fn parse(definition: &str) -> Line {
    let split = definition.split(",");
    let vec: Vec<&str> = split.collect();

    let mut line = Line {
        section_lengths: vec![],
        section_directions: vec![],
    };
    for instruction in vec {
        let dir = (*instruction).chars().nth(0).unwrap();
        line.section_directions.push(dir);
        let length = &(*instruction)[1..];
        let length = length.parse::<usize>().unwrap();
        line.section_lengths.push(length);
    }

    line
}

// impl FromStr for Line {
// }
