use rocket::{options};
/// Catches all OPTION requests in order to get the CORS related Fairing triggered.
#[options("/<_..>")]
pub fn all_options_handler() {
    /* Intentionally left empty */
}
