use std::collections::HashMap;
use std::sync::Mutex;

use lazy_static::lazy_static;

use crate::domain::models::plot::PlotCore;

lazy_static! {
    static ref TIMELINE: Mutex<HashMap<String, PlotCore>> = {
        let m = HashMap::new();
        Mutex::new(m)
    };
}

pub fn get_current_user_timeline() {
    
}
