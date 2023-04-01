///! Account representation for authentication
/// 
#[derive(Serialize)]
pub struct Account {
    pub id: &'static str,
    pub email: &'static str,
    pub password: &'static str,
    pub lastname: &'static str,
    pub firstname: &'static str,
    // is_superuser
    // disabled
    // created_date
    // deleted_date
    // last_login
    // last_ip
}
