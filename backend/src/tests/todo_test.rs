#[allow(dead_code)]
#[cfg(test)]
mod tests {
    use crate::api::jwt::jwt::Claims;
    use crate::api::services::todo_service::TodoService;
    use crate::db::models::todo::*;
    use crate::errors::todo_error::TodoError;
    use async_trait::async_trait;
    use chrono::NaiveDateTime;
    use mockall::mock;

    mock! {
        pub TodoService {}
        #[async_trait]
        impl TodoService for TodoService{
            async fn get_todos(&self, user: Claims) -> Result<Vec<TodoItem>, TodoError>;
            async fn create_todo(&self, user: Claims, todo_req: &RequestCreateTodoItem) -> Result<ResponseCreateTodoItem, TodoError>;
            async fn update_todo(&self, user: Claims, todo_req: &RequestUpdateTodoItem) -> Result<(), TodoError>;
            async fn delete_todo(&self, user: Claims, todo_req: &RequestDeleteTodoItem) -> Result<(), TodoError>;
            async fn complete_todo(&self, user: Claims, todo_req: &RequestCompleteTodoItem) -> Result<(), TodoError>;
        }
    }

    // TODO取得
    #[actix_rt::test]
    async fn test_get_todos() -> Result<(), TodoError> {
        let mut mock_service = MockTodoService::new();

        let deadline =
            NaiveDateTime::parse_from_str("2024-09-13T11:00:00", "%Y-%m-%dT%H:%M:%S").unwrap();
        let created_at = deadline;
        let updated_at = deadline;
        let deleted_at = deadline;

        let mock_todos = vec![TodoItem {
            id: 1,
            user_id: Some(1),
            game_id: Some(1),
            title: "test".to_string(),
            description: "test".to_string(),
            is_completed: false,
            status: Some(1),
            priority: Some(1),
            difficulty: Some(1),
            deadline: Some(deadline),
            created_at,
            updated_at,
            deleted_at: Some(deleted_at),
        }];

        let mock_todos_clone = mock_todos.clone();

        mock_service
            .expect_get_todos()
            .returning(move |_| Ok(mock_todos_clone.clone()));

        let user = Claims {
            id: 1,
            sub: "test_user".to_string(),
            exp: 1239,
        };

        let result = mock_service.get_todos(user).await;

        assert!(result.is_ok());

        let todos = result.unwrap();
        assert_eq!(todos, mock_todos);

        Ok(())
    }

    // TODO追加
    #[actix_rt::test]
    async fn test_create_todo() -> Result<(), TodoError> {
        let mut mock_service = MockTodoService::new();

        let request_create_todo = RequestCreateTodoItem {
            title: "test".to_string(),
            description: "test".to_string(),
        };

        let response_create_todo = ResponseCreateTodoItem {
            title: "test".to_string(),
            description: "test".to_string(),
            is_completed: false,
        };

        let request_create_todo_clone = request_create_todo.clone();
        let response_create_todo_clone = response_create_todo.clone();

        mock_service
            .expect_create_todo()
            .returning(move |_, todo_req| {
                if todo_req == &request_create_todo_clone {
                    Ok(response_create_todo_clone.clone())
                } else {
                    Err(TodoError::DatabaseError("db error".to_string()))
                }
            });

        let user = Claims {
            id: 1,
            sub: "test_user".to_string(),
            exp: 1239,
        };

        let result = mock_service.create_todo(user, &request_create_todo).await;

        assert!(result.is_ok());

        let response = result.unwrap();
        assert_eq!(response, response_create_todo);

        Ok(())
    }

    // TODO更新
    #[actix_rt::test]
    async fn test_update_todo() -> Result<(), TodoError> {
        let mut mock_service = MockTodoService::new();

        let updated_at =
            NaiveDateTime::parse_from_str("2024-09-13T11:00:00", "%Y-%m-%dT%H:%M:%S").unwrap();

        let request_update_todo = RequestUpdateTodoItem {
            id: 1,
            title: Some("test".to_string()),
            description: Some("test".to_string()),
            is_completed: Some(true),
            updated_at,
        };

        let request_update_todo_clone = request_update_todo.clone();

        mock_service
            .expect_update_todo()
            .returning(move |_, todo_req| {
                if todo_req == &request_update_todo_clone {
                    Ok(())
                } else {
                    Err(TodoError::DatabaseError("db error".to_string()))
                }
            });

        let user = Claims {
            id: 1,
            sub: "test_user".to_string(),
            exp: 1239,
        };

        let result = mock_service.update_todo(user, &request_update_todo).await;

        assert!(result.is_ok());

        Ok(())
    }

    // TODO削除
    #[actix_rt::test]
    async fn test_delete_todo() -> Result<(), TodoError> {
        let mut mock_service = MockTodoService::new();

        let request_delete_todo = RequestDeleteTodoItem { id: 1 };

        let request_delete_todo_clone = request_delete_todo.clone();

        mock_service
            .expect_delete_todo()
            .returning(move |_, todo_req| {
                if todo_req == &request_delete_todo_clone {
                    Ok(())
                } else {
                    Err(TodoError::DatabaseError("db error".to_string()))
                }
            });

        let user = Claims {
            id: 1,
            sub: "test_user".to_string(),
            exp: 1239,
        };

        let result = mock_service.delete_todo(user, &request_delete_todo).await;

        assert!(result.is_ok());

        Ok(())
    }

    // TODOステータスを更新
    #[actix_rt::test]
    async fn test_complete_todo() -> Result<(), TodoError> {
        let mut mock_service = MockTodoService::new();

        let request_complete_todo = RequestCompleteTodoItem { id: 1 };

        let request_complete_todo_clone = request_complete_todo.clone();

        mock_service
            .expect_complete_todo()
            .returning(move |_, todo_req| {
                if todo_req == &request_complete_todo_clone {
                    Ok(())
                } else {
                    Err(TodoError::DatabaseError("db error".to_string()))
                }
            });

        let user = Claims {
            id: 1,
            sub: "test_user".to_string(),
            exp: 1239,
        };

        let result = mock_service
            .complete_todo(user, &request_complete_todo)
            .await;

        assert!(result.is_ok());

        Ok(())
    }
}
