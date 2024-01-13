use actix_web::web;

use crate::handlers::{
    // base
    index,
    about,
    raw_index,
    search,
    creature_by_location,

    //about,
    toggle_language,
    toggle_language_index,
    toggle_language_two,
    toggle_language_three,

    // admin
    admin_edit_user,
    admin_edit_user_post,

    // errors
    internal_server_error,
    not_found,

    // password reset
    request_password_reset,
    request_password_reset_post,
    password_email_sent,
    password_reset,
    password_reset_post,

    // login
    login_handler,
    login_form_input,
    logout,
    
    // registration
    register_form_input,
    register_handler,
    registration_error,

    // email validation
    email_verification,
    resend_email_verification,
    verify_code,

    // users
    user_index,
    user_page_handler,
    edit_user,
    edit_user_post,
    delete_user,
    delete_user_handler,

    //creatures
    get_creature,
    new_creature_form,
    edit_creature,
    post_creature,
    edit_creature_post,
    copy_creature,

    // attacks
    get_attack,
    get_attacks,
    add_attack,
    post_attack,
    edit_attack,
    edit_attack_post,
    delete_attack,

    // powers
    get_power,
    get_powers,
    add_power,
    post_power,
    edit_power,
    edit_power_post,
    delete_power,

    // maneuvers
    get_maneuver,
    get_maneuvers,
    add_maneuver,
    post_maneuver,
    edit_maneuver,
    edit_maneuver_post,
    delete_maneuver,
};

pub fn configure_services(config: &mut web::ServiceConfig) {
    config.service(index);
    config.service(about);
    config.service(raw_index);
    config.service(search);
    config.service(creature_by_location);

    //config.service(about);
    config.service(toggle_language);
    config.service(toggle_language_index);
    config.service(toggle_language_two);
    config.service(toggle_language_three);

    // admin
    config.service(admin_edit_user);
    config.service(admin_edit_user_post);

    // errors
    config.service(internal_server_error);
    config.service(not_found);

    // forgot password
    config.service(request_password_reset);
    config.service(request_password_reset_post);
    config.service(password_email_sent);
    config.service(password_reset);
    config.service(password_reset_post);
 
    // login and logout
    config.service(login_handler);
    config.service(login_form_input);
    config.service(logout);

    // registration and validation
    config.service(register_handler);
    config.service(register_form_input);
    config.service(registration_error);
    config.service(email_verification);
    config.service(resend_email_verification);
    config.service(verify_code);
     
     // users 
     config.service(user_page_handler);
     config.service(user_index);
     config.service(edit_user);
     config.service(edit_user_post);
     config.service(delete_user);
     config.service(delete_user_handler);

     //creatures
     config.service(get_creature);
     config.service(new_creature_form);
     config.service(post_creature);
     config.service(edit_creature);
     config.service(edit_creature_post);
     config.service(copy_creature);


    // attacks
    config.service(get_attacks);
    config.service(get_attack);
    config.service(add_attack);
    config.service(post_attack);
    config.service(edit_attack);
    config.service(edit_attack_post);
    config.service(delete_attack);

    // powers
    config.service(get_powers);
    config.service(get_power);
    config.service(add_power);
    config.service(post_power);
    config.service(edit_power);
    config.service(edit_power_post);
    config.service(delete_power);

    // maneuvers
    config.service(get_maneuvers);
    config.service(get_maneuver);
    config.service(add_maneuver);
    config.service(post_maneuver);
    config.service(edit_maneuver);
    config.service(edit_maneuver_post);
    config.service(delete_maneuver);

}
