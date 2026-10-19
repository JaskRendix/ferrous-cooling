use ferrous_cooling::lhp::{HotCase, hot_case_inventory};

fn main() {
    let inventory = hot_case_inventory(HotCase {
        liquid_density: 550.0,
        vapor_density: 8.0,
        liquid_line_volume: 1.0e-5,
        primary_wick_volume: 2.0e-6,
        secondary_wick_volume: 2.0e-6,
        groove_volume: 1.0e-6,
        vapor_line_volume: 5.0e-6,
        condenser_volume: 1.5e-5,
        compensation_chamber_volume: 1.0e-5,
        vapor_fraction: 0.3,
    });

    println!("Required inventory: {:.6} kg", inventory);
}
