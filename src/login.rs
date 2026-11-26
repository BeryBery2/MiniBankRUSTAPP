use std::io::{self, Write};
use sqlx::MySqlPool;
use std::env;
use crate::secure_data;

fn clean_buffer() 
{
    //Cleans the buffer, letting the print show and the input being asked in the same line
    //Actually I have no ideia how this works but it works
    io::stdout().flush().unwrap(); 
}

pub async fn login() -> Result<(), sqlx::Error>
{
    let mut user = String::new();
    let mut password = String::new();

    //Conecting to database
    let address = env::var("DATABASE_URL").expect("Database URL ERROR");
    let connection = MySqlPool::connect(&address).await?;

    print!("Username: ");
    clean_buffer();
    io::stdin().read_line(&mut user).expect("An Error as Ocurred, please try again");

    println!("The password need to be atleast 8 characters long!");
    loop
    {
        print!("Password: ");
        clean_buffer();
        io::stdin().read_line(&mut password).expect("An Error as Ocurred, please try again");

        if password.len() < 8
        {
            println!("The password need to be atleast 8 characters long!");
            password.clear(); //If not the new password will merge with the other one
        }
        else
        {
            password = secure_data::encrypt(&password);
            break;
        }
    }

    sqlx::query("INSERT INTO user (username, password) VALUES (?, ?)")
        .bind(user)
        .bind(password)
        .execute(&connection)
        .await?;
    Ok(())    
}