extern crate time;

use time::Duration;
use time::PrimitiveDateTime as DateTime;

// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
    let one_billion_seconds = Duration::SECOND * 1_000_000_000;
    start + one_billion_seconds
}
