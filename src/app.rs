use std::error;
use std::fs;
use std::process::Command;
use std::path::PathBuf;
use std::fs::File;
use std::io::{Read, Write};
use std::env;
use crossterm::{
      execute,
      terminal::{enable_raw_mode, disable_raw_mode},
      ExecutableCommand,
      terminal::{Clear, ClearType},
};
use std::io::stdout;
use crossterm::style::ResetColor;

pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug)]
pub struct App {
    pub running: bool,
    pub modules: Vec<String>,  // List of module names
    pub current_module_index: usize,  // Index of the currently active module
    pub active_element: ActiveElement,
    pub course_name: String,
    pub module_binaries: Vec<String>,
    pub module_completion: Vec<bool>,
}

#[derive(Debug, PartialEq)]
pub enum ActiveElement {
    None,
    ResumeButton,
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            modules: Vec::new(),
            current_module_index: 0,
            active_element: ActiveElement::None,
            module_binaries: Vec::new(),
            course_name: "__NULL_____".to_string(),
            module_completion: Vec::new(),
        }
    }
}

impl App {
    pub fn new(course_name: String) -> Self {
        let mut app = Self {
            running: true,
            modules: Vec::new(),
            module_binaries: Vec::new(),
            course_name,
            current_module_index: 0,
            active_element: ActiveElement::None,
            module_completion: vec![false; 3],
        };

        app.load_modules();
        app
    }

    pub fn file_exists(&self, file_path: &str) -> bool{
      let full_path = PathBuf::from(&file_path);
      return full_path.exists();
    }

    pub fn create_file(&self, file_path: &str) -> File {
      let full_path = PathBuf::from(&file_path);
      let mut file = File::create(&full_path).unwrap();
      for module in &self.modules {
	  writeln!(file, "{} 0", module).unwrap();
      }
      return file;
    }

    pub fn create_file_sorted(&self, file_path: &str) -> File {
        let full_path = PathBuf::from(file_path);
        let mut file = File::create(&full_path).unwrap();

        // Clone module vector and sort based on numeric prefix
        let mut sorted_modules = self.modules.clone();
        sorted_modules.sort_by_key(|module| {
            module.split('_').next().unwrap().parse::<i32>().unwrap_or(0)
        });

        // Write to file
        for module in &sorted_modules {
            writeln!(file, "{} 0", module).unwrap();
        }

        return file;
    }

    pub fn complete(&self, file_path: &str) -> bool {
      let full_path = PathBuf::from(&file_path);
      let mut file = File::open(&full_path).expect("result");
      let mut content = String::new();
      file.read_to_string(&mut content).unwrap();
      for line in content.lines() {
	let parts: Vec<&str> = line.split_whitespace().collect();
	if parts.len() == 2 {
	    if parts[1].to_string() == "0" { return false; } 
	}
      }
      return true;
    }

    pub fn crt(&self) {
       println!("All modules completed! Certificate generated.");
       std::io::stdout().flush().unwrap();
    }

    pub fn load_modules(&mut self) {
        let course_directory = format!("./{}", self.course_name);

        if let Ok(entries) = fs::read_dir(&course_directory) {
            for entry in entries.filter_map(|e| e.ok()) {
                if let Ok(file_type) = entry.file_type() {
                    if file_type.is_file() {
                        let path = entry.path();
                        if let Some(file_name) = path.file_stem().and_then(|n| n.to_str()) {
                            self.modules.push(file_name.to_string());
                            self.module_binaries.push(path.display().to_string());
                        }
                    }
                }
            }

            self.module_completion = vec![false; self.modules.len()];
        } else {
            eprintln!("Directory not found: {}", course_directory);
            self.modules.clear();
            self.module_binaries.clear();
            self.module_completion = vec![];
        }
    }

    pub fn complete_module(&mut self) {
        if self.current_module_index < self.modules.len() {
            self.module_completion[self.current_module_index] = true;
        }
    }

    pub fn grab_next_module(file_path: &str) -> Result<Option<String>, Box<dyn std::error::Error>> {
        let mut file = File::open(file_path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;

        for line in content.lines() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() == 2 && parts[1] == "0" {
                return Ok(Some(parts[0].to_string()));
            }
        }

        if content.trim().is_empty() {
            Ok(None) // Indicates no modules are available
        } else {
            Ok(Some("__ALL_COMPLETE__".to_string())) // Indicates all modules are complete
        }
    }

    // Run the specified module
    pub fn run_module(&self, file_path: &str) -> Result<String, Box<dyn std::error::Error>> {
        match Self::grab_next_module(file_path)? {
            Some(module_path) => {
                //print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
                //Self::clear_console()?;
                //std::io::stdout().flush().unwrap();

                let full_path = PathBuf::from(format!("{}/{}", self.course_name, module_path));
                //println!("{}", full_path.display());
                let args: Vec<String> = env::args().collect();
                let bin_path = args.get(0).ok_or("No binary path in args")?;
                //std::io::stdout().execute(ResetColor)?;
                Self::clear_screen()?;
                disable_raw_mode()?;
                Command::new(full_path)
                    .arg(file_path)
                    .arg(bin_path)
                    .spawn()?
                    .wait()?;
                Ok(module_path)
            }
            None => Ok("__NO_MODULES__".to_string()) // No modules to run
        }
    }
   
    // For demo 
    pub fn run_single(&self, file_path: &str) -> Result<String, Box<dyn std::error::Error>> {
                let full_path = PathBuf::from(format!("{}/{}", self.course_name, file_path));
                //println!("{}", full_path.display());
                let args: Vec<String> = env::args().collect();
                let bin_path = args.get(0).ok_or("No binary path in args")?;
                //std::io::stdout().execute(ResetColor)?;
                Self::clear_screen()?;
                disable_raw_mode()?;
                Self::clear_console()?;
                Command::new(full_path)
                    .arg(file_path)
                    .arg(bin_path)
                    .spawn()?
                    .wait()?;
                Ok(file_path.to_string())
    }

    pub fn clear_console() -> Result<String, Box<dyn std::error::Error>> {
	Command::new("clear")
	    .spawn()?
	    .wait()?;
        Ok("".to_string())
    }

    fn clear_screen() -> crossterm::Result<()> {
        let mut stdout = stdout();
        stdout.execute(Clear(ClearType::All))?;
        stdout.execute(Clear(ClearType::Purge))?;
        Ok(())
    }

    pub fn get_module_idx(&self, name: &str) -> Option<usize> {
        self.modules.iter().position(|m| m == name)
    }

    pub fn resume_next_module(&mut self) {
        if self.modules.is_empty() { return; }

        if let Some(next_module_index) = self.module_completion.iter().position(|&completed| !completed) {
            self.current_module_index = next_module_index;
            let module_path = &self.module_binaries[next_module_index];
            self.run_module(module_path);
        } else {
            println!("Certificate generation not possible as no modules were found.");
        }
    }

    pub fn is_click_on_resume_button(&self, x: u16, y: u16) -> bool {
        let button_x = 10; // Example X coordinate
        let button_y = 10; // Example Y coordinate
        let button_width = 10; // Example width
        let button_height = 3; // Example height

        // Check if the click is within the button's area
        x >= button_x && x < button_x + button_width &&
        y >= button_y && y < button_y + button_height
    }


    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn next_module(&mut self) {
        if self.current_module_index < self.modules.len() - 1 {
            self.current_module_index += 1;
        }
    }

    pub fn all_modules_completed(&self) -> bool {
        self.current_module_index >= self.modules.len() &&
        self.modules.len() > 0
    }

    pub fn launch_current_module(&mut self) -> AppResult<()> {
        if let Some(path) = self.module_binaries.get(self.current_module_index) {
            let status = Command::new(path).status()?;
            if status.success() {
                self.complete_module();
            } else {
                eprintln!("Module failed to complete successfully.");
            }
        }
        Ok(())
    }
}
