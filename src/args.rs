pub struct Args {
    pub in_file_path: String,
    pub out_file_path: String,
}

impl Args {
    pub fn get(mut args: impl Iterator<Item = String>) -> Result<Args, &'static str> {
        args.next();

        let in_file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get an input file path"),
        };

        let out_file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get an output file path")
        };

        Ok(Args { in_file_path, out_file_path })
    }
}
