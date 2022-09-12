// build.rs

pub mod db;
pub mod storage;

use std::collections::HashMap;

use db::{get_mysql_table_columns, init_db, mysql_column_type_to_rust_type, get_mysql_table_ddl};
use inflector::Inflector;
use serde::{Deserialize, Serialize};
use storage::{gen_root_file, gen_file};
use yarte::*;

use crate::db::{Column, Table};
extern crate inflector;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let table_schema = "dwall";
    let table_names = vec!["Demand", "Book"];

    let mut db = init_db();

    let mut ddls: Vec<String> = vec![];
    for table_name in &table_names {
        let result_map = get_mysql_table_ddl(&mut db, table_name).await.unwrap();
        let ddl = result_map.get("Create Table").unwrap();
        ddls.push(ddl.clone());
    }

    let mut all_columns = get_mysql_table_columns(&mut db, table_schema, table_names.clone())
        .await
        .unwrap();
    all_columns.iter_mut().for_each(|x| {
        let col_ty = mysql_column_type_to_rust_type(&x.column_type);
        x.column_type = col_ty.to_string();
    });

    let tg = TableGlobal {
        table_names: table_names
            .iter()
            .map(|x| x.to_lowercase().to_string())
            .collect(),
        table_ddls: ddls
    };

    // 数据库表DDL
    gen_root_file("migrations/V1__initial.sql", auto!(ywrite!(String, "{{> migrations/ddl  tg}}")));

    // 根目录文件
    gen_root_file(".env", auto!(ywrite!(String, "{{> .env  tg}}")));
    gen_root_file(
        ".gitignore",
        auto!(ywrite!(String, "{{> .gitignore tg}}")),
    );
    gen_root_file(
        "Cargo.toml",
        auto!(ywrite!(String, "{{> Cargo.toml.hbs  tg}}")),
    );
    gen_root_file(
        "README.md",
        auto!(ywrite!(String, "{{> README.md.hbs  tg}}")),
    );

    // 生成mod
    gen_file("main.rs", auto!(ywrite!(String, "{{> src/main tg}}")));
    gen_file(
        "entities/mod.rs",
        auto!(ywrite!(String, "{{> src/entities/mod tg}}")),
    );
    gen_file(
        "server/api/mod.rs",
        auto!(ywrite!(String, "{{> src/server/api/mod tg}}")),
    );
    gen_file(
        "server/api/model/mod.rs",
        auto!(ywrite!(String, "{{> src/server/api/model/mod tg}}")),
    );
    gen_file(
        "server/api/commands/mod.rs",
        auto!(ywrite!(String, "{{> src/server/api/commands/mod tg}}")),
    );
    gen_file(
        "server/mod.rs",
        auto!(ywrite!(String, "{{> src/server/mod tg}}")),
    );
    gen_file(
        "server/error.rs",
        auto!(ywrite!(String, "{{> src/server/error tg}}")),
    );
    gen_file(
        "service/mod.rs",
        auto!(ywrite!(String, "{{> src/service/mod tg}}")),
    );
    gen_file(
        "repository/mod.rs",
        auto!(ywrite!(String, "{{> src/repository/mod tg}}")),
    );
    gen_file(
        "drivers/mod.rs",
        auto!(ywrite!(String, "{{> src/drivers/mod tg}}")),
    );
    gen_file(
        "drivers/cache.rs",
        auto!(ywrite!(String, "{{> src/drivers/cache tg}}")),
    );
    gen_file(
        "drivers/db.rs",
        auto!(ywrite!(String, "{{> src/drivers/db tg}}")),
    );

    table_names.iter().for_each(|table_name| {
        let table_name = String::from(*table_name);
        let table_name_l = table_name.to_string().to_lowercase();
        let columns: Vec<Column> = all_columns
            .iter()
            .filter(|c| c.table_name.eq(&table_name_l))
            .map(|x| x.clone())
            .collect();
        println!("{:?}", columns);
        let table = Table {
            table_name: table_name.to_class_case(),
            table_name_l: table_name_l.to_string(),
            columns,
        };

        gen_file(
            format!("entities/{}_bo.rs", &table_name_l).as_str(),
            auto!(ywrite!(String, "{{> src/entities/BO table}}")),
        );
        gen_file(
            format!("entities/{}_opt_bo.rs", &table_name_l).as_str(),
            auto!(ywrite!(String, "{{> src/entities/OptionBO table}}")),
        );
        gen_file(
            format!("server/api/model/{}_vo.rs", &table_name_l).as_str(),
            auto!(ywrite!(
                String,
                "{{> src/server/api/model/model_vo table}}"
            )),
        );
        gen_file(
            format!("server/api/model/{}_opt_vo.rs", &table_name_l).as_str(),
            auto!(ywrite!(
                String,
                "{{> src/server/api/model/model_opt_vo table}}"
            )),
        );
        gen_file(
            format!("server/api/model/{}_create_vo.rs", &table_name_l).as_str(),
            auto!(ywrite!(
                String,
                "{{> src/server/api/model/model_create_vo table}}"
            )),
        );
        gen_file(
            format!("server/api/model/{}_update_vo.rs", &table_name_l).as_str(),
            auto!(ywrite!(
                String,
                "{{> src/server/api/model/model_update_vo table}}"
            )),
        );
        gen_file(
            format!("server/api/commands/{}_controller.rs", &table_name_l).as_str(),
            auto!(ywrite!(
                String,
                "{{> src/server/api/commands/controller table}}"
            )),
        );
        gen_file(
            format!("service/{}_service.rs", &table_name_l).as_str(),
            auto!(ywrite!(String, "{{> src/service/service table}}")),
        );
        gen_file(
            format!("repository/{}_repo.rs", &table_name_l).as_str(),
            auto!(ywrite!(
                String,
                "{{> src/repository/repository table}}"
            )),
        );
    });
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct TableGlobal {
    pub table_names: Vec<String>,
    pub table_ddls: Vec<String>,
}
