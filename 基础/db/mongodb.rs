extern crate mongodb;
use mongodb::{bson, doc};
use mongodb::{Bson};
use mongodb::{Client, ThreadedClient};
use mongodb::db::ThreadedDatabase;

fn main(){
  let client = Client::with_uri("mongodb://192.168.1.246:27017")
      .expect("Failed to initialize client.");

  let coll = client.db("dsp").collection("movies");
  // coll.insert_one(doc!{ "title": "mengll","age":24 }, None).unwrap();

  // 查询条件
  let filter = doc!{
     "title":"mengll",
     "age":{"$gte":23}
  };

  let cursor = coll.find(Some(filter), None).unwrap();
  for result in cursor {
    if let Ok(item) = result {
      Resp::Ok();
      if let Some(Bson::I32(age)) = item.get("age") {
        println!("{}",age);
      }
      if let Some(&Bson::String(ref title)) = item.get("title") {
        println!("title--1: {}", title);
      }
    }
  }

}


// 新版的MongoDB的读取的操作 mongodb 1.1.1 需要的MongoDB的数据库的集群的需要 4.2 以上才行

use mongodb::{Client, options::{ClientOptions,FindOptions}};
use mongodb::bson::doc;
use tokio::stream::StreamExt;
use mongodb::{
    bson::Document
};

#[tokio::main]
async fn main(){
    let client_options = ClientOptions::parse("mongodb://root:passs@192.168.1.241:44126/admin").await.unwrap();
    let client = Client::with_options(client_options).unwrap();
    let db = client.database("dsp");
    let adtconf = db.collection("adt_conf");
    let limit  = FindOptions::builder().limit(1).build();
    let mut res  = adtconf.find(doc!{"test":1},limit).await.unwrap();
    while  let Some(result) =  res.next().await {
        match result {
             Ok(docs)=> {
                 println!("dock");
                 let d = docs as Document;
                 let s = d.get("test").unwrap().as_i32().unwrap();
                 println!("{}",s)},
             Err(e)=> println!("e：a {:?}",e.labels())
        }
    }

    // match res {
    //     Some(docuemnt) => for k in  docuemnt.iter(){
    //         println!("{} ---{}",k.0,k.1);
    //     } ,
    //     None => return
    // }
    // for db_name in db.list_collection_names(None).await.unwrap() {
    //     println!("{}", db_name);
    // }

}

