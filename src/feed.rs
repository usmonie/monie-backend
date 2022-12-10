use actix_web::{get, Responder, Result, web};

use crate::domain::models::author::AuthorCore;
use crate::domain::models::plot::Plot;

#[get("/feed")]
pub async fn feed() -> Result<impl Responder> {
    Ok(
        web::Json(
            vec![
                Plot {
                    id: "test_id_plot_1".to_string(),
                    author: Box::new(AuthorCore {
                        id: "author_id_1".to_string(),
                        name: "Usman Akhmedov".to_string(),
                        username: "usmonie".to_string(),
                        avatar: None,
                    }),
                    title: Some("Road to California".to_string()),
                    description: Some("Road to California from Texas with my best friends.".to_string()),
                    soundtrack: None,
                    stories: Box::new(vec![]),
                }
            ]
        )
    )
}