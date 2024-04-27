use crate::*;

// Pagination
#[derive(Debug, Deserialize)]
pub struct Pagination {
    start: usize,
    end: usize,
}

fn extract_pagination(params: HashMap<String, String>) -> Result<Pagination, Error> {
    if params.contains_key("start") && params.contains_key("end") {
        return Ok(Pagination {
            start: params
                .get("start")
                .unwrap()
                .parse::<usize>()
                .map_err(Error::ParseInt)?,
            end: params
                .get("end")
                .unwrap()
                .parse::<usize>()
                .map_err(Error::ParseInt)?,
        });
    }
    Err(Error::MissingParameters)
}

pub async fn get_questions(
    State(store): State<Arc<RwLock<Store>>>,
    Query(params): Query<HashMap<String, String>>,
) -> Response {
    if !params.is_empty() {
        match extract_pagination(params) {
            Ok(p) => {
                let pagination = p;

                let res: Vec<Question> = store
                    .read()
                    .await
                    .get_questions()
                    .values()
                    .cloned()
                    .collect();

                let res = &res[pagination.start..pagination.end];
                (StatusCode::CREATED, Json(res)).into_response()
            }
            Err(Error::ParseInt(_)) => (
                StatusCode::RANGE_NOT_SATISFIABLE,
                "Failed to parse range".to_string(),
            )
                .into_response(),
            Err(Error::MissingParameters) => (
                StatusCode::RANGE_NOT_SATISFIABLE,
                "Missing parameters".to_string(),
            )
                .into_response(),
            Err(_) => (StatusCode::BAD_REQUEST, "Bad request".to_string()).into_response(),
        }
    } else {
        let res: Vec<Question> = store
            .read()
            .await
            .get_questions()
            .values()
            .cloned()
            .collect();

        let res = &res;
        (StatusCode::CREATED, Json(res)).into_response()
    }
}

// Create
// Handler method for creating a question, given input from a request
pub async fn add_question(
    State(store): State<Arc<RwLock<Store>>>,
    Json(question): Json<Question>,
) -> Response {
    store.write().await.add_question(question);
    (StatusCode::CREATED, "Question added".to_string()).into_response()
}

// Read
// Handler method for getting a specific question, given the id from a request
pub async fn get_question(
    State(store): State<Arc<RwLock<Store>>>,
    Path(id): Path<String>,
) -> Response {
    match store.read().await.get_question(&id) {
        Ok(q) => (StatusCode::OK, Json(q)).into_response(),
        Err(Error::QuestionNotFound) => {
            (StatusCode::NOT_FOUND, "Question not found".to_string()).into_response()
        }
        Err(_) => (StatusCode::BAD_REQUEST, "Bad request".to_string()).into_response(),
    }
}

// Update
pub async fn update_question(
    State(store): State<Arc<RwLock<Store>>>,
    Path(id): Path<String>,
    Json(question): Json<Question>,
) -> Response {
    match store.write().await.update_question(&id, question) {
        Ok(_) => (StatusCode::OK, "Question updated".to_string()).into_response(),
        Err(Error::QuestionNotFound) => {
            (StatusCode::NOT_FOUND, "Question not found".to_string()).into_response()
        }
        Err(_) => (StatusCode::BAD_REQUEST, "Bad request".to_string()).into_response(),
    }
}

// Delete
pub async fn delete_question(
    State(store): State<Arc<RwLock<Store>>>,
    Path(id): Path<String>,
) -> Response {
    match store.write().await.delete_question(&id) {
        Ok(_) => (StatusCode::OK, "Question deleted".to_string()).into_response(),
        Err(Error::QuestionNotFound) => {
            (StatusCode::NOT_FOUND, "Question not found".to_string()).into_response()
        }
        Err(_) => (StatusCode::BAD_REQUEST, "Bad request".to_string()).into_response(),
    }
}
