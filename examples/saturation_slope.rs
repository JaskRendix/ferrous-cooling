use ferrous_cooling::Fluid;

fn main() {
    let ammonia = Fluid::ammonia();

    let slope = ammonia.saturation_pressure_slope(300.0);

    println!("dP/dT = {:.3} Pa/K", slope);
}
