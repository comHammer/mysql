use std::fmt::format;

use mysql::*;
use mysql::prelude::*;


#[actix_web::main]
async fn main() {
    let url = "mysql://root:WERASDwer1@localhost:3306/api";
    let pool = Pool::new(url).unwrap();
    let mut conn = pool.get_conn().unwrap();
    let stmt = conn.prep("insert into users (name,phone_number) values (:name,:phone_number);").unwrap();
    for i in 0..10000{
        let str1 = format!("user{}",i);
        let str2 = format!("1386477{}",i*2);
        println!("这是第{}个数据：{},   {}",i,str1,str2);
        conn.exec_drop(&stmt, params!{
            "name"=>str1,
            "phone_number"=>str2,
        }).unwrap();
        
    }
}
