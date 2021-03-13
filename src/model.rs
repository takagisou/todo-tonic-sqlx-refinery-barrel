use sqlx::mysql::MySqlPool;
use crate::pb::TodoResponse;

#[derive(sqlx::FromRow, Debug)]
pub struct Todo {
    pub id: i32, // By default, using barrel's types::primary() results in i32
    pub title: String,
    pub is_completed: i8,
}

type DBResult<T> = Result<T, Box<dyn std::error::Error>>;

// TODO: cleanup and re-format query
impl Todo {

    pub async fn add(pool: &MySqlPool, title: String) -> DBResult<TodoResponse> {
        let todo_id = sqlx::query!(
        r#"
        INSERT INTO todos ( title )
        VALUES ( ? )
        "#,
        title
        )
            .execute(pool)
            .await?
            .last_insert_id();

        // let todo = sqlx::query_as!(Todo, "INSERT INTO todos (title) VALUES ( $1 ) RETURNING id, title, is_completed", title)
        //     .fetch_one(pool)
        //     .await?;
        // Ok(todo.into_response())

        let todo = sqlx::query_as!(Todo, "SELECT id, title, is_completed from todos WHERE id = ( ? )", todo_id)
            .fetch_one(pool)
            .await?;
        Ok(todo.into_response())
    }

    // TODO: Is it a heavy operation to convert Todo into TodoResponse?
    pub async fn all(pool: &MySqlPool) -> DBResult<Vec<TodoResponse>> {
        let todos: Vec<Todo> = sqlx::query_as!(Todo, "SELECT id, title, is_completed FROM todos ORDER by id")
            .fetch_all(pool)
            .await?;
        let todo_responses = todos.iter().map(|t| t.into_response()).collect();
        Ok(todo_responses)
    }

    // TODO: Is it a heavy operation to convert Todo into TodoResponse?
    pub async fn incomplete(pool: &MySqlPool) -> DBResult<Vec<TodoResponse>> {
        let todos: Vec<Todo> = sqlx::query_as!(Todo, "SELECT id, title, is_completed FROM todos WHERE is_completed = false ORDER by id")
            .fetch_all(pool)
            .await?;
        let todo_responses = todos.iter().map(|t| t.into_response()).collect();
        Ok(todo_responses)

    }

    pub async fn get(pool: &MySqlPool, id: i32) -> DBResult<TodoResponse> {
        let todo = sqlx::query_as!(Todo, "SELECT id, title, is_completed from todos WHERE id = ( ? )", id)
            .fetch_one(pool)
            .await?;
        Ok(todo.into_response())
    }

    pub async fn mark_complete(pool: &MySqlPool, id: i32) -> DBResult<TodoResponse> {
        let _result = sqlx::query!(
        r#"
        UPDATE todos
        SET is_completed = TRUE
        WHERE id = ( ? )
        "#,
        id)
            .execute(pool)
            .await?;

        let todo = sqlx::query_as!(Todo, "SELECT id, title, is_completed from todos WHERE id = ( ? )", id)
            .fetch_one(pool)
            .await?;
        Ok(todo.into_response())

        //
        //
        // "UPDATE todos set is_completed = 1 where id = ( ? )", id)
        //     .fetch_one(pool)
        //     .await?;
        // Ok(todo.into_response())

    }

    // Mapping Todo struct with TodoResponse (which is used to display data in GRPC)
    // TODO: Is there a better way to do it?
    fn into_response(&self) -> TodoResponse {
        TodoResponse {
            id: self.id,
            title: self.title.clone(),
            is_completed: From::from(self.is_completed)
        }

    }
}
