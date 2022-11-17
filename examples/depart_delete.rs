use anyhow::Result;
use dotenv::dotenv;
use rtxmail::{
    client::{self, Exmailer},
    models::Department,
    Client,
};
use std::env;
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let corp_id = env::var("CORP_ID")?;
    let corp_secret = env::var("CORP_SECRET")?;
    let c = Client::new(
        corp_id,
        corp_secret,
        Some(tokio::time::Duration::from_millis(500)),
    );

    // let delete_depart_id = 7602344046493987977;
    // let users = c.get_department_user(delete_depart_id, Some(true)).await?;
    // info!("users: {}", serde_json::to_string(&users)?);

    // if users.is_empty() {
    //     c.delete_department(delete_depart_id).await?;
    // }

    // 查找部门
    let params = client::ParamsSerchDepartment {
        name: "1585760".to_string(),
        fuzzy: Some(1), // 模糊匹配
    };
    let resp = c.search_department(params).await?;
    info!("{}", serde_json::to_string(&resp)?);
    if resp.len() == 1 {
        let email_depart = resp[0].to_owned();
        delete_department(&c, email_depart, true, true).await?;
    }

    // for depart in resp {
    //     // if depart.name == "海外华人销转中心_2256571" {
    //     delete_department(&c, depart, true, true).await?;
    //     // }
    // }

    Ok(())
}

/// 循环删除当前部门数据与所有子级部门数据
#[async_recursion::async_recursion]
async fn delete_department(
    c: &Client,
    d: Department,
    delete_child_department: bool,
    delete_root: bool,
) -> Result<()> {
    if !delete_child_department {
        return Ok(c.delete_department(d.id).await?);
    }
    info!("处理企业邮箱部门数据: [Id: {}, Name: {}]", d.id, d.name);
    let departs = c.list_department(Some(d.id)).await?;
    for depart in departs {
        if depart.id != d.id {
            delete_department(c, depart, delete_child_department, true).await?;
        }
    }
    if !delete_root {
        return Ok(());
    }
    info!("删除企业邮箱部门数据: [Id: {}, Name: {}]", d.id, d.name);
    let users = c.get_department_user(d.id, Some(true)).await?;
    info!("users: {}", serde_json::to_string(&users)?);
    // Ok(c.delete_department(d.id).await?)
    Ok(())
}
