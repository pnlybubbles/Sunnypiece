use std::io;
use std::io::Write;

pub fn progress_indicator(progress: usize, total: usize) {
  let stdout = io::stdout();
  write!(
    &mut stdout.lock(),
    "\rprocessing... ({}/{} : {:.0}%) ",
    progress,
    total,
    progress as f32 / total as f32 * 100.0
  )
  .unwrap();
}
