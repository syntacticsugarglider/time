//! A method to obtain the local offset from UTC.

#[cfg_attr(target_family = "windows", path = "windows.rs")]
#[cfg_attr(target_family = "unix", path = "unix.rs")]
#[cfg_attr(
    all(
        target_arch = "wasm32",
        not(any(target_os = "emscripten", target_os = "wasi"))
    ),
    path = "wasm_js.rs"
)]
mod imp;

use crate::{OffsetDateTime, UtcOffset};

/// Attempt to obtain the system's UTC offset. If the offset cannot be determined, `None` is
/// returned.
pub(crate) fn local_offset_at(datetime: OffsetDateTime) -> Option<UtcOffset> {
    imp::local_offset_at(datetime)
}
