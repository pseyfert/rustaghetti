/*
 * Copyright (C) 2019  CERN for the benefit of the LHCb collaboration
 * Author: Paul Seyfert <pseyfert@cern.ch>
 *
 * This software is distributed under the terms of the GNU General Public
 * Licence version 3 (GPL Version 3), copied verbatim in the file "LICENSE".
 */

use permutator::HeapPermutationIterator;

const CODESIZE: usize = 531;

struct Program {
    band: [isize; CODESIZE],
    head: usize,
}

fn process(program: &mut Program, inputs: &[isize]) -> Option<isize> {
    let mut input_iterator = 0;

    fn immediate<'a>(
        programband: &'a mut [isize],
        program_pointer: usize,
        offset: usize,
    ) -> &mut isize {
        &mut programband[program_pointer + offset]
    }
    fn position<'a>(
        programband: &'a mut [isize],
        program_pointer: usize,
        offset: usize,
    ) -> &mut isize {
        let addr: usize = programband[program_pointer + offset] as usize;
        &mut programband[addr]
    }

    loop {
        let opcode = program.band[program.head] % 100;

        let first_arg: fn(programband: &mut [isize], program_pointer: usize) -> &mut isize;
        let second_arg: fn(programband: &mut [isize], program_pointer: usize) -> &mut isize;
        let third_arg: fn(programband: &mut [isize], program_pointer: usize) -> &mut isize;

        if (program.band[program.head] / 100) % 10 == 0 {
            fn retval<'a>(programband: &'a mut [isize], program_pointer: usize) -> &mut isize {
                return position(programband, program_pointer, 1);
            }
            first_arg = retval
        } else {
            fn retval<'a>(programband: &'a mut [isize], program_pointer: usize) -> &mut isize {
                return immediate(programband, program_pointer, 1);
            }
            first_arg = retval
        };
        if (program.band[program.head] / 1000) % 10 == 0 {
            fn retval<'a>(programband: &'a mut [isize], program_pointer: usize) -> &mut isize {
                return position(programband, program_pointer, 2);
            }
            second_arg = retval
        } else {
            fn retval<'a>(programband: &'a mut [isize], program_pointer: usize) -> &mut isize {
                return immediate(programband, program_pointer, 2);
            }
            second_arg = retval
        };
        if (program.band[program.head] / 10000) % 10 == 0 {
            fn retval<'a>(programband: &'a mut [isize], program_pointer: usize) -> &mut isize {
                return position(programband, program_pointer, 3);
            }
            third_arg = retval
        } else {
            fn retval<'a>(programband: &'a mut [isize], program_pointer: usize) -> &mut isize {
                return immediate(programband, program_pointer, 3);
            }
            third_arg = retval
        };

        match opcode {
            1 => {
                *third_arg(&mut program.band, program.head) =
                    *first_arg(&mut program.band, program.head)
                        + *second_arg(&mut program.band, program.head);
                program.head += 4;
            }
            2 => {
                *third_arg(&mut program.band, program.head) =
                    *first_arg(&mut program.band, program.head)
                        * *second_arg(&mut program.band, program.head);
                program.head += 4;
            }
            3 => {
                *first_arg(&mut program.band, program.head) = inputs[input_iterator];
                input_iterator += 1;
                program.head += 2;
            }
            4 => {
                let retval = *first_arg(&mut program.band, program.head);
                program.head += 2;
                return Some(retval);
            }
            5 => {
                if *first_arg(&mut program.band, program.head) != 0 {
                    program.head = *second_arg(&mut program.band, program.head) as usize;
                } else {
                    program.head += 3;
                }
            }
            6 => {
                if *first_arg(&mut program.band, program.head) == 0 {
                    program.head = *second_arg(&mut program.band, program.head) as usize;
                } else {
                    program.head += 3;
                }
            }
            7 => {
                if *first_arg(&mut program.band, program.head)
                    < *second_arg(&mut program.band, program.head)
                {
                    *third_arg(&mut program.band, program.head) = 1;
                } else {
                    *third_arg(&mut program.band, program.head) = 0;
                }
                program.head += 4;
            }
            8 => {
                if *first_arg(&mut program.band, program.head)
                    == *second_arg(&mut program.band, program.head)
                {
                    *third_arg(&mut program.band, program.head) = 1;
                } else {
                    *third_arg(&mut program.band, program.head) = 0;
                }
                program.head += 4;
            }
            99 => return None,
            _ => println!("panic"),
        }
    }
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

    let setting = &mut [5, 6, 7, 8, 9];
    let mut max: Option<isize> = None;

    HeapPermutationIterator::new(&mut setting[..]).for_each(|setting| {
        let mut instance1 = Program {
            band: program.clone(),
            head: 0,
        };
        let mut instance2 = Program {
            band: program.clone(),
            head: 0,
        };
        let mut instance3 = Program {
            band: program.clone(),
            head: 0,
        };
        let mut instance4 = Program {
            band: program.clone(),
            head: 0,
        };
        let mut instance5 = Program {
            band: program.clone(),
            head: 0,
        };

        let mut retval = process(&mut instance1, &[setting[0], 0]);
        retval = process(&mut instance2, &[setting[1], retval.unwrap()]);
        retval = process(&mut instance3, &[setting[2], retval.unwrap()]);
        retval = process(&mut instance4, &[setting[3], retval.unwrap()]);
        retval = process(&mut instance5, &[setting[4], retval.unwrap()]);
        let mut lastret = retval.unwrap();

        while retval.is_some() {
            retval = process(&mut instance1, &[retval.unwrap()]);
            if retval.is_none() {
                break;
            }
            retval = process(&mut instance2, &[retval.unwrap()]);
            if retval.is_none() {
                break;
            }
            retval = process(&mut instance3, &[retval.unwrap()]);
            if retval.is_none() {
                break;
            }
            retval = process(&mut instance4, &[retval.unwrap()]);
            if retval.is_none() {
                break;
            }
            retval = process(&mut instance5, &[retval.unwrap()]);
            if retval.is_some() {
                lastret = retval.unwrap();
            }
        }

        // println!("with setting {:#?} got an output of {}", setting, retval[0]);
        if max.is_none() || max.unwrap() < lastret {
            max = Some(lastret);
        }
    });
    println!("max value {}", max.unwrap());
}

fn main() {
    run();
}
