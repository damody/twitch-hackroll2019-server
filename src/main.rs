use actix_web::{
    http,error, middleware, web, App, HttpRequest, HttpResponse, HttpServer,
};
use actix_cors::Cors;
use bytes::BytesMut;
use futures::{Future, Stream};
use json::JsonValue;
use serde_derive::{Deserialize, Serialize};
use failure::Error;

#[derive(Debug, Serialize, Deserialize)]
struct Data_new_live_master {
    user_id: String,
    channel_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Data_add_bits {
    user_id: String,
    channel_id: String,
    bits: i32,
}

#[derive(Debug, Serialize, Deserialize)]
struct Data_add_points {
    user_id: String,
    channel_id: String,
    talent: String,
    point: i32,
}

#[derive(Debug, Serialize, Deserialize)]
struct Data_get_live_master {
    user_id: String,
    channel_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Data_Result {
    msg: String,
}


#[derive(Debug, Serialize, Deserialize)]
struct Data_All {
    points: i32,
    attack: i32,
    speed: i32,
    armor: i32,
    hp: i32,
    steal: i32,
    heal: i32,
    true_damage: i32,
    miss: i32,
    crit: i32,
    stun: i32,
}

fn get_url() -> String {
    "mysql://bits:bitSS7.+.5*98@127.0.0.1:3306/bits".into()
}

fn run_sql(sql: &String, conn: &mut mysql::PooledConn) {
    println!("sql: {}", sql);
    let qres = conn.query(sql.clone());
    match qres {
        Ok(_) => {},
        Err(x) => { println!("Err {:#?}", x)},
    }
}

/// This handler uses json extractor
fn new_live_master(db: web::Data<mysql::Pool> ,item: web::Json<Data_new_live_master>) -> HttpResponse {
    println!("model: {:?}", &item);
    let mut conn = db.get_conn().unwrap();
    let sql = format!("insert into channel (channel_id) values ('{}');", item.0.channel_id);
    run_sql(&sql, &mut conn);
    let sql = format!("insert into user (user_id, channel_id) values ('{}','{}');", item.0.user_id, item.0.channel_id);
    run_sql(&sql, &mut conn);
    let res : Data_Result = Data_Result {msg : "ok".to_owned()};
    HttpResponse::Ok().json(res)
}

/// This handler uses json extractor
fn add_bits(db: web::Data<mysql::Pool> ,item: web::Json<Data_add_bits>) -> HttpResponse {
    println!("model: {:?}", &item);
    let mut conn = db.get_conn().unwrap();
    let sql = format!("select bits from user where user_id='{}' and channel_id='{}';", item.0.user_id, item.0.channel_id);
    println!("sql: {}", sql);
    let qres = conn.query(sql.clone()).unwrap();
    let mut count = 0;
    for row in qres {
        count += 1;
    }
    if count == 0 {
        let sql = format!("insert into user (user_id, channel_id) values ('{}','{}');", item.0.user_id, item.0.channel_id);
        run_sql(&sql, &mut conn);
    }
    let sql = format!("update user SET bits=bits+{}, points=points+{} WHERE user_id='{}' and channel_id='{}';", 
        item.0.bits, item.0.bits, item.0.user_id, item.0.channel_id);
    run_sql(&sql, &mut conn);
    let sql = format!("update user SET points=points+{} WHERE user_id='{}' and channel_id='{}';", 
        item.0.bits, item.0.channel_id, item.0.channel_id);
    run_sql(&sql, &mut conn);

    let res : Data_Result = Data_Result {msg : "ok".to_owned()};
    HttpResponse::Ok().json(res)
}
/// This handler uses json extractor
fn add_points(db: web::Data<mysql::Pool> ,item: web::Json<Data_add_points>) -> HttpResponse {
    println!("model: {:?}", &item);
    let mut conn = db.get_conn().unwrap();
    let sql = format!("select points from user where user_id='{}' and channel_id='{}';", item.0.user_id, item.0.channel_id);
    println!("sql: {}", sql);
    let qres = conn.query(sql.clone()).unwrap();
    let mut count = 0;
    let mut points: i32 = 0;
    for row in qres {
        count += 1;
        let a = row.unwrap().clone();
        points = mysql::from_value(a.get("points").unwrap());
    }
    if points < item.0.point {
        let res : Data_Result = Data_Result {msg : "fail".to_owned()};
        return HttpResponse::Ok().json(res)    
    }
    let sql = format!("update user SET points=points-{} where user_id='{}' and channel_id='{}';", item.0.point, item.0.user_id, item.0.channel_id);
    run_sql(&sql, &mut conn);
    let sql = format!("update channel SET {}={}+{} where channel_id='{}';", item.0.talent, item.0.talent, item.0.point, item.0.channel_id);
    run_sql(&sql, &mut conn);
    let res : Data_Result = Data_Result {msg : "ok".to_owned()};
    HttpResponse::Ok().json(res)
}
/// This handler uses json extractor
fn get_live_master(db: web::Data<mysql::Pool> ,item: web::Json<Data_get_live_master>) -> HttpResponse {
    println!("model: {:?}", &item);
    let mut conn = db.get_conn().unwrap();
    let sql = format!("select attack, speed, armor, hp, steal, heal, true_damage, miss, crit, stun from channel where channel_id='{}';", 
        item.0.channel_id);
    println!("sql: {}", sql);
    let qres = conn.query(sql.clone()).unwrap();
    let mut count = 0;
    let mut points: i32 = 0;
    let mut attack: i32 = 0;
    let mut speed: i32 = 0;
    let mut armor: i32 = 0;
    let mut hp: i32 = 0;
    let mut steal: i32 = 0;
    let mut heal: i32 = 0;
    let mut true_damage: i32 = 0;
    let mut miss: i32 = 0;
    let mut crit: i32 = 0;
    let mut stun: i32 = 0;
    for row in qres {
        count += 1;
        let a = row.unwrap().clone();
        attack = mysql::from_value(a.get("attack").unwrap());
        speed = mysql::from_value(a.get("speed").unwrap());
        armor = mysql::from_value(a.get("armor").unwrap());
        hp = mysql::from_value(a.get("hp").unwrap());
        steal = mysql::from_value(a.get("steal").unwrap());
        heal = mysql::from_value(a.get("heal").unwrap());
        true_damage = mysql::from_value(a.get("true_damage").unwrap());
        miss = mysql::from_value(a.get("miss").unwrap());
        crit = mysql::from_value(a.get("crit").unwrap());
        stun = mysql::from_value(a.get("stun").unwrap());
        break;
    }
    let sql = format!("select points from user where user_id='{}' and channel_id='{}';", 
        item.0.user_id, item.0.channel_id);
    println!("sql: {}", sql);
    let qres = conn.query(sql.clone()).unwrap();
    for row in qres {
        count += 1;
        let a = row.unwrap().clone();
        points = mysql::from_value(a.get("points").unwrap());
    }
    let res = Data_All {
        points: points,
        attack: attack,
        speed: speed,
        armor: armor,
        hp: hp,
        steal: steal,
        heal: heal,
        true_damage: true_damage,
        miss: miss,
        crit: crit,
        stun: stun,
    };
    HttpResponse::Ok().json(res)
}

fn main() -> std::result::Result<(), std::io::Error> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    let pool = mysql::Pool::new(get_url().as_str()).unwrap();
    
    HttpServer::new(move || {
        App::new()
        .wrap(
            Cors::new() // <- Construct CORS middleware builder
              //.allowed_origin("*")
              //.allowed_methods(vec!["GET", "POST", "OPTIONS"])
              //.allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
              //.allowed_header(http::header::CONTENT_TYPE)
              //.max_age(36000)
        )
            .data(pool.clone())
            // enable logger
            .wrap(middleware::Logger::default())
            .data(web::JsonConfig::default().limit(4096)) // <- limit size of the payload (global configuration)
            .service(web::resource("/new_live_master").route(web::post().to(new_live_master)))
            .service(web::resource("/add_bits").route(web::post().to(add_bits)))
            .service(web::resource("/add_points").route(web::post().to(add_points)))
            .service(web::resource("/get_live_master").route(web::post().to(get_live_master)))
    })
    .bind("0.0.0.0:80")?
    .run()
}
