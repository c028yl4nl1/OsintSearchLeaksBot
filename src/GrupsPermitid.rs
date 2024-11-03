pub fn grupos_permitidos(id_user: i64) -> bool {
    let ids = [
        -1002464835404,
        -1002463757773,
        -1002236697784,
        -1002219285605,
    ];
    for id in ids {
        if id == id_user {
            return true;
        }
    }
    false
}
