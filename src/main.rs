



use std::{
    env,
    process::{
        Command,
        Child,
        Stdio},
    
    path::Path,
    

};

extern crate rustyline;

use rustyline::{history::FileHistory, error::ReadlineError};
use rustyline::Editor;





fn run_command(input:&String)->bool{
    
        //split the input into commands 
        let mut commands=input.trim().split(" | ").peekable();
        let mut previous_command:Option<Child>=None;
    
        while let Some(command)=commands.next(){
        
        //split the command and its arguments at whitespaces
        let mut parts=command.trim().split_whitespace();
        //advance iterator to get the command
        let command=parts.next().unwrap();
        //set the args iterator to the parts iterator after command is consumed
        let args=parts;
        
        match command{
            "cd"=>{
                let new_dir=args.peekable().peek().map_or("/", |x| *x);
                let root=Path::new(new_dir);
                if let Err(e)=env::set_current_dir(&root){
                    println!("{}",e);
                }
            },
            "exit" => return true,
            command=>{
                //let standard input be the terminal if the previous command is None
                //if not null, then let it be the previous command
                let stdin = previous_command
                .map_or(
                    Stdio::inherit(),
                    |output: Child| Stdio::from(output.stdout.unwrap())
                );
    
                let stdout = if commands.peek().is_some() {
                    // there is another command piped behind this one
                    // prepare to send output to the next command
                    Stdio::piped()
                } else {
                    // there are no more commands piped behind this one
                    // send output to shell stdout
                    Stdio::inherit()
                };
    
    
                let output = Command::new(command)
                            .args(args)
                            .stdin(stdin)
                            .stdout(stdout)
                            .spawn();
    
                            match output {
                                Ok(output) => { previous_command = Some(output); },
                                Err(e) => {
                                    previous_command = None;
                                    eprintln!("{}", e);
                                },
                            }
    
           }
    
        }
    }  
        if let Some(mut final_command)=previous_command{
            final_command.wait().unwrap();
        }
        false
    }

   
fn main(){
    let mut rl = Editor::<(),FileHistory>::new().unwrap();
   
    loop{
        let input=rl.readline("rushell>> ");

        match input{
            Ok(command)=>{
                let exit_called=run_command(&command);
                rl.add_history_entry(&command).unwrap();
                if exit_called{
                    break;
                }
            },
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break
            },
            Err(e)=>{println!("{}",e);}
        }
        
    }
    
}
