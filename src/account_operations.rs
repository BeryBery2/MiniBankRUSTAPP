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

fn clear_screen()
{
    print!("\x1B[2J\x1B[1;1H"); //This clears the console
    clean_buffer();
}

pub async fn signin() -> Result<(), sqlx::Error>
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

pub async fn login() -> Result<(), sqlx::Error>
{
    let mut typed_user = String::new();
    let mut typed_password = String::new();

    //Conecting to database
    let address = env::var("DATABASE_URL").expect("Database URL ERROR");
    let connection = MySqlPool::connect(&address).await?;

    loop 
    {
        print!("Username: ");
        clean_buffer();
        io::stdin().read_line(&mut typed_user).expect("An Error as Ocurred, please try again");

        print!("Password: ");
        clean_buffer();
        io::stdin().read_line(&mut typed_password).expect("An Error as Ocurred, please try again");
        

        let stored_password: Option<String> = sqlx::query_scalar(
            "SELECT password FROM user WHERE username = ?"
        ).bind(&typed_user)
        .fetch_optional(&connection)
        .await?;

        match stored_password
        {
            None => {
                clear_screen();
                println!("Username not found, Try again.\n");
                typed_user.clear();
                typed_password.clear();
                continue;
            }
            Some(stored_password) => {
                let decrypted_password = secure_data::decrypt(&stored_password);

                //Verifying passwords
                if decrypted_password == typed_password
                {
                    //Loged in
                    break;
                }
                else 
                {
                    clear_screen();
                    println!("Incorrect password.\n");
                    typed_user.clear();
                    typed_password.clear();
                    continue;
                }
            }
        }
    }

    Ok(())
}