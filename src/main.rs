use std::io;
use std::fmt;
use sqlx;


//**Define ENUM for SQL Server Options
#[derive(Clone)]
enum SqlType {
    POSTGRES,
    MARIADB,
    SQLITE,
    MYSQL
}

//**Define dispaly method for SqlType Enum */
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
         AnswerOption::YES => write!(f, "Yes"),
         AnswerOption::NO => write!(f, "No"),
        }
    }
}


#[derive(Clone)]
enum AuthType{
    AUTHTYPE {name:String}
}




// ** define_auth_connection_type returns valid connection string based on auth type and server type.
// ** ensures connection string is formatted for username/password or ssh cert connection type
// ** returns String
//TODO: 

fn define_auth_connection_type(auth_type: i8, server_type: SqlType) -> String {

    match server_type{
        SqlType::POSTGRES => {
            if auth_type == 1{
                return String::from("postgres")
            } else{
                return String::from("postgres")
            }
        }

        _ =>{
            return String::from("F")
        }

    }
}



//define a regex for hostname
//TODO: Define a regex fn to valiadate hostname, i.e. is this IPV4/6, or valid DNS, or local host

// ** connection_prompts creates the connection string based on sql server type. 

fn connection_prompts(server_type:SqlType){
    // Get the user input for the following: hostname, username,password,ssh key,port
    // ssh key, password are optional depending on authentication method for db connection.

    // Hostname
    println!("Enter Host Name");
    let mut std_host_input = String::new();
    io::stdin().read_line(&mut std_host_input);

    println!("Enter Username");
    let mut std_username_input = String::new();
    io::stdin().read_line(&mut std_username_input);
    println!("Enter 1 for username/password authentication or 2 for ssh authentciation(ensure you can access your key pair files)");


    let mut std_auth_type_input = String::new();
    io::stdin().read_line(&mut std_auth_type_input);
    let auth_type : usize = std_auth_type_input.trim().parse().unwrap_or(0);



    match auth_type {
        1 => {
            println!("username/password");
        }

        2 => {
            println!("ssh");
        }
        _ => {
            println!("Invalid Selection, please select from the options below");

        }
    }


}


// define function to take host,username,password,or ssh certificate
fn define_an_connect_endpoint(server_type: SqlType){
    match server_type{
        SqlType::POSTGRES => {
            connection_prompts(server_type)
        }
        SqlType::SQLITE => {
            connection_prompts(server_type)
        }
        SqlType::MARIADB => {
            connection_prompts(server_type)
        }
        SqlType::MYSQL => {
            connection_prompts(server_type)
        }
    }
}

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
            println!("Invalid Selection, please select from the options below");
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
