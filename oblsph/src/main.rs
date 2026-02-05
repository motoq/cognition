/*
 * Copyright 2024 Kurt Motekew
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

use std::env;
use std::process;

use oblsph::Config;
use oblsph::plot_os::plot_os;

use cogs::oblate_spheroid;

/// Plots requested aspects related to the oblate spheroidal reference
/// frame given an input file.
fn main() {
    // Read input file
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("problem parsing arguments: {err}");
        process::exit(0);
    });

    // Create oblate spheroid object upon which analysis is based
    let os = oblate_spheroid::OblateSpheroid::try_from(&(config.eccentricity,
                                                         config.semimajor,
                                                         config.longitude,
                                                         config.latitude))
        .expect("OblateSpheroid Construction Failed: ");

    println!("OblateSpheroid {}", os);

    if config.plot_prefix.len() == 0 {
        process::exit(0);
    }

    // Run through analysis/plot requests
    match plot_os(&os, &config) {
        Ok(_) => println!("Generated file"),
        Err(msg) => println!("Plot not OK {}", msg),
    }

}
