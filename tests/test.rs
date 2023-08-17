use rust_sqlx_example::{
    create_pool, create_user, delete_user, get_users, truncate_table, update_user,
    CreateUserRequest, UpdateUserRequest,
};

#[tokio::test]
async fn test() -> Result<(), sqlx::Error> {
    let pool = create_pool().await?;
    truncate_table(&pool, "users").await?;

    let users = get_users(&pool).await?;
    assert_eq!(users.len(), 0);

    let req1 = CreateUserRequest {
        name: "John".to_string(),
        email: "john@example.com".to_string(),
        address: None,
    };
    let new_id = create_user(&pool, req1).await?;

    let users1 = get_users(&pool).await?;
    assert_eq!(users1.len(), 1);

    let user1 = users1.get(0).unwrap();
    assert_eq!(user1.id, new_id);
    assert_eq!(user1.name, "John".to_string());
    assert_eq!(user1.email, "john@example.com".to_string());
    assert_eq!(user1.address, None);

    let req2 = UpdateUserRequest {
        name: None,
        email: None,
        address: Some("tokyo".to_string()),
    };
    let updated_count = update_user(&pool, user1.id, req2).await?;
    assert_eq!(updated_count, 1);

    let users2 = get_users(&pool).await?;
    assert_eq!(users2.len(), 1);

    let user2 = users2.get(0).unwrap();
    assert_eq!(user2.id, user1.id);
    assert_eq!(user2.name, user1.name);
    assert_eq!(user2.email, user1.email);
    assert_eq!(user2.address, Some("tokyo".to_string()));

    let deleted_count = delete_user(&pool, user2.id).await?;
    assert_eq!(deleted_count, 1);

    let users3 = get_users(&pool).await?;
    assert_eq!(users3.len(), 0);

    Ok(())
}
