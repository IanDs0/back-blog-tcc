use uuid::Uuid;

pub fn find_user_by_id(id: Uuid) -> String{//User {
    "Você está na rota /user/".to_owned()+&id.to_string()
}