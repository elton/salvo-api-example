// API handlers
use salvo::prelude::*;

use crate::schema::User;

#[handler]
pub async fn index() -> Result<&'static str, ()> {
    Ok("The system is up")
}

#[handler]
pub async fn get_params(req: &mut Request) -> Result<String, ()> {
    // 可以是以哦那个req.param::<T>("name")来获取参数 或者 req.params().get("name")来获取url中名为name的参数。以api/name/<name>为例
    // let name = req.param::<&str>("name").unwrap();
    let name = req.params().get("name").unwrap();
    Ok(format!("your name is: {}", name))
}

#[handler]
pub async fn get_query(req: &mut Request) -> Result<String, ()> {
    // 可以是以req.query::<T>("title")来获取参数 或者 req.queries().get("title")来获取url中名为title的参数。以api/title?title=xxx为例
    let title = req.query::<String>("title").unwrap();
    Ok(format!("your title is: {}", title))
}

#[handler]
pub async fn create_user(req: &mut Request) -> Result<&'static str, ()> {
    let user = req.parse_json::<User>().await.unwrap();
    tracing::debug!("create user: {:?}", user);
    Ok("create user")
}