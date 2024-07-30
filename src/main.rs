use std::io;
use std::fmt;


// define ENUM for SQL Server Options
//
#[derive(Clone)]
enum SqlType {
    POSTGRES,
    MARIADB,
    SQLITE,
    MYSQL
}

impl fmt::Display for SqlType{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SqlType::POSTGRES => write!(f, "Postgres"),
            SqlType::MARIADB => write!(f, "MariaDb"),
            SqlType::SQLITE => write!(f, "SQLite"),
            SqlType::MYSQL => write!(f, "MySQL")
        }
    }
}


#[derive(Clone)]
enum AnswerOption{
    YES,
    NO,
}

impl fmt::Display for AnswerOption{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
         AnswerOption::YES => write!(f, "Yes")
         AnswerOption::NO => write!(f, "No")
        }
    }
}






//define a regex for hostname

fn connection_prompts(server_type:SqlType){
    // Get the user input for the following: hostname, username,password,ssh key,port
    // ssh key, password are optional depending on authentication method for db connection.

    // Hostname
    println!("Enter Host Name");
    let mut std_host_input = String::new();
    io::stdin().read_line(&mut std_host_input);

    println!("Enter Username");
    let mut std_username_input = String::new();
    io::stdin().read_line(&mut std_username_input)

    println("Are you using SSH or username/password for authentication?")

    let auth_option = match 


    



    

}


// define function to take host,username,password,or ssh certificate
fn define_an_connect_endpoint(server_type: SqlType){
    match server_type{
        SqlType::POSTGRES => {
            println!("TEST")
        }
        SqlType::SQLITE => {
            println!("TEST")
        }
        SqlType::MARIADB => {
            println!("TEST")
        }
        SqlType::MYSQL => {
            println!("TEST")
        }
    }
}


// define function to get keyboard interrupt option


// define function to creat progress bar when connecting

//

// define function to connect to host based on selected 
fn select_sql_server_type() -> SqlType{
    // Endpoint options for DBs currently supported by sqlx
    let endpoint_options = [SqlType::POSTGRES, SqlType::SQLITE, SqlType::MARIADB, SqlType::MYSQL];
    println!("Select sql server option");
    for (i, option) in endpoint_options.iter().enumerate(){
        println!("{:}{}{}", i + 1, ":", option.to_string());
    }

    // Get user input selection
    let mut std_input = String::new();
    io::stdin().read_line(&mut std_input);
    let user_selection : usize = std_input.trim().parse().unwrap_or(0);
    
    // match the user_selection based on value from endpoint_options, if an invalid option allow user to select again. 
      match user_selection {
        1 => {
            println!("You selected Postgres"); 
            return endpoint_options[0].clone()
        }

        2 => {
            println!("You selected SQLite");
             return endpoint_options[1].clone()
        }

        3 => {
            println!("You selected MariaDB");
             return endpoint_options[2].clone()
        }

        4 => {
            println!("You selected MySQL");
             return endpoint_options[3].clone()
        }
        _ => {
            println!("Invalid Selection, please select from the options eblow");
            return select_sql_server_type()
        }
    };
        
}

fn main() -> io::Result<()> {

    let test  =  select_sql_server_type();
    define_an_connect_endpoint(test);
    // println!("{}",test);
    Ok(())
}
