use ferrous_cooling::lhp::{ColdCase, cold_case_inventory};

fn main() {
    let inventory = cold_case_inventory(ColdCase {
        rho_liquid: 600.0,
        rho_vapor: 5.0,
        loop_volume: 1.0e-4,
        cc_volume: 2.0e-5,
        liquid_fraction: 0.8,
    });

    println!("Required inventory: {:.6} kg", inventory);
}
