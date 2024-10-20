use std::io::{self, Write};

use bll::user_buisness_logic::UserBuisnessLogic;
use dal::{entites::user::User, psql_repository::user_repository::DataAccess};
use dotenv::dotenv;
use prettytable::{Table, row};

mod dal;
mod bll;
fn main(){
    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let data_access = DataAccess::new(&database_url).unwrap();
    let mut logic = UserBuisnessLogic::new(data_access);

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

                println!("Введите email: ");
                let mut email = String::new();
                io::stdin().read_line(&mut email).unwrap();
                
                println!("Введите пароль: ");
                let mut password = String::new();
                io::stdin().read_line(&mut password).unwrap();

                println!("Введите адрес проживания: ");
                let mut address = String::new();
                io::stdin().read_line(&mut address).unwrap();

                println!("Введите номер телефона: ");
                let mut phone = String::new();
                io::stdin().read_line(&mut phone).unwrap();

                let _ = logic.register_user(User{id: 0,name, email, password, phone, address});
            }
            "list" => {
                let users = logic.get_all_users().unwrap();
                let mut table = Table::new();
                table.add_row(row!["ID", "Name", "Email", "Phone"]);
                for user in users {
                    table.add_row(row![user.id, user.name, user.email, user.phone]);
                }
                table.printstd();
            }
            "exit" => break,
            _ => println!("Неизвестная команда!"),
        }
    }
}
