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
