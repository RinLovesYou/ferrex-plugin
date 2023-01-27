use crate::log;

scotch_guest::export_alloc!();

#[scotch_guest::guest_function]
fn init() {
    crate::run().unwrap_or_else(|e| {
        log!("{}", e.to_string());
    })
}