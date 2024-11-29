use std::fs::File;
use std::io::{Write, BufWriter};

use nalgebra as na;

pub fn plot_arrow(out: &mut BufWriter<File>,
                  orgn: &na::SMatrix<f64, 3, 1>,
                  dstn: &na::SMatrix<f64, 3, 1>,
                  rgb: &str) -> std::io::Result<()> {
    write!(out, "\nset arrow from {:.3e}, {:.3e}, {:.3e}",
                 orgn[0], orgn[1], orgn[2])?;
    write!(out, " to {:.3e}, {:.3e}, {:.3e}", dstn[0], dstn[1], dstn[2])?;
    write!(out, " filled back lw 3 lc rgb \"{}\"", rgb)?;
    Ok(())
}
