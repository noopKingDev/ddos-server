use crate::utils::terminal::prompt;

use crate::TcpStream;

static USER_NAME_VALID: &str = "dudu";
static USER_PASS_VALID: &str = "dudu";

#[derive(Debug)]
pub struct User {
    pub name: String,
    pass: String
}

pub async fn auth_user(mut socket: &mut TcpStream) -> Result<User,String> {


    let user_name = match prompt("user: ", &mut socket).await {
        Ok(res) => res,
        Err(_e) => return Err("Dados recebidos invalidos ".to_string()),
    };

    let user_pass = match prompt("pass: ", &mut socket).await {
        Ok(res) => res,
        Err(_e) => return Err("Dados recebidos invalidos ".to_string()),
    };


    if user_name.trim() == USER_NAME_VALID && user_pass.trim() == USER_PASS_VALID {
        println!("auth completed with success, user auth is {user_name}");

        let data_user: User = User {
            name: user_name.trim().to_string(),
            pass: user_pass.trim().to_string()
        };
        return Ok(data_user);
    }

    Err("Autenticação falhou!".to_string())
}