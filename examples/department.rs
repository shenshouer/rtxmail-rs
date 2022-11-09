use anyhow::Result;
use dotenv::dotenv;
use rtxmail::{client::Exmailer, Client};
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let corp_id = env::var("CORP_ID")?;
    let corp_secret = env::var("CORP_SECRET")?;
    let c = Client::new(
        corp_id,
        corp_secret,
        Some(tokio::time::Duration::from_millis(500)),
    );

    // 查找部门
    // let params = client::ParamsSerchDepartment {
    //     name: "技术".to_string(),
    //     fuzzy: Some(1), // 模糊匹配
    // };
    // let resp = c.search_department(params).await?;

    // 获取部门列表
    let resp = c.list_department(None).await?;
    println!("{}", serde_json::to_string(&resp)?);

    // 创建部门
    // let params = client::ParamsCreateDepartment {
    //     name: "这是一个测试部门".to_string(),
    //     parent_id: 1,
    //     order: None,
    // };

    // let id = c.create_department(params).await?;
    // println!("新部门ID: {}", id);

    // c.delete_department(5669736856398649979).await?;

    // let params = client::ParamsUpdateDepartment {
    //     id: 0,
    //     name: None,
    //     parent_id: None,
    //     order: None,
    // };
    // println!("{}", serde_json::to_string(&params)?);

    // 根据uid获取用户
    // let resp = c.get_user("shenshouer2955@xxx.com").await?;

    // 根据部门id获取用户
    // let resp = c.get_department_user(1, Some(true)).await?;

    // 检查邮箱
    // let resp = c
    //     .batchcheck_user(&[
    //         "shenshouer2955@xxx.com",
    //         "songshenxiang6296@xxx.com",
    //         "test@xxx.com",
    //     ])
    //     .await?;

    // let resp = c.get_group("devops@xxx.com").await?;
    // println!("{}", serde_json::to_string(&resp)?);

    Ok(())
}
