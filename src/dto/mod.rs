use serde::{Deserialize, Serialize};

/// 创建部门参数
#[derive(Debug, Deserialize, Serialize)]
pub struct ParamsCreateDepartment {
    pub name: String,
    #[serde(rename = "parentid")]
    pub parent_id: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<u32>,
}

/// 更新部门参数
#[derive(Debug, Deserialize, Serialize)]
pub struct ParamsUpdateDepartment {
    pub id: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "parentid", skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<u32>,
}

/// 搜索部门参数
#[derive(Debug, Deserialize, Serialize)]
pub struct ParamsSerchDepartment {
    pub name: String,
    // 是否模糊查询 1/0：是否模糊匹配
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fuzzy: Option<u32>,
}

/// 创建用户参数
#[derive(Debug, Deserialize, Serialize)]
pub struct ParamsCreateUser {
    /// 成员UserID。企业邮帐号名，邮箱格式
    #[serde(rename = "userid")]
    pub user_id: String,
    /// 成员名称。长度为1~64个字节
    pub name: String,
    /// 成员所属部门id列表，不超过20个
    pub department: Vec<u64>,
    /// 职位信息。长度为0~64个字节
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<String>,
    /// 手机号码
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mobile: Option<String>,
    /// 座机号码
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tel: Option<String>,
    /// 编号
    #[serde(skip_serializing_if = "Option::is_none", rename = "extid")]
    pub ext_id: Option<String>,
    /// 性别。1表示男性，2表示女性
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gender: Option<String>,
    /// 别名列表
    /// 1.Slaves 上限为5个
    /// 2.Slaves 为邮箱格式
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slaves: Option<Vec<String>>,
    /// 英文和数字
    pub password: String,
    /// 用户重新登录时是否重设密码, 登陆重设密码后，该标志位还原。0表示否，1表示是，缺省为0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpwd_login: Option<u8>,
}

/// 更新用户参数
#[derive(Debug, Deserialize, Serialize)]
pub struct ParamsUpdateUser {
    //成员UserID。企业邮帐号名，邮箱格式
    #[serde(rename = "userid")]
    pub user_id: String,
    /// 成员名称。长度为0~64个字节
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 成员所属部门id列表，不超过20个
    pub department: Option<Vec<u64>>,
    ///	职位信息。长度为0~64个字节
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<String>,
    ///	手机号码
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mobile: Option<String>,
    /// 座机号码
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tel: Option<String>,
    ///	编号
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extid: Option<String>,
    ///	性别。1表示男性，2表示女性
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gender: Option<String>,
    /// slaves	否	别名列表
    /// 1.Slaves 上限为5个
    /// 2.Slaves 为邮箱格式
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slaves: Option<Vec<String>>,
    /// 启用/禁用成员。1表示启用成员，0表示禁用成员
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable: Option<u8>,
    ///	密码
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    // 用户重新登录时是否重设密码, 登陆重设密码后，该标志位还原。0表示否，1表示是，缺省为0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpwd_login: Option<u8>,
}

/// 创建群组参数
/// {
/// 	"groupid": "zhangsangroup@gzdev.com",
/// 	"groupname": "zhangsangroup ,
/// 	"userlist": ["zhangsanp@gzdev.com", "lisi@gzdev.com"],
/// 	"grouplist": ["group@gzdev.com"],
/// 	"department": [1, 2],
/// 	"allow_type": 4,
/// 	"allow_userlist": ["zhangsanp@gzdev.com"]
/// }
#[derive(Debug, Deserialize, Serialize)]
pub struct ParamsCreateGroup {
    /// 是	邮件群组名称
    pub groupid: String,
    /// 是	邮件群组名称
    pub groupname: String,
    /// 否	成员帐号，userlist，grouplist，department至少一个。成员由userlist，grouplist，department共同组成
    #[serde(skip_serializing_if = "Option::is_none")]
    pub userlist: Option<Vec<String>>,
    ///	否	成员邮件群组，userlist，grouplist，department至少一个。成员由userlist，grouplist，department共同组成
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grouplist: Option<Vec<String>>,
    /// 否	成员部门，userlist，grouplist，department至少一个。成员由userlist，grouplist，department共同组成
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department: Option<Vec<u64>>,
    ///	是	群发权限。0: 企业成员, 1任何人， 2:组内成员，3:指定成员
    pub allow_type: u8,
    /// 否	群发权限为指定成员时，需要指定成员
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_userlist: Option<String>,
}

/// 更新群组参数

#[derive(Debug, Deserialize, Serialize)]
pub struct ParamsUpdateGroup {
    /// 是	邮件群组名称
    pub groupid: String,
    /// 是	邮件群组名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groupname: Option<String>,
    /// 否	成员帐号，userlist，grouplist，department至少一个。成员由userlist，grouplist，department共同组成
    #[serde(skip_serializing_if = "Option::is_none")]
    pub userlist: Option<Vec<String>>,
    ///	否	成员邮件群组，userlist，grouplist，department至少一个。成员由userlist，grouplist，department共同组成
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grouplist: Option<Vec<String>>,
    /// 否	成员部门，userlist，grouplist，department至少一个。成员由userlist，grouplist，department共同组成
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department: Option<Vec<u64>>,
    ///	是	群发权限。0: 企业成员, 1任何人， 2:组内成员，3:指定成员
    pub allow_type: Option<u8>,
    /// 否	群发权限为指定成员时，需要指定成员
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_userlist: Option<String>,
}
