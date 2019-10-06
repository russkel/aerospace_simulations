#[macro_use] extern crate prettytable;
use prettytable::{Table, format};

fn geomspace(start:f64, end:f64, spacing:u32) -> Vec<f64> {
    let delta = (end.log10() - start.log10()) / f64::from(spacing-1);

    (0..spacing).map(|i| start * 10_f64.powf(delta * f64::from(i))).collect()
}

fn kg_m3_to_slug_ft3(kg_m3:f64) -> f64 {
    kg_m3 / 515.378819
}

fn main() {
    println!("Conversion table of air density from kg/m³ to slug/ft³:");
    let mut table = Table::new();
    table.set_titles(row!["kg/m³", "slug/ft³"]);

    let (rho_start, rho_end) = (1.225_f64, 0.01225_f64); // kg/m^3
    let log_spacing = 10;

    for i in geomspace(rho_start, rho_end, log_spacing) {
        table.add_row(row![format!("{:.6}", i), format!("{:.6}", kg_m3_to_slug_ft3(i))]);
    }

    table.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);
    table.printstd();
}
