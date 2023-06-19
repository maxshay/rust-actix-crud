use super::DbPool;

use actix_web::{delete, get, post, put, web, Error, HttpResponse};

use crate::models::TweetPayload;
use crate::ops::{add_a_tweet, delete_tweet, find_all, find_by_id, update_tweet};

#[get("/tweets")]
async fn index(pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    let tweets = web::block(move || {
        let mut conn = pool.get()?;
        find_all(&mut conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(tweets))
}

#[post("/tweets")]
async fn create(
    pool: web::Data<DbPool>,
    payload: web::Json<TweetPayload>,
) -> Result<HttpResponse, Error> {
    let tweet = web::block(move || {
        let mut conn = pool.get()?;
        add_a_tweet(&payload.message, &mut conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(tweet))
}

#[get("/tweets/{id}")]
async fn show(id: web::Path<i32>, pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    let tweet = web::block(move || {
        let mut conn = pool.get()?;
        find_by_id(id.into_inner(), &mut conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(tweet))
}

#[put("/tweets/{id}")]
async fn update(
    id: web::Path<i32>,
    payload: web::Json<TweetPayload>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, Error> {
    let tweet = web::block(move || {
        let mut conn = pool.get()?;
        update_tweet(id.into_inner(), payload.message.clone(), &mut conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(tweet))
}

#[delete("/tweets/{id}")]
async fn destroy(id: web::Path<i32>, pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    let result = web::block(move || {
        let mut conn = pool.get()?;
        delete_tweet(id.into_inner(), &mut conn)
    })
    .await?
    .map(|tweet| HttpResponse::Ok().json(tweet))
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(result)
}

// https://framagit.org/mohsen/rust_crud_rest_api/-/blob/master/tests/user_controller_test.rs
