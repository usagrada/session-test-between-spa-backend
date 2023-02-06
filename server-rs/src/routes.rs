use crate::models::User;
use actix_session::Session;
use actix_web::{get, post, web, HttpResponse, Responder};
use serde::Deserialize;
use sqlx::{Pool, Sqlite};
use uuid::Uuid;

#[get("/")]
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
pub async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

pub async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[derive(Deserialize)]
pub struct AddUserInfo {
    name: String,
}

#[post("/users/add")]
pub async fn add_user(
    pool: web::Data<Pool<Sqlite>>,
    info: web::Json<AddUserInfo>,
) -> impl Responder {
    // let name = path.into_inner();
    let name = info.name.clone();
    println!("Add User: {}", &name);
    let text = format!("User {} added!", &name);
    web::block(|| async move {
        let user_id = Uuid::new_v4();
        let mut conn = pool.acquire().await.unwrap();
        let query_str = format!(
            "insert into users (name, user_id) values ('{}', '{}')",
            &name, &user_id
        );
        println!("Query: {}", &query_str);
        let query = sqlx::query_as::<_, User>(&query_str);
        query.fetch_all(&mut conn).await.expect("add user failed");
    })
    .await
    .expect("error in add_user")
    .await;
    HttpResponse::Ok().body(text)
}

#[post("/login")]
pub async fn login(
    session: Session,
    pool: web::Data<Pool<Sqlite>>,
    info: web::Json<AddUserInfo>,
) -> impl Responder {
    let name = info.name.clone();
    println!("Login Request from User: {}", &name);
    let text = format!("Login Success!");

    let result = web::block(|| async move {
        let mut conn = pool.acquire().await.unwrap();
        let query_str = format!("select * from users where name = '{}'", &name);
        println!("Query: {}", &query_str);
        let query = sqlx::query_as::<_, User>(&query_str);
        let user_data = query.fetch_all(&mut conn).await.expect("add user failed");
        user_data
    })
    .await
    .expect("error in add_user")
    .await;

    if result.len() == 0 {
        return HttpResponse::ExpectationFailed().body("User not found");
    }
    let result = &result[0];
    let session_data = session.get::<String>("user_id").unwrap();
    match session_data {
        Some(user) => {
            println!("Session User: {}", user);
        }
        None => {
            println!("No Session User");
            session.insert("user_id", &result.user_id).unwrap();
        }
    }

    HttpResponse::Ok().json(User {
        user_id: result.user_id.clone(),
        name: result.name.clone(),
    })
}

#[get("/users/{name}")]
pub async fn get_user(
    session: Session,
    pool: web::Data<Pool<Sqlite>>,
    path: web::Path<String>,
) -> impl Responder {
    let name = path.into_inner();
    println!("Get User: {}", &name);

    let session_data = session.get::<String>("user").unwrap();
    match session_data {
        Some(user) => {
            println!("Session User: {}", user);
        }
        None => {
            println!("No Session User");
            session.insert("user", &name).unwrap();
        }
    }

    let text = format!("User data {} is got!", &name);
    web::block(|| async move {
        let mut conn = pool.acquire().await.unwrap();
        let query_str = format!("insert into users (name) values ('{}')", &name);
        println!("Query: {}", &query_str);
        let query = sqlx::query_as::<_, User>(&query_str);
        query.fetch_all(&mut conn).await.expect("add user failed");
    })
    .await
    .expect("error in add_user")
    .await;
    HttpResponse::Ok().body(text)
}

#[get("/users")]
pub async fn get_all_users(pool: web::Data<Pool<Sqlite>>) -> impl Responder {
    println!("Get All User");
    let users = web::block(|| {
        async move {
            // let query = query!("insert into users (name) values ($1)", name);
            let mut conn = pool.acquire().await.unwrap();
            let query_str = format!("select * from users");
            let query = sqlx::query_as::<_, User>(&query_str);
            let res = query
                .fetch_all(&mut conn)
                .await
                .expect("get all user failed");
            println!("Get All User: {:?}", res.len());
            res
        }
    })
    .await
    .expect("error in add_user")
    .await;

    HttpResponse::Ok().json(users)
}
