use anyhow::Result;
use dotenv::dotenv;
use rtxmail::{client::Exmailer, Client};
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let corp_id = env::var("CORP_ID")?;
    let corp_secret = env::var("CORP_SECRET")?;
    let c = Client::new(corp_id, corp_secret);

    // 根据uid获取用户
    let resp = c.get_user("shenshouer2955@xxx.com").await;

    match resp {
        Err(err) => {
            let err_msg = format!("{}", err);
            if err_msg.contains("userid not found") {
                println!("未找到用户",)
            }
        }
        Ok(user) => println!("{}", serde_json::to_string(&user)?),
    }

    // 根据部门id获取用户
    // let resp = c.get_department_user(1, Some(true)).await?;

    // println!("{}", serde_json::to_string(&resp)?);

    Ok(())
}
