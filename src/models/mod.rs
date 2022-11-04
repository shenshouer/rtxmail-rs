use serde::{Deserialize, Serialize};

/// 部门
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Department {
    id: u64,
    name: String,
    #[serde(rename = "parentid")]
    parent_id: u64,
    order: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    path: Option<String>,
}

/// 用户
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct User {
    #[serde(rename = "userid")]
    user_id: String,
    name: String,
    department: Vec<u64>,
    position: String,
    mobile: String,
    gender: Option<String>,
    enable: u8,
    slaves: Vec<String>,
    cpwd_login: Option<u8>,
}

/// 用户检查数据
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct UserCheck {
    user: String,
    #[serde(rename = "type")]
    kind: u8,
}

/// 群组数据
/// "groupid": "zhangsangroup@gzdev.com",
/// "groupname": "zhangsangroup",
/// "userlist": ["zhangsanp@gzdev.com", "lisi@gzdev.com"],
/// "grouplist": [" group@gzdev.com "],
/// "department": [1, 2],
/// "allow_type": 3,
/// "allow_userlist": ["zhangsanp@gzdev.com"]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Group {
    /// 	邮件群组id，邮件格式
    groupid: String,
    /// 	邮件群组名称
    groupname: String,
    /// 	成员帐号
    userlist: Vec<String>,
    /// 	成员邮件群组
    grouplist: Vec<String>,
    /// 	成员部门
    department: Vec<u64>,
    /// 	群发权限。0: 企业成员, 1任何人， 2:组内成员，3:指定成员
    allow_type: u8,
    /// 	群发权限为指定成员时，需要指定成员，否则赋值失效
    allow_userlist: Vec<String>,
}
