use std::error::Error;
use std::fs::{self, OpenOptions};
use std::io::Write;

pub fn add_todo (task: String) -> Result<(), Box<dyn Error>>  {
  // let mut file = fs::File::create("todo-list.txt")?;
  // file.write(task.as_bytes())?;

  let mut data_file = OpenOptions::new()
  .append(true)
  .open("todo-list.txt")?;

  data_file.write(task.as_bytes())?;

  Ok(())
}

pub fn read_todo () -> Result<(), Box<dyn Error>>  {
  let tasks = fs::read_to_string("todo-list.txt")?;
  for task in tasks.lines() {
    println!("{}", task);
  }
  Ok(())
}



// pub struct Config {
//   task: String,
// }

// impl Config {
//   pub fn build (mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
//     args.next();

//     let task = match args.next() {
//       Some(task) => task,
//       None => return Err("something went wrong"),
//     };

//     Ok(Config{
//       task
//     })
//   }
// }


