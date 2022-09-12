use std::env;

use rbatis::Rbatis;
use rbatis::executor::Executor;
use rbatis::py_sql;
use rbdc_mysql::driver::MysqlDriver;
use rbdc_pg::driver::PgDriver;
use rbdc_sqlite::driver::SqliteDriver;
use url::Url;
use serde::{Serialize, Deserialize};
use lazy_static::lazy_static;

lazy_static! {
        // connect database
    pub static ref DATABASE_URL: String =
        env::var("DATABASE_URL").expect("No DATABASE_URL provided");
}

#[derive(Debug, Default, Serialize, Deserialize, Clone,)]
pub struct Table {
    pub table_name: String,
    pub table_name_l: String,
    pub columns: Vec<Column>,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone,)]
pub struct Column {
    pub table_name: String,
    pub column_name: String,
    pub column_type: String,
    pub column_comment: String,

}

#[py_sql(
    "`select COLUMN_NAME as column_name, COLUMN_COMMENT as column_comment , COLUMN_TYPE as column_type, TABLE_NAME as table_name`
 ` from information_schema.COLUMNS c where c.table_name in (`
 trim ',':
   for key,item in table_name:
    `#{item},`
 `) and c.table_schema = #{schema} `"
)]
pub async fn get_mysql_table_columns(rb: &mut dyn Executor, schema: &str, table_name: Vec<&str>) -> Vec<Column> {
    impled!()
}

pub fn get_db_type() -> DBType {
    let parsed_db_url = Url::parse(&DATABASE_URL).ok();
    match parsed_db_url {
        Some(url) => match url.scheme() {
            "postgres" => DBType::Pg,
            "mysql" => DBType::Mysql,
            "sqlite" => DBType::Sqlite,
            _ => panic!("unsupport database"),
        },
        None => {
            panic!("Incorrect database url")
        }
    }
}

pub fn init_db() -> Rbatis {
    let db = Rbatis::new();

    match get_db_type() {
        DBType::Pg => db.init(PgDriver {}, DATABASE_URL.as_str()).expect("postgres database is unreachable"),
        DBType::Mysql => db.init(MysqlDriver {}, DATABASE_URL.as_str()).expect("mysql database is unreachable"),
        DBType::Sqlite => db.init(SqliteDriver {}, DATABASE_URL.as_str()).expect("sqlite database is unreachable"),
    };

    fast_log::init(fast_log::Config::new().console()).expect("rbatis init fail");

    db
}

pub fn mysql_column_type_to_rust_type(col_ty: &str) -> &str {
    let ty = col_ty.to_uppercase();
    if ty.starts_with("INT") {
        "i64"
    } else if ty.starts_with("BIGINT") {
        "i64"
    } else if ty.starts_with("BOOL") {
        "bool"
    } else if ty.starts_with("NUMERIC") {
        "f64"
    }else {
        "String"
    }
}

pub enum DBType {
    Mysql,
    Pg,
    Sqlite,
}


