/*
 * Copyright (C) 2019  CERN for the benefit of the LHCb collaboration
 * Author: Paul Seyfert <pseyfert@cern.ch>
 *
 * This software is distributed under the terms of the GNU General Public
 * Licence version 3 (GPL Version 3), copied verbatim in the file "LICENSE".
 */

fn process(program: &mut [usize]) -> usize {
    let mut program_pointer: usize = 0;
    let mut opcode = program[program_pointer];

    loop {
        match opcode {
            1 => {
                program[program[program_pointer + 3]] =
                    program[program[program_pointer + 1]] + program[program[program_pointer + 2]]
            }
            2 => {
                program[program[program_pointer + 3]] =
                    program[program[program_pointer + 1]] * program[program[program_pointer + 2]]
            }
            99 => break,
            _ => println!("panic"),
        }

        program_pointer += 4;
        opcode = program[program_pointer];
    }

    program[0]
}

fn run(noun: usize, verb: usize) -> usize {
    let mut program = [
        1, 0, 0, 3, 1, 1, 2, 3, 1, 3, 4, 3, 1, 5, 0, 3, 2, 9, 1, 19, 1, 9, 19, 23, 1, 23, 5, 27, 2,
        27, 10, 31, 1, 6, 31, 35, 1, 6, 35, 39, 2, 9, 39, 43, 1, 6, 43, 47, 1, 47, 5, 51, 1, 51,
        13, 55, 1, 55, 13, 59, 1, 59, 5, 63, 2, 63, 6, 67, 1, 5, 67, 71, 1, 71, 13, 75, 1, 10, 75,
        79, 2, 79, 6, 83, 2, 9, 83, 87, 1, 5, 87, 91, 1, 91, 5, 95, 2, 9, 95, 99, 1, 6, 99, 103, 1,
        9, 103, 107, 2, 9, 107, 111, 1, 111, 6, 115, 2, 9, 115, 119, 1, 119, 6, 123, 1, 123, 9,
        127, 2, 127, 13, 131, 1, 131, 9, 135, 1, 10, 135, 139, 2, 139, 10, 143, 1, 143, 5, 147, 2,
        147, 6, 151, 1, 151, 5, 155, 1, 2, 155, 159, 1, 6, 159, 0, 99, 2, 0, 14, 0,
    ];
    program[1] = noun;
    program[2] = verb;

    let retval = process(&mut program);

    return retval;
}

fn main() {
    for noun in 0..100 {
        for verb in 0..100 {
            if run(noun, verb) == 19690720 {
                println!("answer is {}{}", noun, verb);
                break;
            }
        }
    }
}
