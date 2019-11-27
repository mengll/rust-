fn get_pg(){
    let conn = Connection::connect("postgres://tianyan:Wuaf%21%40%23comTydaHan52@192.168.1.241:44125/tianyan", TlsMode::None).unwrap();
    println!("{}",conn.is_active()); // 当前是否激活

    // 传递的参数的实现 参数切片 传递的参数
    let id :i32 = 3;
    for row in &conn.query("SELECT id, muid FROM ty_device_active_new  where id = $1", &[&id]).unwrap() {
        let id:i32 = row.get(0);
        let muid:String = row.get("muid");
        println!("Found person {:?}---{}", id,muid);
    }

    // 调用账号的信息
    let s = get_muid(&conn,5);
    println!("5=>{}",s);
}

fn get_muid(pg:&postgres::Connection,id:i32)->String {
    for row in &pg.query("SELECT id, muid FROM ty_device_active_new  where id = $1", &[&id]).unwrap() {
        let id:i32 = row.get(0);
        let c:String = row.get(1);
        println!("1=>{}",c);
        let muid:String = row.get("muid");
        println!("Found person {:?}---{}", id,muid);
        return muid;
    }
    return String::from("");
}
