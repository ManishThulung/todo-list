use std::error::Error;
use std::fs::File;
use std::io::Write;

pub struct Config {
  task: String,
}

impl Config {
  pub fn build (mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
    args.next();

    let task = match args.next() {
      Some(task) => task,
      None => return Err("something went wrong"),
    };

    Ok(Config{
      task
    })
  }
}

pub fn add_todo (config: Config) -> Result<(), Box<dyn Error>>  {
  let mut file = File::create("todo-list.txt")?;

  file.write(config.task.as_bytes())?;
  
  Ok(())
}
