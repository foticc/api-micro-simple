use actix_web::web;

mod menu_api;
mod department_api;
mod auth_api;
mod user_api;
mod role_api;
mod permission_api;

pub fn dispatch(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/menu")
            .service(menu_api::list)
            .service(menu_api::create)
            .service(menu_api::find_one)
            .service(menu_api::update)
            .service(menu_api::delete)
    );

    cfg.service(
        web::scope("/auth")
            .service(auth_api::sign_in)
            .service(auth_api::sign_out)
            .service(auth_api::get_menu_by_user_auth_code)
    );

    cfg.service(
        web::scope("/user")
            .service(user_api::find_one_auth_code)
            .service(user_api::list)
            .service(user_api::find_one)
            .service(user_api::create)
            .service(user_api::update)
            .service(user_api::modify_psd)
    );


    cfg.service(
        web::scope("/department")
            .service(department_api::list)
            .service(department_api::create)
            .service(department_api::delete)
    );

    cfg.service(
        web::scope("/role")
            .service(role_api::list)
            .service(role_api::create)
            .service(role_api::find_one)
            .service(role_api::update)
            .service(role_api::delete)
    );


    cfg.service(
        web::scope("/permission")
            .service(permission_api::get_menus_permission_by_role_id)
            .service(permission_api::assign_role_perm_code)
    );

}