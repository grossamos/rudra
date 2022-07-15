mod io;
mod debug;

pub use io::read_file_to_string_or_err;
pub use debug::print_debug_message;
pub use debug::print_error_and_exit;
