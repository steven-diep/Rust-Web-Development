use crate::*;

/// Store struct that is being used as an in-memory storage for questions
#[derive(Clone)]
pub struct Store {
    pub questions: HashMap<i32, Question>,
    pub connection: PgPool,
}

impl Store {
    /// Constructor to create a new in-memory storage
    pub async fn new() -> Result<Self, Box<dyn Error>> {
        use std::env::var;

        let file = var("PG_PASSWORD_FILE")?;
        let password = std::fs::read_to_string(file)?;
        let url = format!(
            "postgres://{}:{}@{}:5432/{}",
            var("PG_USER")?,
            password.trim(),
            var("PG_HOST")?,
            var("PG_DBNAME")?,
        );

        let pool = PgPool::connect(&url).await?;
        sqlx::migrate!().run(&pool).await?;
        Ok(Store {
            questions: HashMap::new(),
            connection: pool,
        })
    }

    /// Add a given question to the hash map
    pub async fn add_question(&mut self, new_question: NewQuestion) -> Result<(), sqlx::Error> {
        let mut transaction = self.connection.begin().await?;
        match sqlx::query(
            "INSERT INTO questions (title, content, tags)
                VALUES ($1, $2, $3);",
        )
        .bind(new_question.title)
        .bind(new_question.content)
        .bind(new_question.tags)
        .execute(&mut *transaction)
        .await
        {
            Ok(_) => {
                transaction.commit().await?;
                Ok(())
            }
            Err(e) => Err(e),
        }
    }

    /// Return a reference to the entire hash map
    pub async fn get_questions(
        &self,
        limit: Option<i32>,
        offset: i32,
    ) -> Result<Vec<Question>, sqlx::Error> {
        match sqlx::query("SELECT * FROM questions LIMIT $1 OFFSET $2;")
            .bind(limit)
            .bind(offset)
            .map(|row: PgRow| Question {
                id: row.get("id"),
                title: row.get("title"),
                content: row.get("content"),
                tags: row.get("tags"),
            })
            .fetch_all(&self.connection)
            .await
        {
            Ok(questions) => Ok(questions),
            Err(e) => Err(e),
        }
    }

    /// Return a reference to a question given a specified id
    pub async fn get_question(&self, id: &i32) -> Result<Question, sqlx::Error> {
        match sqlx::query("SELECT * FROM questions WHERE id = $1;")
            .bind(id)
            .map(|row: PgRow| Question {
                id: row.get("id"),
                title: row.get("title"),
                content: row.get("content"),
                tags: row.get("tags"),
            })
            .fetch_one(&self.connection)
            .await
        {
            Ok(q) => Ok(q),
            Err(e) => Err(e),
        }
    }

    /// Update a question given a specified id and new data
    pub async fn update_question(&mut self, id: &i32, new_question: NewQuestion) -> Result<(), sqlx::Error> {
        let mut transaction = self.connection.begin().await?;
        match sqlx::query(
            "UPDATE questions 
                SET title = $1, content = $2, tags = $3
                WHERE id = $4;",
        )
        .bind(new_question.title)
        .bind(new_question.content)
        .bind(new_question.tags)
        .bind(id)
        .execute(&mut *transaction)
        .await
        {
            Ok(_) => {
                transaction.commit().await?;
                Ok(())
            }
            Err(e) => Err(e),
        }
    }

    /// Delete a question given a specified id
    pub async fn delete_question(&mut self, id: &i32) -> Result<(), sqlx::Error> {
        let mut transaction = self.connection.begin().await?;
        match sqlx::query(
            "DELETE FROM questions WHERE id = $1;",
        )
        .bind(id)
        .execute(&mut *transaction)
        .await
        {
            Ok(_) => {
                transaction.commit().await?;
                Ok(())
            }
            Err(e) => Err(e),
        }
    }
}
