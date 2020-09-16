 pub async fn get_msg(address:&'static str) -> String{
  let uri = address.parse::<Uri>().unwrap();
  let client = Client::new();
  let req = Request::builder().method(Method::GET).uri(uri).body(Body::from("echo")).unwrap();
  let  res = client.request(req).await.unwrap();
  println!("{:?}", hyper::body::to_bytes(res.into_body()).await.unwrap());
  "12".to_string()
}

// 发送Http的Get 请求 现在这个写法在处理HTTPS的请求的时候，会遇到一定的问题
