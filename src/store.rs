use crate::*;

/// Store struct that has a connection to a database
#[derive(Clone)]
pub struct Store {
    pub connection: PgPool,
}

impl Store {
    /// Constructor to create a datastore and connect to the database
    pub async fn new() -> Result<Self, Box<dyn Error>> {
        use std::env::var;

        // Get all of the environment variables and prepare the url
        let file = var("PG_PASSWORD_FILE")?;
        let password = std::fs::read_to_string(file)?;
        let url = format!(
            "postgres://{}:{}@{}:5432/{}",
            var("PG_USER")?,
            password.trim(),
            var("PG_HOST")?,
            var("PG_DBNAME")?,
        );

        // Connect to the database
        let pool = PgPool::connect(&url).await?;

        // Run the migration files (in the 'migrations' directory)
        // migrate!() will search the directory with the .toml file for the 'migrations' directory
        sqlx::migrate!().run(&pool).await?;

        // Return the data store with a connection to the database
        Ok(Store { connection: pool })
    }

    // Questions

    /// Add a given question to database
    pub async fn add_question(&mut self, new_question: NewQuestion) -> Result<(), sqlx::Error> {
        // Create a transaction so that the operation will be atomic since we are modifying the db
        let mut transaction = self.connection.begin().await?;

        // Write and execute the query
        match sqlx::query(
            "INSERT INTO questions (title, content, tags)
                VALUES ($1, $2, $3);",
        )
            .bind(new_question.title)
            .bind(new_question.content)
            .bind(new_question.tags)
            .execute(&mut *transaction)
            .await
        // Match the results from the query and commit the query if ok
        {
            Ok(_) => {
                transaction.commit().await?;
                Ok(())
            }
            Err(e) => {
                tracing::event!(tracing::Level::ERROR, "{:?}", e);
                Err(e)
            }
        }
    }

    /// Get items from the database, apply a limit and offset if applicable
    pub async fn get_questions(
        &self,
        limit: Option<i32>,
        offset: i32,
    ) -> Result<Vec<Question>, sqlx::Error> {
        // Write and execute the query
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
        // Match the results from the query and return the questions if ok
        {
            Ok(questions) => Ok(questions),
            Err(e) => {
                tracing::event!(tracing::Level::ERROR, "{:?}", e);
                Err(e)
            }
        }
    }

    /// Get an item from the database given a specified id
    pub async fn get_question(&self, id: &i32) -> Result<Question, sqlx::Error> {
        // Write and execute the query
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
        // Match the results from the query and return the question if ok
        {
            Ok(q) => Ok(q),
            Err(e) => {
                tracing::event!(tracing::Level::ERROR, "{:?}", e);
                Err(e)
            }
        }
    }

    /// Update a question in the database given a specified id and new data
    pub async fn update_question(
        &mut self,
        id: &i32,
        new_question: NewQuestion,
    ) -> Result<(), sqlx::Error> {
        // Create a transaction so that the operation will be atomic since we are modifying the db
        let mut transaction = self.connection.begin().await?;

        // Write and execute the query
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
        // Match the results from the query and commit the query if ok
        {
            Ok(_) => {
                transaction.commit().await?;
                Ok(())
            }
            Err(e) => {
                tracing::event!(tracing::Level::ERROR, "{:?}", e);
                Err(e)
            }
        }
    }

    /// Delete a question from the database fr given a specified id
    pub async fn delete_question(&mut self, id: &i32) -> Result<(), sqlx::Error> {
        // Create a transaction so that the operation will be atomic since we are modifying the db
        let mut transaction = self.connection.begin().await?;

        // Write and execute the query
        match sqlx::query("DELETE FROM questions WHERE id = $1;")
            .bind(id)
            .execute(&mut *transaction)
            .await
        // Match the results from the query and commit the query if ok
        {
            Ok(_) => {
                transaction.commit().await?;
                Ok(())
            }
            Err(e) => {
                tracing::event!(tracing::Level::ERROR, "{:?}", e);
                Err(e)
            }
        }
    }

    // Answers

    /// Get items from the database, apply a limit and offset if applicable
    pub async fn get_answers(
        &self,
        limit: Option<i32>,
        offset: i32,
    ) -> Result<Vec<Answer>, sqlx::Error> {
        // Write and execute the query
        match sqlx::query("SELECT * FROM answers LIMIT $1 OFFSET $2;")
            .bind(limit)
            .bind(offset)
            .map(|row: PgRow| Answer {
                id: row.get("id"),
                content: row.get("content"),
                corresponding_question: row.get("corresponding_question"),
            })
            .fetch_all(&self.connection)
            .await
        // Match the results from the query and return the answers if ok
        {
            Ok(answers) => Ok(answers),
            Err(e) => {
                tracing::event!(tracing::Level::ERROR, "{:?}", e);
                Err(e)
            }
        }
    }

    /// Add a given answer to database
    pub async fn add_answer(&mut self, new_answer: NewAnswer) -> Result<(), sqlx::Error> {
        // Create a transaction so that the operation will be atomic since we are modifying the db
        let mut transaction = self.connection.begin().await?;

        // Write and execute the query
        match sqlx::query(
            "INSERT INTO answers (content, corresponding_question)
                VALUES ($1, $2);",
        )
            .bind(new_answer.content)
            .bind(new_answer.corresponding_question)
            .execute(&mut *transaction)
            .await
        // Match the results from the query and commit the query if ok
        {
            Ok(_) => {
                transaction.commit().await?;
                Ok(())
            }
            Err(e) => {
                tracing::event!(tracing::Level::ERROR, "{:?}", e);
                Err(e)
            }
        }
    }
}
