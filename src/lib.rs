extern crate core;

pub mod presentation {
    pub mod controllers {
        pub mod posts {
            pub mod post_controller;
            pub mod routes {
                pub mod get_post;
                pub mod add_post;
                pub mod delete_post;
                pub mod update_route;
            }
        }
    }
}

pub mod entities {
    pub mod post;
    pub mod errors {
        pub mod db_errors;
    }
}

pub mod repository {
    pub mod posts {
        pub mod post_repository;

        pub mod methods {
            pub mod add;
            pub mod contains;
            pub mod delete;
            pub mod get;
            pub mod update;
        }
    }
}

pub mod services {
    pub mod use_cases {
        pub mod add_post;
        pub mod delete_post;
        pub mod get_post;
        pub mod update;
    }
}