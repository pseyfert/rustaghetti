/*
 * Copyright (C) 2019  CERN for the benefit of the LHCb collaboration
 * Author: Paul Seyfert <pseyfert@cern.ch>
 *
 * This software is distributed under the terms of the GNU General Public
 * Licence version 3 (GPL Version 3), copied verbatim in the file "LICENSE".
 */

fn check(ds: [i32; 6]) -> bool {
    for i in 0..5 {
        if ds[i] > ds[i + 1] {
            return false;
        }
    }
    if ds[0] == ds[1] && ds[0] != ds[2] {
        return true;
    }
    if ds[5] == ds[4] && ds[4] != ds[3] {
        return true;
    }
    for i in 1..4 {
        if ds[i] == ds[i + 1] && ds[i] != ds[i - 1] && ds[i] != ds[i + 2] {
            return true;
        }
    }

    return false;
}

fn main() {
    let mut valids: usize = 0;
    for i in 234208..765869 {
        let digits: [i32; 6] = [
            (i / 100000) % 10,
            (i / 10000) % 10,
            (i / 1000) % 10,
            (i / 100) % 10,
            (i / 10) % 10,
            (i) % 10,
        ];
        if check(digits) {
            valids += 1;
        }
    }
    println!("{}", valids);
}
