use actix_cors::Cors;
use actix_web::{web::Data, App, HttpServer};
use dotenvy::dotenv;

use api::middlewares::jwt_middleware;
use libraries::app_state::AppState;

mod api;
mod constants;
mod db;
mod errors;
mod libraries;

const PROJECT_PATH: &'static str = env!("CARGO_MANIFEST_DIR");

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    dotenv().ok();

    let pool = db::pool::get_db_pool().await;

    let app_state = AppState::init(&pool);

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            // .allowed_origin("http://localhost:3000")
            .allowed_methods(vec!["GET", "PUT", "POST", "DELETE"])
            .allowed_headers(vec!["Authorization", "Content-Type"])
            .max_age(60 * 60 * 24);

        App::new()
            .wrap(jwt_middleware::JwtMiddleware)
            .wrap(cors)
            .app_data(Data::new(pool.clone()))
            .app_data(Data::new(app_state.clone()))
            .service(api::handler::handlers::api_scope())
    })
    .bind("0.0.0.0:8080")?
    .workers(20) // 同時接続は20人を想定
    .run()
    .await
}

#[allow(dead_code)]
#[cfg(test)]
mod tests {
    use crate::libraries::logger;
    use chrono::NaiveDateTime;
    use futures::future::join_all;
    use rand::{Rng, seq::SliceRandom};
    use reqwest::{header, Client};
    use serde::{Deserialize, Serialize};

    #[tokio::test]
    async fn test_async_15_requests() -> Result<(), reqwest::Error> {
        let mut futures = Vec::new();

        #[derive(Serialize, Deserialize, Debug)]
        struct LoginPostData {
            name: String,
            password: String,
        }

        #[derive(Debug, Deserialize)]
        struct LoginResponseData {
            id: i32,
            name: String,
            token: String,
        }

        #[derive(Debug, Deserialize)]
        struct TableResponseData {
            id: i32,
            table_number: i32,
            note: Option<String>,
        }

        #[derive(Debug, Deserialize)]
        struct MenuResponseData {
            id: i32,
            name: String,
            cook_time_seconds: i32,
            price: i32,
        }

        #[derive(Serialize, Deserialize, Debug)]
        struct AddOrderPostData {
            restaurant_table_id: i32,
            menu_id: i32,
        }

        #[derive(Serialize, Deserialize, Debug)]
        struct AddOrdersPostData {
            restaurant_table_id: i32,
            menu_ids: Vec<i32>,
        }

        #[derive(Debug, Deserialize)]
        struct OrderResponseData {
            id: i32,
            table_number: i32,
            table_note: Option<String>,
            menu_name: Option<String>,
            price: Option<i32>,
            cook_time_seconds: Option<i32>,
            order_id: Option<i64>,
            expected_cook_finish_time: Option<NaiveDateTime>,
            ordered_time: Option<NaiveDateTime>,
            is_served_by_staff: Option<bool>,
            served_by_user_id: Option<i32>,
            serve_staff_name: Option<String>,
            checked_by_user_id: Option<i32>,
            check_staff_name: Option<String>,
        }

        #[derive(Serialize, Deserialize, Debug)]
        struct DeleteOrderRequest {
            order_id: i64,
        }

        #[derive(Serialize, Deserialize, Debug)]
        struct CompleteOrderRequest {
            order_id: i64,
        }

        #[derive(Serialize, Deserialize, Debug)]
        struct DeleteAllOrdersRequest {
            restaurant_table_id: i32,
        }

        futures.push(async move {
            for _ in 0..15 {
                let client = Client::new();
                let mut rng = rand::thread_rng();
                let number = rng.gen_range(1..=20);
                let user: LoginResponseData = client
                    .post("http://localhost/api/auth/login")
                    .json(
                        &(LoginPostData {
                            name: format!("{}{}", "test_user", number),
                            password: "password".to_string(),
                        }),
                    )
                    .send()
                    .await?
                    .json()
                    .await?;
                logger::log(logger::Header::INFO, &format!("{:?}", user));

                let tables: Vec<TableResponseData> = client
                    .get("http://localhost/api/table")
                    .header(header::AUTHORIZATION, format!("Bearer {}", user.token))
                    .send()
                    .await?
                    .json()
                    .await?;
                logger::log(logger::Header::INFO, &format!("{:?}", tables));

                let table_id = rng.gen_range(0..=9);
                let table = tables.get(table_id).unwrap();

                let menu_items: Vec<MenuResponseData> = client
                    .get("http://localhost/api/menu")
                    .header(header::AUTHORIZATION, format!("Bearer {}", user.token))
                    .send()
                    .await?
                    .json()
                    .await?;
                logger::log(logger::Header::INFO, &format!("{:?}", menu_items));

                let pick_one = rng.gen_range(0..=29);
                let pick_two = rng.gen_range(0..=29);
                let pick_three = rng.gen_range(0..=29);
                let menu_one = menu_items.get(pick_one).unwrap();
                let menu_item_one: MenuResponseData = client
                    .get(format!("http://localhost/api/menu/{}", menu_one.id))
                    .header(header::AUTHORIZATION, format!("Bearer {}", user.token))
                    .send()
                    .await?
                    .json()
                    .await?;
                logger::log(logger::Header::INFO, &format!("{:?}", menu_item_one));

                let menu_two = menu_items.get(pick_two).unwrap();
                let menu_item_two: MenuResponseData = client
                    .get(format!("http://localhost/api/menu/{}", menu_two.id))
                    .header(header::AUTHORIZATION, format!("Bearer {}", user.token))
                    .send()
                    .await?
                    .json()
                    .await?;
                logger::log(logger::Header::INFO, &format!("{:?}", menu_item_two));

                let menu_three = menu_items.get(pick_three).unwrap();
                let menu_item_three: MenuResponseData = client
                    .get(format!("http://localhost/api/menu/{}", menu_three.id))
                    .header(header::AUTHORIZATION, format!("Bearer {}", user.token))
                    .send()
                    .await?
                    .json()
                    .await?;
                logger::log(logger::Header::INFO, &format!("{:?}", menu_item_three));
                logger::log(logger::Header::INFO, &format!("Menu IDs: {}, {}, {}", menu_item_one.id, menu_item_two.id, menu_item_three.id));

                client
                    .post("http://localhost/api/orders")
                    .header(header::AUTHORIZATION, format!("Bearer {}", user.token))
                    .json(
                        &(AddOrdersPostData {
                            restaurant_table_id: table.id,
                            menu_ids: vec![
                                menu_item_one.id,
                                menu_item_two.id,
                                menu_item_three.id,
                            ],
                        }),
                    )
                    .send()
                    .await?;
                logger::log(logger::Header::INFO, "Order success");

                let order_items: Vec<OrderResponseData> = client
                    .get(format!("http://localhost/api/table/{}/order", table.id))
                    .header(header::AUTHORIZATION, format!("Bearer {}", user.token))
                    .send()
                    .await?
                    .json()
                    .await?;
                logger::log(logger::Header::INFO, &format!("Table: {:?}",  table.id));
                logger::log(logger::Header::INFO, &format!("{:?}", order_items));

                let indices: Vec<usize> = (0..order_items.len()).collect();
                let selected_indices: Vec<&usize> = indices.choose_multiple(&mut rng, 2).collect();
            
                let rand_order_item_one = &order_items[*selected_indices[0]];
                let rand_order_item_two = &order_items[*selected_indices[1]];

                client
                    .delete(format!("http://localhost/api/order"))
                    .header(header::AUTHORIZATION, format!("Bearer {}", user.token))
                    .json(
                        &(DeleteOrderRequest {
                            order_id: rand_order_item_one.order_id.unwrap(),
                        }),
                    )
                    .send()
                    .await?;
                logger::log(logger::Header::INFO, "Successfully Canceled");

                client
                    .delete(format!("http://localhost/api/order/complete"))
                    .header(header::AUTHORIZATION, format!("Bearer {}", user.token))
                    .json(
                        &(CompleteOrderRequest {
                            order_id: rand_order_item_two.order_id.unwrap(),
                        }),
                    )
                    .send()
                    .await?;
                logger::log(logger::Header::INFO, "Successfully Served");

                client
                    .delete(format!("http://localhost/api/table/order"))
                    .header(header::AUTHORIZATION, format!("Bearer {}", user.token))
                    .json(
                        &(DeleteAllOrdersRequest {
                            restaurant_table_id: table.id,
                        }),
                    )
                    .send()
                    .await?;
                logger::log(logger::Header::INFO, "All orders deleted");
            }
            Ok::<(), reqwest::Error>(())
        });

        let results: Vec<Result<(), reqwest::Error>> = join_all(futures).await;

        for result in results {
            assert!(result.is_ok(), "Request failed");
        }

        Ok(())
    }
}
