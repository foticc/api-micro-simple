use actix_web::body::MessageBody;
use actix_web::web::Json;
use serde::{Deserialize, Serialize};
use derive_more::Display;
use crate::service::menu_service::SearchParams;

#[derive(Serialize, Debug)]
pub struct CommonResult<T> {
    pub code: u16,
    pub msg: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
}

impl<T> CommonResult<T> {
    pub fn fail(code: u16, msg: String) -> Json<CommonResult<T>> {
        Json(
            Self {
                code,
                msg,
                data: None,
            }
        )
    }

    pub fn success_none()->Json<CommonResult<T>> {
        Json(
            Self {
                code: 200,
                msg: String::from("success"),
                data: None,
            }
        )
    }

    pub fn success(data: T) -> Json<CommonResult<T>> {
        Json(
            Self {
                code: 200,
                msg: String::from("success"),
                data:Some(data)
            }
        )
    }
}

impl<T> Display for CommonResult<T> where T: Serialize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}


#[derive(Serialize,Debug,Deserialize)]
pub struct FilterParam<T> {
    #[serde(rename(deserialize = "pageIndex", serialize = "pageIndex"))]
    pub page_index:u64,
    #[serde(rename(deserialize = "pageSize", serialize = "pageSize"))]
    pub page_size:u64,
    pub filters:Option<T>
}

#[derive(Serialize,Debug)]
pub struct PageResult<T> where T: Serialize {
    #[serde(rename(serialize = "pageIndex"))]
    pub page_index: u64,
    #[serde(rename(serialize = "pageSize"))]
    pub page_size:u64,
    pub list: Vec<T>,
    pub total: u64,
}

impl<T> PageResult<T> where T: Serialize {
    pub fn new(page_index: u64,page_size:u64, list: Vec<T>, total: u64) -> PageResult<T> where T: Serialize {
        Self{
            page_index,
            page_size,
            list,
            total,
        }
    }
}