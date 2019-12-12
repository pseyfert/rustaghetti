/*
 * Copyright (C) 2019  CERN for the benefit of the LHCb collaboration
 * Author: Paul Seyfert <pseyfert@cern.ch>
 *
 * This software is distributed under the terms of the GNU General Public
 * Licence version 3 (GPL Version 3), copied verbatim in the file "LICENSE".
 */

fn process(program: &mut [isize]) -> isize {
    let mut program_pointer: usize = 0;

    fn immediate<'a>(program: &'a mut [isize], program_pointer: usize, offset: usize) -> &mut isize {
        &mut program[program_pointer+offset]
    }
    fn position<'a>(program: &'a mut [isize], program_pointer: usize, offset: usize) -> &mut isize {
        let addr: usize = program[program_pointer+offset] as usize;
        &mut program[addr]
    }

    loop {
        let opcode = program[program_pointer] % 100;

        let first_arg: fn(program: &mut [isize], program_pointer: usize) -> &mut isize;
        let second_arg: fn(program: &mut [isize], program_pointer: usize) -> &mut isize;
        let third_arg: fn(program: &mut [isize], program_pointer: usize) -> &mut isize;


        if (program[program_pointer]/100)%10 == 0 {
            fn retval<'a>(program: &'a mut [isize], program_pointer: usize) -> &mut isize {
                return position(program, program_pointer, 1)
            }
            first_arg = retval
        } else {
            fn retval<'a>(program: &'a mut [isize], program_pointer: usize) -> &mut isize {
                return immediate(program, program_pointer, 1)
            }
            first_arg = retval
        };
        if (program[program_pointer]/1000)%10 == 0 {
            fn retval<'a>(program: &'a mut [isize], program_pointer: usize) -> &mut isize {
                return position(program, program_pointer, 2)
            }
            second_arg = retval
        } else {
            fn retval<'a>(program: &'a mut [isize], program_pointer: usize) -> &mut isize {
                return immediate(program, program_pointer, 2)
            }
            second_arg = retval
        };
        if (program[program_pointer]/10000)%10 == 0 {
            fn retval<'a>(program: &'a mut [isize], program_pointer: usize) -> &mut isize {
                return position(program, program_pointer, 3)
            }
            third_arg = retval
        } else {
            fn retval<'a>(program: &'a mut [isize], program_pointer: usize) -> &mut isize {
                return immediate(program, program_pointer, 3)
            }
            third_arg = retval
        };

        let mut inc: usize = 1;
        match opcode {
            1 => {
                *third_arg(program, program_pointer) = *first_arg(program, program_pointer) + *second_arg(program, program_pointer);
                inc = 4;
            },
            2 => {
                *third_arg(program, program_pointer) = *first_arg(program, program_pointer) * *second_arg(program, program_pointer);
                inc = 4;
            },
            3 => {
                // take input
                println!("feeding 1");
                *first_arg(program,program_pointer) = 1;
                inc = 2;
            },
            4 => {
                println!("{}", *first_arg(program,program_pointer));
                inc = 2;
            },
            99 => break,
            _ => println!("panic"),
        }

        program_pointer += inc;
    }

    program[0]
}

fn run() -> isize {
    let mut program = [3,225,1,225,6,6,1100,1,238,225,104,0,1102,46,47,225,2,122,130,224,101,-1998,224,224,4,224,1002,223,8,223,1001,224,6,224,1,224,223,223,1102,61,51,225,102,32,92,224,101,-800,224,224,4,224,1002,223,8,223,1001,224,1,224,1,223,224,223,1101,61,64,225,1001,118,25,224,101,-106,224,224,4,224,1002,223,8,223,101,1,224,224,1,224,223,223,1102,33,25,225,1102,73,67,224,101,-4891,224,224,4,224,1002,223,8,223,1001,224,4,224,1,224,223,223,1101,14,81,225,1102,17,74,225,1102,52,67,225,1101,94,27,225,101,71,39,224,101,-132,224,224,4,224,1002,223,8,223,101,5,224,224,1,224,223,223,1002,14,38,224,101,-1786,224,224,4,224,102,8,223,223,1001,224,2,224,1,223,224,223,1,65,126,224,1001,224,-128,224,4,224,1002,223,8,223,101,6,224,224,1,224,223,223,1101,81,40,224,1001,224,-121,224,4,224,102,8,223,223,101,4,224,224,1,223,224,223,4,223,99,0,0,0,677,0,0,0,0,0,0,0,0,0,0,0,1105,0,99999,1105,227,247,1105,1,99999,1005,227,99999,1005,0,256,1105,1,99999,1106,227,99999,1106,0,265,1105,1,99999,1006,0,99999,1006,227,274,1105,1,99999,1105,1,280,1105,1,99999,1,225,225,225,1101,294,0,0,105,1,0,1105,1,99999,1106,0,300,1105,1,99999,1,225,225,225,1101,314,0,0,106,0,0,1105,1,99999,1008,677,226,224,1002,223,2,223,1005,224,329,1001,223,1,223,107,677,677,224,102,2,223,223,1005,224,344,101,1,223,223,1107,677,677,224,102,2,223,223,1005,224,359,1001,223,1,223,1108,226,226,224,1002,223,2,223,1006,224,374,101,1,223,223,107,226,226,224,1002,223,2,223,1005,224,389,1001,223,1,223,108,226,226,224,1002,223,2,223,1005,224,404,1001,223,1,223,1008,677,677,224,1002,223,2,223,1006,224,419,1001,223,1,223,1107,677,226,224,102,2,223,223,1005,224,434,1001,223,1,223,108,226,677,224,102,2,223,223,1006,224,449,1001,223,1,223,8,677,226,224,102,2,223,223,1006,224,464,1001,223,1,223,1007,677,226,224,1002,223,2,223,1006,224,479,1001,223,1,223,1007,677,677,224,1002,223,2,223,1005,224,494,1001,223,1,223,1107,226,677,224,1002,223,2,223,1006,224,509,101,1,223,223,1108,226,677,224,102,2,223,223,1005,224,524,1001,223,1,223,7,226,226,224,102,2,223,223,1005,224,539,1001,223,1,223,8,677,677,224,1002,223,2,223,1005,224,554,101,1,223,223,107,677,226,224,102,2,223,223,1006,224,569,1001,223,1,223,7,226,677,224,1002,223,2,223,1005,224,584,1001,223,1,223,1008,226,226,224,1002,223,2,223,1006,224,599,101,1,223,223,1108,677,226,224,102,2,223,223,1006,224,614,101,1,223,223,7,677,226,224,102,2,223,223,1005,224,629,1001,223,1,223,8,226,677,224,1002,223,2,223,1006,224,644,101,1,223,223,1007,226,226,224,102,2,223,223,1005,224,659,101,1,223,223,108,677,677,224,1002,223,2,223,1006,224,674,1001,223,1,223,4,223,99,226];

    let retval = process(&mut program);

    return retval;
}

fn main() {
    run();
}
