pub mod preprocessor;
use paste::paste;
use rocket::Route;

macro_rules! import_page {
    ($module:ident) => {
        mod $module;
        paste! {
            use $module::$module as [ < $module _page > ];
        }
    };
}

import_page!(gayness);
import_page!(links);
import_page!(projects);
import_page!(index);
import_page!(snake);
import_page!(pay_me);

pub fn return_routes() -> Vec<Route> {
    routes![
        pay_me_page,
        index_page,
        projects_page,
        links_page,
        gayness_page,
        snake_page
    ]
}
