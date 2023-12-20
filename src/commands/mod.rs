
pub mod scheduler;
use crate::Data;
use crate::Error;





pub use scheduler::*;

pub fn get() -> Vec<poise::Command<Data, Error>> {
    let commands = vec![
        scheduler::scheduler_setup(),
        // Add more commands as needed
    ];


        commands
}


