/// Calculates nicotine ammount
pub fn calculate_nic(target_v: f64, want_nic_conc: f64, base_conc: f64) -> f64 {
    (target_v * want_nic_conc) / base_conc
}
