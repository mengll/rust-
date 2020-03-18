// 街巷当前Kafka的数据
pub fn parse_kafka(){
    let f = File::open("。。。/Desktop/nginxlog.txt").unwrap();
    let reader = BufReader::new(f);
    for line in reader.lines() {
        // line 是 std::result::Result<std::string::String, std::io::Error> 类型
        // line 不包含换行符
        let line = line.unwrap();
        let pk = line.as_str().splitn(10," ").collect::<Vec<&str>>();
        let mut dt:String = pk[3].to_string();
        dt.remove(0);
        let dd = DateTime::parse_from_str((dt.to_string() + " +0800").as_str(), "%-d/%b/%-Y:%-H:%M:%S %z").unwrap();
        let clicktime = dd.timestamp();
        let uu = "http://adt.qcwan.com".to_string() + pk[6];
        let u = Url::parse(uu.as_str()).unwrap();
        match u.path() {
            "/v1/req" => {
                let s = u.query_pairs().into_iter();
                let mut ocpc = Ocpc::new();
                for ss in s {
                    let v = ss.1.as_ref();
                    match ss.0.as_ref() {
                        "gameid" |"game_id" => {
                            ocpc.gameid = v;
                        }
                        "muid"|"meiMD5"|"uid"|"imei_md5"|"idfaMD5"|"imeiMD5"|"oaid_md5" =>{
                            if v != "__OAID_MD5__" {
                                ocpc.muid = v;
                            }
                        },
                        "callback"|"baclurl"|"callback_url" =>{
                            ocpc.backurl = v;
                        },
                        "rid" =>{
                            ocpc.rid = v;
                        },
                        "ostype"|"os"|"OS"|"device_os_type" =>{
                            ocpc.ostype = v;
                        },
                        "idfa"|"ifa"|"IDFA" =>{
                            ocpc.idfa = v;
                        },
                        "imei" =>{
                            ocpc.imei = v;
                        },
                        "apptype"|"app_type" =>{
                            ocpc.apptype = v;
                        },
                        "ip"|"Ip"=>{
                            ocpc.ip = v;
                        },
                        "advertiser_id"|"company_id"|"company"|"companyid"|"userid" =>{
                            ocpc.uid = v;
                        },
                        "mac"|"MAC_MD5" =>{
                            ocpc.mac = v;
                        },
                        "uuid"|"UniqueID" => {
                            ocpc.uuid = v;
                        },
                        "akey"|"c"|"key"|"delivery" =>{
                            ocpc.akey = v;
                        },
                        "appid" |"app" =>{
                            ocpc.appid = v;
                        },
                        "click_id" =>{
                            ocpc.clickid = v;
                        },
                        "productid" =>{
                            ocpc.productid = v;
                        },
                        "deviceid"|"devicetype" =>{
                            ocpc.deviceid = v;
                        },
                        "signature"|"p"|"source" =>{
                            ocpc.signature = v;
                        },
                        "sign"|"token" =>{
                            ocpc.sign = v;
                        },
                        "campaign_id"|"outer_campaign_id"|"wx_adgroup_id" =>{
                            ocpc.ad_group = v;
                        },
                        "adid"|"adgroup_id"|"outer_adgroup_id"|"pid"|"wx_campaign_id" =>{
                            ocpc.ad_id = v;
                        },
                        "cid"|"creative_id"|"outer_creative_id"|"aid"|"wx_creative_id" => {
                            ocpc.cid = v;
                        },
                        "is_md" =>{
                            ocpc.is_md = v;
                        },
                        "slot" =>{
                            ocpc.slot = v;
                        },
                        "union_site,adgroup_name" =>{
                            ocpc.union_site = v;
                        },
                        "ua" =>{
                            ocpc.ua = v;
                        },
                        "oaid" =>{
                            if v != "__OAID__" {
                                ocpc.oaid = v;
                            }
                        },
                        "mt" =>{
                            let mt = format!("{:.1}",v);
                            let mt:u64 = mt.parse().unwrap_or(0);
                            ocpc.message_type = mt;
                        },
                        "business_id" =>{
                            let bs:u16 = v.parse().unwrap();
                            ocpc.business_id = bs;
                        },
                        "page_id" =>{
                            let page_id:u64 = v.parse().unwrap();
                            ocpc.page_id = page_id;
                        },
                        "click_time"|"ts" =>{
                            let ts:u64 = v.parse().unwrap_or(0);
                            ocpc.clicktime = clicktime as u64;
                            ocpc.createts = ts;
                        },
                        "channel"|"ch"|"af_channel" =>{
                            let ch:u16 = v.parse().unwrap_or(0);
                            if ch != 0 {
                                ocpc.channel = ch;
                            }
                        },
                        "apk_id" =>{
                            let gid:u64 = v.parse().unwrap_or(0);
                            ocpc.gid = gid;
                        }
                        _=> ()
                    }
                }

                // 设置muid的值
                if ocpc.muid == "" {
                    if  ocpc.imei != "" && ocpc.is_md == "false" && ocpc.imei != "__IMEI__" {
                        ocpc.muid = ocpc.md5(ocpc.imei).as_str()
                    }
                    if ocpc.imei!= "" && ocpc.is_md == "true" && ocpc.imei != "__IMEI__" {
                        ocpc.muid = ocpc.imei.clone()
                    }
                    if ocpc.idfa != "" {
                        ocpc.muid = ocpc.md5(ocpc.idfa).as_str()
                    }
                    if ocpc.muid == "" && ocpc.oaid != "" && ocpc.oaid != "__OAID__" {
                        let oaid =  ocpc.oaid.clone();
                        ocpc.muid = oaid;
                    }
                }

                println!("muid:{:?} oiad:{:?} imei:{:?}",ocpc.muid,ocpc.oaid,ocpc.imei);
                if ocpc.muid == "" && ocpc.oaid =="" && ocpc.imei == "__IMEI__" && ocpc.idfa == "" {
                    println!("{:?}",line)
                }
            },
            // "/gdt"|"/wx" => println!("wx,gdt"),
            _ => ()
        }
    }
}
