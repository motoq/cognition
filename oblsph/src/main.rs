use std::env;
use std::process;

use oblsph::Config;
use oblsph::plot_os::plot_os;

use cogs::oblate_spheroid;

fn main() {

    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("problem parsing arguments: {err}");
        process::exit(0);
    });

    let os = oblate_spheroid::OblateSpheroid::try_from(&(config.eccentricity,
                                                         config.semimajor,
                                                         config.longitude,
                                                         config.latitude))
        .expect("OblateSpheroid Construction: ");

    println!("OblateSpheroid {}", os);

    println!("\n");

    if config.plot_prefix.len() == 0 {
        process::exit(0);
    }

    //let _ = plot_os(&os);
    match plot_os(&os, &config) {
        Ok(_) => println!("Generated file"),
        Err(msg) => println!("Plot not OK {}", msg),
    }

}
