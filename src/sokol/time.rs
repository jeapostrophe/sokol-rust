// machine generated, do not edit


extern { pub fn stm_setup() -> (); }
pub fn setup() -> () { unsafe {
    return stm_setup();
} }
extern { pub fn stm_now() -> u64; }
pub fn now() -> u64 { unsafe {
    return stm_now();
} }
extern { pub fn stm_diff(new_ticks: u64, old_ticks: u64) -> u64; }
pub fn diff(new_ticks: u64, old_ticks: u64) -> u64 { unsafe {
    return stm_diff(new_ticks, old_ticks);
} }
extern { pub fn stm_since(start_ticks: u64) -> u64; }
pub fn since(start_ticks: u64) -> u64 { unsafe {
    return stm_since(start_ticks);
} }
extern { pub fn stm_laptime(last_time: *mut u64) -> u64; }
pub fn laptime(last_time: *mut u64) -> u64 { unsafe {
    return stm_laptime(last_time);
} }
extern { pub fn stm_round_to_common_refresh_rate(frame_ticks: u64) -> u64; }
pub fn round_to_common_refresh_rate(frame_ticks: u64) -> u64 { unsafe {
    return stm_round_to_common_refresh_rate(frame_ticks);
} }
extern { pub fn stm_sec(ticks: u64) -> f64; }
pub fn sec(ticks: u64) -> f64 { unsafe {
    return stm_sec(ticks);
} }
extern { pub fn stm_ms(ticks: u64) -> f64; }
pub fn ms(ticks: u64) -> f64 { unsafe {
    return stm_ms(ticks);
} }
extern { pub fn stm_us(ticks: u64) -> f64; }
pub fn us(ticks: u64) -> f64 { unsafe {
    return stm_us(ticks);
} }
extern { pub fn stm_ns(ticks: u64) -> f64; }
pub fn ns(ticks: u64) -> f64 { unsafe {
    return stm_ns(ticks);
} }
