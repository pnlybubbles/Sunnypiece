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

pub struct ProgressIndicator {
  total: usize,
  progress: usize,
  start_time: time::Tm,
}

impl ProgressIndicator {
  pub fn new(total: usize) -> Self {
    let start_time = time::now();
    println!("start: {}", start_time.strftime("%+").unwrap());

    ProgressIndicator {
      total: total,
      progress: 0,
      start_time: start_time,
    }
  }

  pub fn next(&mut self) {
    self.progress += 1;
    self.update()
  }

  pub fn end(&self) {
    let end_time = time::now();
    println!("");
    println!("end: {}", end_time.strftime("%+").unwrap());
    println!(
      "elapse: {}s",
      (end_time - self.start_time).num_milliseconds() as f32 / 1000.0
    );
  }

  fn update(&self) {
    let stdout = io::stdout();
    write!(
      &mut stdout.lock(),
      "\rprocessing... ({}/{} : {:.0}%) ",
      self.progress,
      self.total,
      self.progress as f32 / self.total as f32 * 100.0
    )
    .unwrap();
  }
}
