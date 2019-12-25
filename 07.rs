/*
 * Copyright (C) 2019  CERN for the benefit of the LHCb collaboration
 * Author: Paul Seyfert <pseyfert@cern.ch>
 *
 * This software is distributed under the terms of the GNU General Public
 * Licence version 3 (GPL Version 3), copied verbatim in the file "LICENSE".
 */

use permutator::HeapPermutationIterator;

fn process(program: &mut [isize], inputs: &[isize]) -> Vec<isize> {
    let mut retvals = vec![];
    let mut program_pointer: usize = 0;
    let mut input_iterator = 0;

    fn immediate<'a>(
        program: &'a mut [isize],
        program_pointer: usize,
        offset: usize,
    ) -> &mut isize {
        &mut program[program_pointer + offset]
    }
    fn position<'a>(program: &'a mut [isize], program_pointer: usize, offset: usize) -> &mut isize {
        let addr: usize = program[program_pointer + offset] as usize;
        &mut program[addr]
    }

    loop {
        let opcode = program[program_pointer] % 100;

        let first_arg: fn(program: &mut [isize], program_pointer: usize) -> &mut isize;
        let second_arg: fn(program: &mut [isize], program_pointer: usize) -> &mut isize;
        let third_arg: fn(program: &mut [isize], program_pointer: usize) -> &mut isize;

        if (program[program_pointer] / 100) % 10 == 0 {
            fn retval<'a>(program: &'a mut [isize], program_pointer: usize) -> &mut isize {
                return position(program, program_pointer, 1);
            }
            first_arg = retval
        } else {
            fn retval<'a>(program: &'a mut [isize], program_pointer: usize) -> &mut isize {
                return immediate(program, program_pointer, 1);
            }
            first_arg = retval
        };
        if (program[program_pointer] / 1000) % 10 == 0 {
            fn retval<'a>(program: &'a mut [isize], program_pointer: usize) -> &mut isize {
                return position(program, program_pointer, 2);
            }
            second_arg = retval
        } else {
            fn retval<'a>(program: &'a mut [isize], program_pointer: usize) -> &mut isize {
                return immediate(program, program_pointer, 2);
            }
            second_arg = retval
        };
        if (program[program_pointer] / 10000) % 10 == 0 {
            fn retval<'a>(program: &'a mut [isize], program_pointer: usize) -> &mut isize {
                return position(program, program_pointer, 3);
            }
            third_arg = retval
        } else {
            fn retval<'a>(program: &'a mut [isize], program_pointer: usize) -> &mut isize {
                return immediate(program, program_pointer, 3);
            }
            third_arg = retval
        };

        match opcode {
            1 => {
                *third_arg(program, program_pointer) =
                    *first_arg(program, program_pointer) + *second_arg(program, program_pointer);
                program_pointer += 4;
            }
            2 => {
                *third_arg(program, program_pointer) =
                    *first_arg(program, program_pointer) * *second_arg(program, program_pointer);
                program_pointer += 4;
            }
            3 => {
                *first_arg(program, program_pointer) = inputs[input_iterator];
                input_iterator += 1;
                program_pointer += 2;
            }
            4 => {
                retvals.push(*first_arg(program, program_pointer));
                program_pointer += 2;
            }
            5 => {
                if *first_arg(program, program_pointer) != 0 {
                    program_pointer = *second_arg(program, program_pointer) as usize;
                } else {
                    program_pointer += 3;
                }
            }
            6 => {
                if *first_arg(program, program_pointer) == 0 {
                    program_pointer = *second_arg(program, program_pointer) as usize;
                } else {
                    program_pointer += 3;
                }
            }
            7 => {
                if *first_arg(program, program_pointer) < *second_arg(program, program_pointer) {
                    *third_arg(program, program_pointer) = 1;
                } else {
                    *third_arg(program, program_pointer) = 0;
                }
                program_pointer += 4;
            }
            8 => {
                if *first_arg(program, program_pointer) == *second_arg(program, program_pointer) {
                    *third_arg(program, program_pointer) = 1;
                } else {
                    *third_arg(program, program_pointer) = 0;
                }
                program_pointer += 4;
            }
            99 => break,
            _ => println!("panic"),
        }
    }
    retvals
}

fn run() {
    // let program = [
    //     3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0,
    // ];
    // let program = [
    //     3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23, 23, 4, 23, 99,
    //     0, 0,
    // ];
    let program = [
        3, 8, 1001, 8, 10, 8, 105, 1, 0, 0, 21, 34, 51, 76, 101, 126, 207, 288, 369, 450, 99999, 3,
        9, 102, 4, 9, 9, 1001, 9, 2, 9, 4, 9, 99, 3, 9, 1001, 9, 2, 9, 1002, 9, 3, 9, 101, 3, 9, 9,
        4, 9, 99, 3, 9, 102, 5, 9, 9, 1001, 9, 2, 9, 102, 2, 9, 9, 101, 3, 9, 9, 1002, 9, 2, 9, 4,
        9, 99, 3, 9, 101, 5, 9, 9, 102, 5, 9, 9, 1001, 9, 2, 9, 102, 3, 9, 9, 1001, 9, 3, 9, 4, 9,
        99, 3, 9, 101, 2, 9, 9, 1002, 9, 5, 9, 1001, 9, 5, 9, 1002, 9, 4, 9, 101, 5, 9, 9, 4, 9,
        99, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 1001, 9, 1, 9, 4, 9, 3, 9,
        102, 2, 9, 9, 4, 9, 3, 9, 1001, 9, 1, 9, 4, 9, 3, 9, 101, 1, 9, 9, 4, 9, 3, 9, 1001, 9, 2,
        9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 1001, 9, 1, 9, 4, 9, 3, 9, 1001, 9, 1, 9, 4, 9,
        99, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 101, 2, 9, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9,
        1002, 9, 2, 9, 4, 9, 3, 9, 1001, 9, 2, 9, 4, 9, 3, 9, 101, 1, 9, 9, 4, 9, 3, 9, 1001, 9, 2,
        9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 101, 1, 9, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 99,
        3, 9, 101, 2, 9, 9, 4, 9, 3, 9, 101, 2, 9, 9, 4, 9, 3, 9, 101, 1, 9, 9, 4, 9, 3, 9, 1001,
        9, 2, 9, 4, 9, 3, 9, 101, 2, 9, 9, 4, 9, 3, 9, 101, 2, 9, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4,
        9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 1001, 9, 2, 9, 4, 9, 3, 9, 1001, 9, 1, 9, 4, 9, 99, 3,
        9, 1001, 9, 2, 9, 4, 9, 3, 9, 101, 1, 9, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 1001, 9,
        1, 9, 4, 9, 3, 9, 101, 1, 9, 9, 4, 9, 3, 9, 101, 2, 9, 9, 4, 9, 3, 9, 1001, 9, 1, 9, 4, 9,
        3, 9, 101, 2, 9, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 99, 3, 9,
        1001, 9, 2, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 102, 2, 9,
        9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 1001, 9, 1, 9, 4, 9, 3, 9, 101, 2, 9, 9, 4, 9, 3,
        9, 102, 2, 9, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 99,
    ];

    let setting = &mut [0, 1, 2, 3, 4];
    let mut max: Option<isize> = None;

    HeapPermutationIterator::new(&mut setting[..]).for_each(|setting| {
        let mut instance1 = program.clone();
        let mut instance2 = program.clone();
        let mut instance3 = program.clone();
        let mut instance4 = program.clone();
        let mut instance5 = program.clone();

        let retval = process(&mut instance1, &[setting[0], 0]);
        assert_eq!(retval.len(), 1);
        let retval = process(&mut instance2, &[setting[1], retval[0]]);
        assert_eq!(retval.len(), 1);
        let retval = process(&mut instance3, &[setting[2], retval[0]]);
        assert_eq!(retval.len(), 1);
        let retval = process(&mut instance4, &[setting[3], retval[0]]);
        assert_eq!(retval.len(), 1);
        let retval = process(&mut instance5, &[setting[4], retval[0]]);
        assert_eq!(retval.len(), 1);

        // println!("with setting {:#?} got an output of {}", setting, retval[0]);
        if max.is_none() || max.unwrap() < retval[0] {
            max = Some(retval[0]);
        }
    });
    println!("max value {}", max.unwrap());
}

fn main() {
    run();
}
