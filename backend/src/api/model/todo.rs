// use uuid::Uuid;
// use chrono::{NaiveDateTime};
// use serde::{Deserialize, Serialize};

// #[derive(Debug, Clone)]
// pub struct TodoItem {
//     pub id: Uuid,
//     pub title: String,
//     pub description: String,
//     pub is_completed: bool,
//     pub created_at: NaiveDateTime,
// }

// #[derive(Serialize, Deserialize, Debug)]
// pub struct CreateTodoItem {
//     pub title: String,
//     pub description: String,
//     pub is_completed: bool,
// }

// #[derive(Serialize, Deserialize, Debug)]
// pub struct UpdateTodoItem {
//     pub title: Option<String>,
//     pub description: Option<String>,
//     pub is_completed: Option<bool>,
//     pub update_at: NaiveDateTime,
// }

// #[derive(Serialize, Deserialize, Debug)]
// pub struct DeleteTodoItem {
//     pub id: Uuid
// }

// #[derive(Serialize, Deserialize, Debug)]
// pub struct CompleteTodoItem {
//     pub id: Uuid
// }

// impl TodoItem {
//     /// TODO作成
//     pub async fn create_todo(
//         req: HttpRequest,
//         todo_req: web::Json<CreateTodoItem>,
//         pool: web::Data<Pool<PostgresConnectionManager<NoTls>>>
//     ) -> impl Responder {
//         let user = match jwt::verify(&req) {
//             Ok(user_info) => user_info,
//             Err(err) => {
//                 logger::log(logger::Header::ERROR, &err.to_string());
//                 return HttpResponse::new(StatusCode::UNAUTHORIZED);
//             }
//         };

//         let mut conn = pool.get().await.unwrap();

//         let mut transaction = conn.transaction().await.unwrap();
//         let rows_result = service::todo_service::check_existence_and_insert_todo(
//             &mut transaction,
//             todo_req.title,
//             todo_req.description,
//             user.sub.clone(),
//         ).await;

//         match rows_result {
//             Ok(result) => {
//                 match result {
//                     Ok(_rows) => {
//                         transaction.commit().await.unwrap();
//                         return HttpResponse::Ok().finish();
//                     }
//                     Err(err) => {
//                         transaction.rollback().await.unwrap();
//                         logger::log(logger::Header::ERROR, &err.to_string());
//                         return HttpResponse::InternalServerError().finish();
//                     }
//                 }
//             },
//             Err(mut err) => {
//                 return err.finish();
//             }
//         };
//     }

//     /// TODO更新
//     pub async fn update_todo(
//         _req: HttpRequest,
//         todo_req: web::Json<UpdateTodoItem>,
//         pool: web::Data<Pool<PostgresConnectionManager<NoTls>>>
//     ) -> impl Responder {
//         let mut conn = pool.get().await.unwrap();

//         let transaction = conn.transaction().await.unwrap();

//         let rows_result = transaction.execute(
//             r#"
//             UPDATE
//                 todos
//             SET
//                 title = $1,
//                 description = $2,
//                 is_completed = $3,
//                 updated_at = $4
//             "#,
//             &[
//                 &todo_req.id,
//                 &todo_req.title,
//                 &todo_req.description,
//                 &todo_req.is_completed,
//                 &todo_req.updated_at,
//             ]
//         ).await;

//         transaction.commit().await.unwrap();

//         match rows_result {
//             Ok(_result) => {
//                 return HttpResponse::Ok();
//             },
//             Err(error) => {
//                 logger::log(logger::Header::ERROR, &err.to_string());
//                 return HttpResponse::InternalServerError();
//             }
//         }
//     }

//     /// TODO削除
//     pub async fn delete_todo(
//         _req: HttpRequest,
//         todo_req: web::Json<DeleteTodoItem>,
//         pool: web::Data<Pool<PostgresConnectionManager<NoTls>>>
//     ) -> impl Responder {
//         let mut conn = pool.get().await.unwrap();

//         let transaction = conn.transaction().await.unrap();

//         let rows_result = transaction.execute(
//             r#"
//             UPDATE
//                 todos
//             SET
//                 delete_at = now()
//             WHERE
//                 todos.id = $1
//             "#,
//             &[
//                 &todo_req.id,
//             ]
//         ).await;

//         transaction.commit().await.unwrap();

//         match rows_result {
//             Ok(_result) => {
//                 return HttpResponse::Ok();
//             },
//             Err(err) => {
//                 logger::log(logger::Header::ERROR, &err.to_string());
//                 return HttpResponse::InternalServerError();
//             }
//         };
//     }

//     /// タスク完了更新
//     pub async fn complete_todo(
//         req: HttpRequest,
//         todo_req: web::Json<CompleteTodoItem>,
//         pool: web::Data<Pool<PostgresConnectionManager<NoTls>>>
//     ) -> impl Responder {
//         let user = match jwt::verify(&req) {
//             Ok(user_info) => user_info,
//             Err(err) => {
//                 logger::log(logger::Header::ERROR, &err.to_string());
//                 return HttpResponse::new(StatusCode::UNAUTHORIZED);
//             }
//         };

//         let mut conn = pool.get().await.unwrap();

//         let transaction = conn.transaction().await.unwrap();
        
//         // ユーザーの存在チェック
//         let user_row_result = transaction.query_one(
//             r#"
//             SELECT id
//             FROM users
//             WHERE name = $1
//             "#,
//             &[&user.sub]
//         ).await;
//         let user_id: i32 = match user_row_result {
//             Ok(user_row) => user_row.get("id"),
//             Err(error) => {
//                 logger::log(logger::Header::ERROR, &error.to_string());
//                 return HttpResponse::BadRequest().json(format!("ユーザー情報が存在しません。"))
//             }
//         };

//         let rows_result = transaction.execute(
//             r#"
//             UPDATE
//                 todos
//             SET
//                 deleted_at = now(),
//                 user_id = $2,
//                 is_completed = true
//             WHERE
//                 users.id = $1
//             ;
//             "#,
//             &[
//                 &todo_req.id,
//                 &user_id
//             ]
//         ).await;

//         transaction.commit().await.unwrap();

//         match rows_result {
//             Ok(_result) => {
//                 return HttpResponse::Ok().finish();
//             },
//             Err(error) => {
//                 logger::log(logger::Header::ERROR, &error.to_string());
//                 return HttpResponse::InternalServerError().finish();
//             }
//         }
//     }
// }