use ferrous_cooling::lhp::{heat_leak, subcooling_heat};

fn main() {
    let heat_leak_rate = heat_leak(0.25, 310.0, 300.0);

    let cc_heat = subcooling_heat(0.001, 4700.0, 300.0, 295.0);

    println!("Heat leak: {:.3} W", heat_leak_rate);

    println!("Compensation chamber heat: {:.3} W", cc_heat);
}
