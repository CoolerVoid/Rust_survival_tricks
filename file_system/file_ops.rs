use std::{
    fs::{File, OpenOptions},
    io::{self, BufRead, BufReader, Write},
    path::Path
};

fn write_log(filename: String, input: String) -> io::Result<()> {

   if Path::new(&filename).exists() == false {
   	 File::create(&filename).expect("Unable to create file");
   }

   let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(filename)
        .unwrap();

    if let Err(e) = writeln!(&mut file, "{}",input) {
	return Err(e)
    }
 
    Ok(())
}

fn lines_to_vec(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
    BufReader::new(File::open(filename)?).lines().collect()
}


fn read_lines(filename: impl AsRef<Path>) -> io::Result<()> {
    // Open the file in read-only mode (ignoring errors).
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    // Read the file line by line using the lines() iterator from std::io::BufRead.
    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap(); 
        println!("{}. {}", index + 1, line);
    }

    Ok(())
}



fn main() {
// read each line and save in vector
 let lines = lines_to_vec("test.txt").expect("Could not load lines");

  for line in lines {
   println!("{:?}", line);
  }

// Write log example
 write_log("log.txt".to_string(),"kadabra!\n".to_string()).expect("Could not write in log file");

// rename file
 std::fs::rename("log.txt", "log2.txt").expect("Could not rename");

// read lines
 let _test_call_line = read_lines("log.txt").expect("Error on read file");

 std::fs::copy("log2.txt", "log1.txt").expect("Count not copy the file."); 

// remove a file
 std::fs::remove_file("log2.txt").expect("Could not remove logs");
 std::fs::remove_file("log1.txt").expect("Could not remove logs");


// create a directory
 std::fs::create_dir("test").expect("Could not create a directory");
// remove a directory
 std::fs::remove_dir("test").expect("Could not delete a directory");

}
