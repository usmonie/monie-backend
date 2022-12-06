use diesel::Queryable;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Queryable)]
pub struct Story {
    pub id: String,
    pub author_id: String,
    pub created_at: u64,
    pub text: Option<String>,
    pub scene_id: String,
    pub scene_type: u8,
    pub soundtrack_id: String,
}


// impl Story {
//     pub fn load_stories_by_plot(user_id: String, plot_id: String) -> Result<Vec<Self>, Error> {
//         let conn = db::connection()?;
//         // let stories = stories::table.load::<Story>(&conn)?;
//         // Ok(stories)
//     }
// }