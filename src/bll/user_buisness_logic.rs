use postgres::Error;

use crate::dal::{entites::user::User, interfaces::data_access_trat::DataAccessTrait};

pub struct UserBuisnessLogic <T: DataAccessTrait<User>>{
    data_access: T
}

impl <T :DataAccessTrait<User>> UserBuisnessLogic<T> {
    pub fn new(data_access: T) -> Self{
        UserBuisnessLogic{data_access}
    }

    pub fn register_user(&mut self, user: User) -> Result<(), std::io::Error>{
        match self.data_access.create(user.clone()) {
            Ok(_) => {
                println!("Registration succesful, {}", &user.name);
                Ok(())},
                Err(err) => {
                    eprintln!("Registration failed: {}", err);
                    Err(std::io::Error::new(std::io::ErrorKind::Other, "Failed to register user"))
                }
        }
    }

    pub fn get_all_users(&mut self) -> Result<Vec<User>, std::io::Error>{
        match self.data_access.get_all() {
            Ok(users) => Ok(users),
            Err(err) => {
                eprintln!("Can't get users ... : {}", err);
                Err(std::io::Error::new(std::io::ErrorKind::Other, "Failed to get users"))
            }
        }
    }
}