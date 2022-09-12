// build.rs

use std::{env, path::Path, fs,};
use inflector::Inflector;
use rbatis::Rbatis;
use rbatis::executor::Executor;
use rbatis::py_sql;
use rbdc_mysql::driver::MysqlDriver;
use rbdc_pg::driver::PgDriver;
use rbdc_sqlite::driver::SqliteDriver;
use url::Url;
use lazy_static::lazy_static;
use serde::{Serialize, Deserialize};
use yarte::*;
extern crate inflector;

lazy_static! {
        // connect database
    pub static ref DATABASE_URL: String =
        env::var("DATABASE_URL").expect("No DATABASE_URL provided");
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let table_schema = "dwall";
    let table_names = vec!["Demand", "Book"];

    let mut db = init_db();

    let mut all_columns = get_mysql_table_columns(&mut db, table_schema, table_names.clone()).await.unwrap();
    all_columns.iter_mut().for_each(|x| {
        let col_ty = mysql_column_type_to_rust_type(&x.column_type);
        x.column_type = col_ty.to_string();
    });

    let tg = TableGlobal {
        table_names: table_names.iter().map(|x| x.to_lowercase().to_string()).collect(),
    };

    gen_root_file(".env", auto!(ywrite_html!(String, "{{> .env  tg}}")));
    gen_root_file(".gitignore", auto!(ywrite_html!(String, "{{> .gitignore tg}}")));
    gen_root_file("Cargo.toml", auto!(ywrite_html!(String, "{{> Cargo.toml.hbs  tg}}")));
    gen_root_file("README.md", auto!(ywrite_html!(String, "{{> README.md.hbs  tg}}")));

    // 生成mod
    gen_file("main.rs", auto!(ywrite_html!(String, "{{> src/main tg}}")));
    gen_file("entities/mod.rs", auto!(ywrite_html!(String, "{{> src/entities/mod tg}}")));
    gen_file("server/api/mod.rs", auto!(ywrite_html!(String, "{{> src/server/api/mod tg}}")));
    gen_file("server/api/model/mod.rs", auto!(ywrite_html!(String, "{{> src/server/api/model/mod tg}}")));
    gen_file("server/api/commands/mod.rs", auto!(ywrite_html!(String, "{{> src/server/api/commands/mod tg}}")));
    gen_file("server/mod.rs", auto!(ywrite_html!(String, "{{> src/server/mod tg}}")));
    gen_file("server/error.rs", auto!(ywrite_html!(String, "{{> src/server/error tg}}")));
    gen_file("service/mod.rs", auto!(ywrite_html!(String, "{{> src/service/mod tg}}")));
    gen_file("repository/mod.rs", auto!(ywrite_html!(String, "{{> src/repository/mod tg}}")));
    gen_file("drivers/mod.rs", auto!(ywrite_html!(String, "{{> src/drivers/mod tg}}")));
    gen_file("drivers/cache.rs", auto!(ywrite_html!(String, "{{> src/drivers/cache tg}}")));
    gen_file("drivers/db.rs", auto!(ywrite_html!(String, "{{> src/drivers/db tg}}")));

    table_names.iter().for_each(|table_name| {
        let table_name = String::from(*table_name);
        let table_name_l = table_name.to_string().to_lowercase();
        let columns: Vec<Column> = all_columns.iter().filter(|c| c.table_name.eq(&table_name_l)).map(|x| x.clone()).collect();
        println!("{:?}", columns);
        let table = Table {
            table_name: table_name.to_class_case(),
            table_name_l: table_name_l.to_string(),
            columns,
        };

        gen_file(format!("entities/{}_bo.rs", &table_name_l).as_str(), auto!(ywrite_html!(String, "{{> src/entities/BO table}}")));
        gen_file(format!("entities/{}_opt_bo.rs", &table_name_l).as_str(), auto!(ywrite_html!(String, "{{> src/entities/OptionBO table}}")));
        gen_file(format!("server/api/model/{}_vo.rs", &table_name_l).as_str(), auto!(ywrite_html!(String, "{{> src/server/api/model/model_vo table}}")));
        gen_file(format!("server/api/model/{}_opt_vo.rs", &table_name_l).as_str(), auto!(ywrite_html!(String, "{{> src/server/api/model/model_opt_vo table}}")));
        gen_file(format!("server/api/model/{}_create_vo.rs", &table_name_l).as_str(), auto!(ywrite_html!(String, "{{> src/server/api/model/model_create_vo table}}")));
        gen_file(format!("server/api/model/{}_update_vo.rs", &table_name_l).as_str(), auto!(ywrite_html!(String, "{{> src/server/api/model/model_update_vo table}}")));
        gen_file(format!("server/api/commands/{}_controller.rs", &table_name_l).as_str(), auto!(ywrite_html!(String, "{{> src/server/api/commands/controller table}}")));
        gen_file(format!("service/{}_service.rs", &table_name_l).as_str(), auto!(ywrite_html!(String, "{{> src/service/service table}}")));
        gen_file(format!("repository/{}_repo.rs", &table_name_l).as_str(), auto!(ywrite_html!(String, "{{> src/repository/repository table}}")));
    });


}

fn gen_root_file(file: &str, content: String ) {
    let dest_path = Path::new("../gen/").join(file);
    let dir_path = dest_path.parent().unwrap();
    if !dir_path.exists() {
        fs::create_dir_all(dir_path);
    }
    fs::write(
        &dest_path,
        content
        ).unwrap();
}

fn gen_file(file: &str, content: String ) {
    let dest_path = Path::new("../gen/src/").join(file);
    let dir_path = dest_path.parent().unwrap();
    if !dir_path.exists() {
        fs::create_dir_all(dir_path);
    }
    fs::write(
        &dest_path,
        content
        ).unwrap();
}

fn mysql_column_type_to_rust_type(col_ty: &str) -> &str {
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

#[derive(Debug, Default, Serialize, Deserialize, Clone,)]
pub struct TableGlobal {
    pub table_names: Vec<String>,
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
async fn get_mysql_table_columns(rb: &mut dyn Executor, schema: &str, table_name: Vec<&str>) -> Vec<Column> {
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

pub enum DBType {
    Mysql,
    Pg,
    Sqlite,
}