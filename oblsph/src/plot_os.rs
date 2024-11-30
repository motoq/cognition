/*
 * Copyright 2024 Kurt Motekew
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

use std::fs::File;
use std::io::{Write, BufWriter};

use cogs::gp_plot::gp_plot_basis;
use cogs::oblate_spheroid;

use crate::Config;

pub enum OsPlotType {
    BasisCovariant,
    BasisContravariant,
}

pub fn plot_os(os: &oblate_spheroid::OblateSpheroid,
               cfg: &Config) -> std::io::Result<()> {

    println!("Semiminor {}", os.get_semiminor());

    let mut file_name = cfg.plot_prefix.clone();
    file_name.push_str(".gp");
    let file = File::create(&file_name)?;
    let mut writer = BufWriter::new(file);
    write!(writer, "set title \"Oblate Spheroid\"")?;
    write!(writer, "\nset parametric")?;
    write!(writer, "\nset isosamples 25")?;
    write!(writer, "\nsplot [-pi:pi][-pi/2:pi/2]")?;
    write!(writer, " {:.3e}*cos(u)*cos(v)", os.get_semimajor())?;
    write!(writer, ", {:.3e}*sin(u)*cos(v)", os.get_semimajor())?;
    write!(writer, ", {:.3e}*sin(v)", os.get_semiminor())?;

    for plt in &cfg.plot_types {
        match plt {
            OsPlotType::BasisCovariant => {
                let xyz0 = os.get_cartesian();
                let basis = os.get_cov_basis();
                gp_plot_basis(&mut writer, &xyz0, &basis)?;
            }
            OsPlotType::BasisContravariant => {
                let xyz0 = os.get_cartesian();
                let basis = os.get_cont_basis();
                gp_plot_basis(&mut writer, &xyz0, &basis)?;
            }
        }
    }

    write!(writer, "\nset view equal xyz")?;

    //write!(writer, "\npause mouse close\n")

    writer.flush()?;
    Ok(())
}

