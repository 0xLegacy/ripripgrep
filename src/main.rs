use std::fs;
use std::io::{self, BufRead, Write};
use std::path::Path;
use std::sync::Arc;
use std::thread;
use dialoguer::{theme::ColorfulTheme, Confirm};

const DIRECTORY_PATH: &str = "Folder Path";

fn read_pseudo() -> io::Result<String> {
   print!("Who are you looking for, you bandit? ");
   io::stdout().flush()?;
   let mut pseudo = String::new();
   io::stdin().read_line(&mut pseudo)?;
   Ok(pseudo.trim().to_string())
}

fn process_file(path: &Path, pseudo: Arc<String>) {
   let file = fs::File::open(&path).unwrap();
   let reader = io::BufReader::new(file);

   for result in reader.split(b'\n') {
       let line = String::from_utf8_lossy(&result.unwrap()).into_owned();
       if line.contains(&*pseudo) {
           println!("{}", line);
       }
   }
}

fn main() -> io::Result<()> {
   loop {
       let pseudo = match read_pseudo() {
           Ok(p) => p,
           Err(e) => {
               eprintln!("Error reading standard input : {}", e);
               return Err(e);
           }
       };

       let path = Path::new(DIRECTORY_PATH);

       if path.is_dir() {
           let entries = fs::read_dir(path)?;
           let pseudo = Arc::new(pseudo);
           let mut handles = vec![];

           for entry in entries {
               let entry = entry?;
               let path = entry.path();
               if path.is_file() && path.extension().unwrap_or_default() == "txt" {
                  let pseudo = Arc::clone(&pseudo);
                  let handle = thread::spawn(move || {
                      process_file(&path, pseudo);
                  });
                  handles.push(handle);
               }
           }

           for handle in handles {
               handle.join().unwrap();
           }
       } else {
           eprintln!("folder not found");
       }

       let cont = Confirm::with_theme(&ColorfulTheme::default())
           .with_prompt("Would you like to search again?")
           .interact()
           .unwrap_or(false);
       
       if !cont {
           break;
       }
   }

   Ok(())
}