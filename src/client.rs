pub use crate::{dto::*, models::*};
use crate::{
    errs::{new_api_error, Result},
    utils::http::{do_http, PostParameters},
};
use async_trait::async_trait;
use reqwest::Method;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::{json, Value};
use std::ops::Add;

const BASE_URL: &str = "https://api.exmail.qq.com";

#[derive(Debug)]
pub struct Client {
    pub(crate) corp_id: String,
    pub(crate) corp_secret: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct Token {
    access_token: String,
    expires_in: u32,
}

impl Client {
    pub fn new(corp_id: String, corp_secret: String) -> Client {
        Client {
            corp_id,
            corp_secret,
        }
    }

    async fn access_token(&self) -> Result<String> {
        let query_body = json!({
            "corpid": self.corp_id,
            "corpsecret": self.corp_secret,
        });

        let resp = do_http(
            Method::GET,
            &format!("{}/cgi-bin/gettoken", BASE_URL),
            None,
            Some(query_body),
            None,
        )
        .await?;

        let data = resp.json::<Token>().await?;

        Ok(data.access_token)
    }

    // http 请求
    async fn request<R: Responser + DeserializeOwned>(
        &self,
        method: Method,
        url: &str,
        body: Option<Value>,
    ) -> Result<R> {
        let body = if let Some(data) = body {
            Some(PostParameters::json(data))
        } else {
            None
        };
        let resp = do_http(method, url, None, None, body)
            .await?
            .json::<R>()
            // .text()
            .await?;

        if resp.error_code() != 0 {
            return Err(new_api_error(resp.error_code(), resp.error_message()));
        }

        // println!("{resp}");

        Ok(resp)
        // Ok(Response::default())
    }
}

#[async_trait]
pub trait Exmailer {
    /// 创建部门，成功返回创建后的部门ID
    async fn create_department(&self, params: ParamsCreateDepartment) -> Result<u64>;
    /// 更新部门
    async fn update_department(&self, params: ParamsUpdateDepartment) -> Result<()>;
    /// 删除部门
    async fn delete_department(&self, id: u64) -> Result<()>;
    /// 获取部门列表， ID为1时获取根部门下的子部门
    async fn list_department(&self, id: Option<u64>) -> Result<Vec<Department>>;
    /// 查找部门
    async fn search_department(&self, params: ParamsSerchDepartment) -> Result<Vec<Department>>;
    /// 创建用户
    async fn create_user(&self, params: ParamsCreateUser) -> Result<()>;
    /// 更新用户
    async fn update_user(&self, params: ParamsUpdateUser) -> Result<()>;
    /// 删除用户
    async fn delete_user(&self, user_id: &str) -> Result<()>;
    /// 获取用户
    async fn get_user(&self, user_id: &str) -> Result<User>;
    /// 获取部门用户
    async fn get_department_user(
        &self,
        department_id: u64,
        fetch_child: Option<bool>,
    ) -> Result<Vec<User>>;
    /// 批量检查账户
    async fn batchcheck_user(&self, userids: &[&str]) -> Result<Vec<UserCheck>>;
    /// 创建群组
    async fn create_group(&self, params: ParamsCreateGroup) -> Result<()>;
    /// 更新群组
    async fn update_group(&self, params: ParamsUpdateGroup) -> Result<()>;
    /// 删除群组
    async fn delete_group(&self, group_id: &str) -> Result<()>;
    /// 获取群组信息
    async fn get_group(&self, group_id: &str) -> Result<Group>;
}

trait Responser {
    fn error_code(&self) -> u64;
    fn error_message(&self) -> String;
}

#[derive(Debug, Deserialize, Serialize, Default)]
struct Response {
    #[serde(rename = "errcode")]
    error_code: u64,
    #[serde(rename = "errmsg")]
    error_message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    department: Option<Vec<Department>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<u64>,
    #[serde(rename = "userlist", skip_serializing_if = "Option::is_none")]
    user_list: Option<Vec<User>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    list: Option<Vec<UserCheck>>,
}

impl Responser for Response {
    fn error_code(&self) -> u64 {
        self.error_code
    }
    fn error_message(&self) -> String {
        self.error_message.to_owned()
    }
}

#[derive(Debug, Deserialize, Serialize, Default)]
struct GetResponse<T> {
    #[serde(rename = "errcode")]
    error_code: u64,
    #[serde(rename = "errmsg")]
    error_message: String,
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    data: Option<T>,
}

impl<T> Responser for GetResponse<T> {
    fn error_code(&self) -> u64 {
        self.error_code
    }
    fn error_message(&self) -> String {
        self.error_message.to_owned()
    }
}

#[async_trait]
impl Exmailer for Client {
    /// 参考接口说明：https://service.rtxmail.net/api/267.html
    async fn create_department(&self, params: ParamsCreateDepartment) -> Result<u64> {
        let token = self.access_token().await?;
        let resp = self
            .request::<Response>(
                Method::POST,
                &format!("{BASE_URL}/cgi-bin/department/create?access_token={token}"),
                Some(serde_json::to_value(params)?),
            )
            .await?;

        Ok(resp.id.unwrap())
    }

    /// 参考接口说明：https://service.rtxmail.net/api/268.html
    async fn update_department(&self, params: ParamsUpdateDepartment) -> Result<()> {
        let token = self.access_token().await?;
        self.request::<Response>(
            Method::POST,
            &format!("{BASE_URL}/cgi-bin/department/update?access_token={token}"),
            Some(serde_json::to_value(params)?),
        )
        .await?;

        Ok(())
    }

    /// 参考接口说明：https://service.rtxmail.net/api/269.html
    async fn delete_department(&self, id: u64) -> Result<()> {
        let token = self.access_token().await?;
        self.request::<Response>(
            Method::GET,
            &format!("{BASE_URL}/cgi-bin/department/delete?access_token={token}&id={id}"),
            None,
        )
        .await?;

        Ok(())
    }

    /// 参考接口说明：https://service.rtxmail.net/api/270.html
    async fn list_department(&self, id: Option<u64>) -> Result<Vec<Department>> {
        let token = self.access_token().await?;
        let id = id.unwrap_or(1);

        let resp: Response = self
            .request(
                Method::GET,
                &format!("{BASE_URL}/cgi-bin/department/list?access_token={token}&id={id}"),
                None,
            )
            .await?;

        Ok(resp.department.unwrap())
    }

    /// 参考接口说明：https://service.rtxmail.net/api/271.html
    async fn search_department(&self, params: ParamsSerchDepartment) -> Result<Vec<Department>> {
        let token = self.access_token().await?;
        let resp: Response = self
            .request(
                Method::POST,
                &format!("{BASE_URL}/cgi-bin/department/search?access_token={token}"),
                Some(serde_json::to_value(&params)?),
            )
            .await?;

        Ok(resp.department.unwrap())
    }

    /// 参考接口说明：https://service.rtxmail.net/api/272.html
    async fn create_user(&self, params: ParamsCreateUser) -> Result<()> {
        let token = self.access_token().await?;
        self.request::<Response>(
            Method::POST,
            &format!("{BASE_URL}/cgi-bin/user/create?access_token={token}"),
            Some(serde_json::to_value(params)?),
        )
        .await?;

        Ok(())
    }

    /// 参考接口说明：https://service.rtxmail.net/api/273.html
    async fn update_user(&self, params: ParamsUpdateUser) -> Result<()> {
        let token = self.access_token().await?;
        self.request::<Response>(
            Method::POST,
            &format!("{BASE_URL}/cgi-bin/user/update?access_token={token}"),
            Some(serde_json::to_value(params)?),
        )
        .await?;

        Ok(())
    }

    /// 参考接口说明：https://service.rtxmail.net/api/274.html
    async fn delete_user(&self, user_id: &str) -> Result<()> {
        let token = self.access_token().await?;
        self.request::<Response>(
            Method::GET,
            &format!("{BASE_URL}/cgi-bin/user/delete?access_token={token}&userid={user_id}"),
            None,
        )
        .await?;

        Ok(())
    }

    /// 参考接口说明：https://service.rtxmail.net/api/275.html
    async fn get_user(&self, user_id: &str) -> Result<User> {
        let token = self.access_token().await?;
        let resp = self
            .request::<GetResponse<User>>(
                Method::GET,
                &format!("{BASE_URL}/cgi-bin/user/get?access_token={token}&userid={user_id}"),
                None,
            )
            .await?;

        Ok(resp.data.unwrap())
    }

    /// 参考接口说明：https://service.rtxmail.net/api/277.html
    async fn get_department_user(
        &self,
        department_id: u64,
        fetch_child: Option<bool>,
    ) -> Result<Vec<User>> {
        let token = self.access_token().await?;
        let fetch_child = fetch_child
            .map(|x| if x { 1 } else { 0 })
            .unwrap_or_default();
        let resp = self
            .request::<Response>(
                Method::GET,
                &format!("{BASE_URL}/cgi-bin/user/list?access_token={token}&department_id={department_id}&fetch_child={fetch_child}"),
                None,
            )
            .await?;

        Ok(resp.user_list.unwrap())
    }

    /// 参考接口说明：https://service.rtxmail.net/api/278.html
    async fn batchcheck_user(&self, userids: &[&str]) -> Result<Vec<UserCheck>> {
        let token = self.access_token().await?;
        let resp = self
            .request::<Response>(
                Method::POST,
                &format!("{BASE_URL}/cgi-bin/user/batchcheck?access_token={token}"),
                Some(serde_json::json!({
                    "userlist": userids,
                })),
            )
            .await?;
        Ok(resp.list.unwrap())
    }

    /// 参考接口说明：https://service.rtxmail.net/api/279.html
    async fn create_group(&self, params: ParamsCreateGroup) -> Result<()> {
        let token = self.access_token().await?;
        self.request::<Response>(
            Method::POST,
            &format!("{BASE_URL}/cgi-bin/group/create?access_token={token}"),
            Some(serde_json::to_value(params)?),
        )
        .await?;

        Ok(())
    }

    /// 参考接口说明：https://service.rtxmail.net/api/280.html
    async fn update_group(&self, params: ParamsUpdateGroup) -> Result<()> {
        let token = self.access_token().await?;
        self.request::<Response>(
            Method::POST,
            &format!("{BASE_URL}/cgi-bin/group/update?access_token={token}"),
            Some(serde_json::to_value(params)?),
        )
        .await?;

        Ok(())
    }

    /// 参考接口说明：https://service.rtxmail.net/api/281.html
    async fn delete_group(&self, group_id: &str) -> Result<()> {
        let token = self.access_token().await?;
        self.request::<Response>(
            Method::GET,
            &format!("{BASE_URL}/cgi-bin/group/delete?access_token={token}&groupid={group_id}"),
            None,
        )
        .await?;

        Ok(())
    }

    /// 参考接口说明：https://service.rtxmail.net/api/282.html
    async fn get_group(&self, group_id: &str) -> Result<Group> {
        let token = self.access_token().await?;
        let resp = self
            .request::<GetResponse<Group>>(
                Method::GET,
                &format!("{BASE_URL}/cgi-bin/group/get?access_token={token}&userid={group_id}"),
                None,
            )
            .await?;

        Ok(resp.data.unwrap())
    }
}

#[cfg(test)]
pub mod tests {

    use serde::{Deserialize, Serialize};

    #[derive(Debug, Deserialize, Serialize)]
    struct Rr {
        errcode: u64,
        errmsg: String,
        #[serde(rename = "userid")]
        user_id: String,
        name: String,
        department: Vec<u64>,
        position: String,
        mobile: String,
        gender: String,
        enable: u8,
        slaves: Vec<String>,
        cpwd_login: Option<u8>,
    }

    #[test]
    fn test() {
        let json_str = r##"{
            "errcode":0,
            "errmsg":"ok",
            "userid":"shenshouer2955@ipalfish.com",
            "name":"沈首二",
            "department":[7065571265906742219,6726112443993748595],
            "position":"",
            "mobile":"",
            "tel":"",
            "extid":"",
            "gender":"1",
            "slaves":[],
            "enable":1
        }"##;

        // let resp = serde_json::from_str::<Rr>(json_str);
        let resp = serde_json::from_str::<serde_json::Value>(json_str);
        println!("{:?}", resp);
        if let Ok(v) = resp {
            let r = serde_json::from_value::<super::GetResponse<super::User>>(v);
            println!("{:?}", r);
        }
    }
}
