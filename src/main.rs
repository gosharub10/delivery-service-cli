use std::io::{self, Write};

use bll::user_buisness_logic::UserBuisnessLogic;
use dal::psql_repository::user_repository::DataAccess;
use dotenv::dotenv;

mod dal;
mod bll;
fn main(){
    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let data_access = DataAccess::new(&database_url).unwrap();
    let logic = UserBuisnessLogic::new(data_access);

    loop {
        println!("Введите команду (add, list, exit):");
        let mut command = String::new();
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut command).unwrap();
        let command = command.trim();

        match command {
            "add" => {
                println!("Введите имя: ");
                let mut name = String::new();
                io::stdin().read_line(&mut name).unwrap();

                println!("Введите имя: ");
                let mut name = String::new();
                io::stdin().read_line(&mut name).unwrap();

                logic.register_user();
            }
            "list" => {
                let items = logic.get_all_items()?;
                for item in items {
                    println!("ID: {}, Name: {}", item.id, item.name);
                }
            }
            "exit" => break,
            _ => println!("Неизвестная команда!"),
        }
    }
    Ok(())
}
