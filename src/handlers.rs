// API handlers
use salvo::prelude::*;

use crate::schema::resp::ResponseData;
use crate::schema::user::User;

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

    // 不处理错误，会导致panic，但是整个应用不会崩溃。会返回ERR_EMPTY_RESPONSE。其他正常的请求还是可以正常返回的。
}

#[handler]
pub async fn create_user(req: &mut Request, res: &mut Response) {
    // 将POST请求的body转换为User结构体
    // 使用`？`,将表达式转换成Result类型，如果是Err，则直接返回错误，如果是Ok，则得到计算
    // let user: User = req.parse_json().await?;
    let user = req.parse_body::<User>().await.unwrap();
    tracing::info!("create user: {:?}", user);

    // 包装返回数据
    let resp = ResponseData {
        success: true,
        message: "create user success".to_string(),
        error_code: None,
        data: Some(user),
    };

    res.status_code(StatusCode::CREATED).render(Json(resp));
}
