use std::fs::File;
use std::io::{Write, BufWriter};

use nalgebra as na;

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
                let (e1, e2, e3) = os.get_cov_basis();
                let mut xyz = xyz0 + e1;
                plot_arrow(&mut writer, &xyz0, &xyz, &"red".to_string())?;
                xyz = xyz0 + e2;
                plot_arrow(&mut writer, &xyz0, &xyz, &"green".to_string())?;
                xyz = xyz0 + e3;
                plot_arrow(&mut writer, &xyz0, &xyz, &"blue".to_string())?;
            }
            OsPlotType::BasisContravariant => {
                let xyz0 = os.get_cartesian();
                let (e1, e2, e3) = os.get_cont_basis();
                let mut xyz = xyz0 + e1;
                plot_arrow(&mut writer, &xyz0, &xyz, &"red".to_string())?;
                xyz = xyz0 + e2;
                plot_arrow(&mut writer, &xyz0, &xyz, &"green".to_string())?;
                xyz = xyz0 + e3;
                plot_arrow(&mut writer, &xyz0, &xyz, &"blue".to_string())?;
            }
        }
    }

    write!(writer, "\nset view equal xyz")?;

    //write!(writer, "\npause mouse close\n")

    writer.flush()?;
    Ok(())
}

fn plot_arrow(out: &mut BufWriter<File>,
              orgn: &na::SMatrix<f64, 3, 1>,
              dstn: &na::SMatrix<f64, 3, 1>,
              rgb: &str) -> std::io::Result<()> {
    write!(out, "\nset arrow from {:.3e}, {:.3e}, {:.3e}",
                 orgn[0], orgn[1], orgn[2])?;
    write!(out, " to {:.3e}, {:.3e}, {:.3e}", dstn[0], dstn[1], dstn[2])?;
    write!(out, " filled back lw 3 lc rgb \"{}\"", rgb)?;
    Ok(())
}


