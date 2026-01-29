pub mod atomize;
pub mod branch;
pub mod commit;
pub mod diff;
pub mod walk;

pub use diff::{changed_files, open_repo, resolve_commit};
