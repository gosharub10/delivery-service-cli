use postgres::{Client, NoTls};

use crate::dal::{entites::user:: User, interfaces::data_access_trat::DataAccessTrait};
pub struct DataAccess{
    pool: Client
}

impl DataAccess {
    pub fn new(database_url: &str) -> Result<Self, postgres::Error>{
        let pool = Client::connect(&database_url, NoTls).expect("Set DATABASE_URL... !");
        
        Ok(DataAccess{pool})
    }
}

impl DataAccessTrait<User> for DataAccess {
    fn create(&mut self,entity: User) -> Result<(), postgres::Error> {
        let _ = &self.pool.query("INSERT INTO users (user_name, user_email, user_password, user_phone, user_address) VALUES ($1,$2,$3,$4,$5)"
        , &[&entity.name, &entity.email, &entity.password, &entity.phone, &entity.address])?;
        Ok(())
    }

    fn get_all(&mut self) -> Result<Vec<User>, String> {
        let mut users: Vec<User> = Vec::new();
        for row in &self.pool.query("SELECT * FROM users", &[]).unwrap() {
            let id: i32 = row.get(0);
            let name: String = row.get(1);
            let email: String = row.get(2);
            let password: String = row.get(3);
            let phone: String = row.get(4);
            let address: String = row.get(5);

            users.push(User{id, name, email,password, phone, address});
        }

        if !users.is_empty() {
            Ok(users)
        } else {
            Err(format!("Can't get all users"))
        }
    }
}