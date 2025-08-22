use crate::env::get_f64;

pub fn risk_threshold(equity: f64) -> f64 {
    equity * get_f64("RISK_MAX_PER_TRADE")
}
