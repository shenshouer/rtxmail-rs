use anyhow::Result;
use dotenv::dotenv;
use rtxmail::{
    client::{self, Exmailer},
    Client,
};
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

    let ids = vec![
        855947, 855947, 855947, 855947, 2053326, 2172871, 2172873, 2172874, 1944523, 2026591,
        2029210, 2029211, 2105895, 2026592, 2238884, 2246687, 2114108, 2053332, 2026593, 2053327,
        2053330, 2238070, 1896868, 1896869, 1896933, 1966494, 2172867, 1966496, 2256571, 2210693,
        2210691, 2210692, 1974687, 2052118, 1808068, 2034549, 1896322,
    ];

    let mut ds = vec![];
    for id in ids {
        // 查找部门
        let params = client::ParamsSerchDepartment {
            name: format!("{id}"),
            fuzzy: Some(1), // 模糊匹配
        };
        let resp = c.search_department(params).await?;
        // println!("{}", serde_json::to_string(&resp)?);
        ds.push(resp);
    }
    println!("{}", serde_json::to_string(&ds)?);

    // // 查找部门
    // let params = client::ParamsSerchDepartment {
    //     name: "技术".to_string(),
    //     fuzzy: Some(1), // 模糊匹配
    // };
    // let resp = c.search_department(params).await?;
    // println!("{}", serde_json::to_string(&resp)?);

    Ok(())
}
