/*
 * Copyright 2024 Kurt Motekew
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

use std::fs::File;
use std::io::{Write, BufWriter};

use nalgebra as na;

/**
 * Creates a Gnuplot command, as a String, to plot an arrow from one
 * Cartesian point to another.  The color must be indicated, but the
 * linewidth is fixed at '3'.
 *
 * @param  orgn  Origin, Cartesian coordinates
 * @param  dstn  Destination, Cartesian coordinates
 * @param  rgb   Gnuplot compatble rbg color
 *
 * @return  Gnuplot command to plot an arrow
 */
pub fn gp_arrow(orgn: &na::SMatrix<f64, 3, 1>,
                dstn: &na::SMatrix<f64, 3, 1>,
                rgb: &str) -> String {
    format!("set arrow from {:.3e}, {:.3e}, {:.3e}",
             orgn[0], orgn[1], orgn[2]) +
        &format!(" to {:.3e}, {:.3e}, {:.3e}", dstn[0], dstn[1], dstn[2]) +
        &format!(" lw 3 lc rgb \"{}\"", rgb)
}

/**
 * Writes Gnuplot commands to a BufWriter that plots three vectors as
 * with the arrow command, all originating from the same point.  The
 * vectors are color coded by order as red, green, blue (RGB - 1st, 2nd, 3rd).
 *
 * @param  writer  Destination for output
 * @param  xyz0    Origin for basis vectors
 * @param  basis   Three basis vectors to plot
 *
 * @return  Errors if a problem with writing to the BufWriter occurs
 */
pub fn gp_plot_basis(writer: &mut BufWriter<File>,
                     xyz0: &na::SMatrix<f64, 3, 1>,
                     basis: &(na::SMatrix<f64, 3, 1>,
                              na::SMatrix<f64, 3, 1>,
                              na::SMatrix<f64, 3, 1>)) -> std::io::Result<()> {
    let (e1, e2, e3) = basis;
    let mut xyz = xyz0 + e1;
    write!(writer, "\n{}", gp_arrow(&xyz0, &xyz, &"red".to_string()))?;
    xyz = xyz0 + e2;
    write!(writer, "\n{}", gp_arrow(&xyz0, &xyz, &"green".to_string()))?;
    xyz = xyz0 + e3;
    write!(writer, "\n{}", gp_arrow(&xyz0, &xyz, &"blue".to_string()))?;
    Ok(())
}
