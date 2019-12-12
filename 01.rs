/*
 * Copyright (C) 2019  CERN for the benefit of the LHCb collaboration
 * Author: Paul Seyfert <pseyfert@cern.ch>
 *
 * This software is distributed under the terms of the GNU General Public
 * Licence version 3 (GPL Version 3), copied verbatim in the file "LICENSE".
 */

// use std::io;

fn single_mass(m: i32) -> i32 {
    (m / 3) - 2
}

fn single_module(m: i32) -> i32 {
    let mut total_fuel: i32 = 0;
    let mut sip = single_mass(m);
    while sip > 0 {
        total_fuel += sip;
        sip = single_mass(sip);
    }
    total_fuel
}

fn all_modules(m: &[i32]) -> i32 {
    let mut sum = 0;

    for module in m.iter() {
        sum += single_module(*module);
    }
    sum
}

fn main() {
    // let mut modules: std::vec::Vec<i32> = std::vec::Vec::new();
    // let mut input = String::new();
    //
    // println!("starting loop");
    // loop {
    // 	io::stdin().read_line(&mut input);
    // 	println!("input line was '{}'", input);
    // 	let module: i32 = input.trim().parse()
    // 		.expect("couldn't parse number");
    // 	println!("input as number was '{}'", module);
    // 	if module == 0 {
    // 		break;
    // 	}
    // 	modules.push(module);
    // }

    let modules = [
        141589, 93261, 104320, 81961, 99212, 80661, 78734, 76783, 148694, 114382, 141508, 114659,
        107687, 83845, 79690, 59366, 133984, 121431, 144033, 60628, 112095, 78560, 142103, 128943,
        109209, 108999, 144208, 113134, 76591, 57098, 127233, 143194, 85736, 128733, 132275,
        128871, 115164, 50617, 138648, 73023, 98822, 63572, 102841, 54817, 123579, 113025, 90063,
        112330, 117131, 87661, 147299, 146812, 102343, 58763, 59569, 135997, 146057, 108574, 70215,
        74304, 93988, 128150, 76391, 110718, 135513, 62057, 72921, 76889, 67794, 79041, 71987,
        148584, 145472, 131139, 78569, 62584, 85610, 106800, 128550, 81694, 105892, 91250, 69465,
        115222, 73511, 75887, 74891, 127555, 131553, 140892, 69685, 108927, 105759, 105884, 112178,
        109708, 116894, 63459, 133853, 111303,
    ];

    let total_fuel = all_modules(&modules);

    println!("fuel needed: {}", total_fuel);
}
