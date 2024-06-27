pub trait Command {
    // name: String;
    // fn new() -> Self;

    // fn find(&self, args: Vec<String>) -> Box<dyn Command>;

    fn execute(&self, args: Vec<String>) -> Result<(), ()> {
        let command = &args[0];
        let args = &args[1..];

        println!("Current command: {}", command);

        // for subcommand in &self.subcommands {
        //     println!("name: {}", subcommand.name);
        //     if subcommand.name == *command || subcommand.shortname == *command {
        //         // subcommand.execute(args.to_vec())
        //         println!("name: {}", subcommand.name);
        //         return Ok(())
        //     }
        // }

        Err(())
    }
}


pub struct Secrets {
    name: String,
    // shortname: vec![String],
    args: Vec<String>,
    description: String,
    usage: String,
    // subcommands: Vec<Command>,
}

// impl Command for Secrets {

    // fn find(&self, args: Vec<String>) -> Box<dyn Command> {
    //     // return Box::new(self.clone());
    // }

    // fn new() -> Command {
    //     Command {
    //         name: "secrets".to_string(),
    //         shortname: vec!["s".to_string()],
    //         args: vec![],
    //         description: "Show help".to_string(),
    //         usage: "secrets <command>".to_string(),
    //         subcommands: vec![],
    //     }
    // }
// }