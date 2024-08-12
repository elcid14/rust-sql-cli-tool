use std::io;
use std::fmt;


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
enum AuthType{
    AUTHTYPE {name:String}
}


// ** Utility function to get filepath for certs if needed.
// fn get_cert_files (auth_type: usize, filepaths:Vec<String>) -> Vec<String>{

// }




// ** Utility function to get password

fn get_password(auth_type: Option<usize>) -> Option<String> {
    match auth_type {
        Some(1) => {
            println!("Enter password");
            let mut std_password_input = String::new();
            io::stdin().read_line(&mut std_password_input).expect("Failed to read passowrd");
            Some(std_password_input.trim().to_string())

    }

    _ => {
        return None
    }
}
}


// ** define_auth_connection_type returns valid connection string based on auth type and server type.
// ** ensures connection string is formatted for username/password or ssh cert connection type
// ** returns String

fn define_auth_connection_type(auth_type: Option<usize>, server_type: Option<SqlType>, username:String, host:String, database:String, port:String) -> String {

        let mut password: String = String::from("");
        let mut ssl_cert_path: String = String::from("");
        let mut ssl_key_path:String = String::from("");
        let mut ssl_root_cert_path: String = String::from("");

        if auth_type == Some(1) {
            password = get_password(auth_type).expect("Not valid password string");
            println!("Password entered succesfully");

        } else {
            println!("Using certificate based auth instead of username/passowrd");
            println!("Please enter the following ");
        }

    match (server_type, auth_type){
        (Some(SqlType::POSTGRES), Some(auth_type)) => {
            if auth_type == 1{
                return String::from(format!("postgres://{username}:{password}@{host}:{port}/{database}"))
            } else {
                return String::from(format!("postgres://{username}:{password}@{host}:{port}/{database}?sslmode=require&sslcert={ssl_cert_path}&sslkey={ssl_key_path}&sslrootcert={ssl_root_cert_path}"))
            }
        }

        (Some(SqlType::MARIADB), Some(auth_type)) => {
            if auth_type == 1{
                return String::from("MariaDb")
            } else{
                return String::from("MariaDb")
            }
        }

        (Some(SqlType::MYSQL), Some(auth_type)) => {
            if auth_type == 1{
                return String::from("MariaDb")
            } else{
                return String::from("MariaDb")
            }
        }

        (Some(SqlType::SQLITE), Some(auth_type)) => {
            if auth_type == 1{
                return String::from("MariaDb")
            } else{
                return String::from("MariaDb")
            }
        }

        _ =>{
            return String::from("F")
        }

    }
}


// ** define helper function that returns number of connection pools

fn number_of_connection_pools(connection_option:&String) -> usize{

    if connection_option == "y" {
        println!("Enter number of connections between 1 and 10");
        let mut std_connection_number_input = String::new();
        io::stdin().read_line(&mut std_connection_number_input).expect("Failed to read line");
        let number_connections: usize = std_connection_number_input.trim().parse().expect("Please enter a valid number");
        return number_connections;
    
    } else {
        return 1;
    }
}


//define a regex for hostname
//TODO: Define a regex fn to valiadate hostname, i.e. is this IPV4/6, or valid DNS, or local host

// ** connection_prompts creates the connection string based on sql server type. 

fn connection_prompts(server_type:SqlType) -> String{
    // Get the user input for the following: hostname, username,password,ssh key,port
    // ssh key, password are optional depending on authentication method for db connection.

    // Hostname
    println!("Enter Host Name");
    let mut std_host_input = String::new();
    let _ = io::stdin().read_line(&mut std_host_input);
    // Username
    println!("Enter Username");
    let mut std_username_input = String::new();
    let _ = io::stdin().read_line(&mut std_username_input);

    //Database
    println!("Enter name of database you wish to connect to");
    let mut std_database_input: String = String::new();
    let _ = io::stdin().read_line(&mut std_database_input);

    //Port
    println!("Enter the port to connect to your endpoint");
    let mut std_port_input:String = String::new();
    let _ = io::stdin().read_line( &mut std_port_input);

    //Connection pools

    println!("Do you want to poool your connections?(y/n)");
    let mut std_connection_answer_input: String = String::new();
    let _ = io::stdin().read_line(&mut std_connection_answer_input);


    //Auth Type
    println!("Enter 1 for username/password authentication or 2 for ssh authentciation(ensure you can access your key pair files)");
    let mut std_auth_type_input: String = String::new();
    let _ = io::stdin().read_line(&mut std_auth_type_input);
    let auth_type : usize = std_auth_type_input.trim().parse().unwrap_or(0);

    let number_connections: usize = number_of_connection_pools(&std_connection_answer_input);


    let connection_string: String =  define_auth_connection_type(Some(auth_type), Some(server_type), std_host_input, std_username_input, std_database_input, std_port_input).replace("\n", "");

    println!("{:?}", connection_string);

    return connection_string

    }


// define function to take host,username,password,or ssh certificate
fn define_an_connect_endpoint(server_type: SqlType) -> String{
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
    let endpoint_options: [SqlType; 4] = [SqlType::POSTGRES, SqlType::SQLITE, SqlType::MARIADB, SqlType::MYSQL];
    println!("Select sql server option");
    for (i, option) in endpoint_options.iter().enumerate(){
        println!("{:}{}{}", i + 1, ":", option.to_string());
    }

    // Get user input selection
    let mut std_input: String = String::new();
    let _ = io::stdin().read_line(&mut std_input);
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
    let server: SqlType  =  select_sql_server_type();
    define_an_connect_endpoint(server);

    Ok(())
}
