use cogs::utl_const::RAD_PER_DEG;
use cogs::oblate_spheroid;

/**
 * This program performs a consistency check on the oblate spheroidal
 * struct.  It iterates over a wide range of eccentricities, semimajor axes
 * longitudes, and latitude.  Each set is used to initialize an OblateSpheroid
 * object.  The Cartesian coordinates are then pulled to initialize another
 * OblateSpheroid struct.  The difference between all the elements are RSS'ed
 * and added to a total error that is printed upon completion.
 */
fn main() {
    let decc: f64 = 0.05;
    let dsma: f64 = 0.1;
    let dlon: f64 = RAD_PER_DEG*3.0;
    let dlat: f64 = 0.05;

    let mut count = 0;
    let mut rss_error: f64  = 0.0;

    let mut ecc: f64 = 0.0;
    while ecc < 0.9 {
        let mut sma: f64 = 0.1;
        while sma < 7.5 {
            let mut lat: f64 = -0.9;
            while lat < 0.9 {
                let mut lon: f64 = RAD_PER_DEG*-180.0;
                while lon < RAD_PER_DEG*180.0 {
                    let os1 =
                            oblate_spheroid::
                            OblateSpheroid::try_from(&(ecc, sma, lon, lat))
                            .expect("OblateSpheroid Construction: ");
                    let xyz = os1.get_cartesian();
                    let os2 = oblate_spheroid::
                              OblateSpheroid::try_from(&(ecc, xyz))
                                  .expect("OblateSpheroid Construction: ");
        
                    let de = os2.get_eccentricity() - os1.get_eccentricity();
                    let da = os2.get_semimajor() - os1.get_semimajor();
                    let dn = os2.get_longitude() - os1.get_longitude();
                    let dt = os2.get_latitude() - os1.get_latitude();
                    rss_error += (de*de + da*da + dn*dn + dt*dt).sqrt();
                    count += 1;
        
                    lon += dlon;
                }
                lat += dlat;
            }
            sma += dsma;
        }
        ecc += decc;
    }
    println!("RSS Error over {} tests: {}", count, rss_error);
}
