use crate::*;

/// Pagination struct that is being extracted from the query params
/// NOTE: `start` and `end` do not relate to the ids used by the questions
#[derive(Debug, Deserialize, Default)]
pub struct Pagination {
    limit: Option<i32>,
    offset: i32,
}

/// Error struct used for matching custom errors
#[derive(Debug)]
pub enum Err {
    MissingParameters,
    ParseInt(std::num::ParseIntError),
    QuestionNotFound,
}

/// Implements error messages for the custom Error struct
impl std::fmt::Display for Err {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Err::ParseInt(ref err) => {
                write!(f, "Cannot parse parameter: {}", err)
            }
            Err::MissingParameters => write!(f, "Missing parameter"),
            Err::QuestionNotFound => write!(f, "Question not found"),
        }
    }
}

/// Extract query parameters from the `questions` route
fn extract_pagination(params: HashMap<String, String>) -> Result<Pagination, Err> {
    // Checks to see if the parameters passed contains the required fields
    if params.contains_key("limit") && params.contains_key("offset") {
        // Parse the arguments into integers, otherwise return an error
        return Ok(Pagination {
            limit: Some(params
                .get("limit")
                .unwrap()
                .parse::<i32>()
                .map_err(Err::ParseInt)?),
            offset: params
                .get("offset")
                .unwrap()
                .parse::<i32>()
                .map_err(Err::ParseInt)?,
        });
    }
    // If any of the required fields are missing, return an error
    Err(Err::MissingParameters)
}

/// Fetch questions from the `questions` route
/// # Example query
/// GET requests to this route can have a pagination attached so we just return the questions we need
/// `/questions?limit=3&offset=1`
pub async fn get_questions(
    State(store): State<Arc<RwLock<Store>>>,
    Query(params): Query<HashMap<String, String>>,
) -> Response {
    let mut pagination = Pagination::default();

    // If parameters are passed, parse them
    if !params.is_empty() {
        // Extract the parameters
        match extract_pagination(params) {
            // If the parameters are good, request read access to the database, get the questions,
            // and store them in a vector. Then return the specified slice in a response.
            Ok(p) => {
                pagination = p;
            }
            // If we get an error, return a response with an error message
            Err(Err::ParseInt(_)) => return (
                StatusCode::RANGE_NOT_SATISFIABLE,
                "Failed to parse range".to_string(),
            )
                .into_response(),
            Err(Err::MissingParameters) => return (
                StatusCode::RANGE_NOT_SATISFIABLE,
                "Missing parameters".to_string(),
            )
                .into_response(),
            Err(_) => return (StatusCode::BAD_REQUEST, "Bad request".to_string()).into_response(),
        }
    }
    // Get the questions
    let res: Vec<Question> = match store
        .read()
        .await
        .get_questions(pagination.limit, pagination.offset)
        .await {
            Ok(res) => res,
            Err(e) => return (StatusCode::BAD_REQUEST, e.to_string()).into_response(),
        };
    let res = &res;
    (StatusCode::OK, Json(res)).into_response()
    
}

// CREATE OPERATION

/// Create a new question in the `questions` based on a json body specifying the new data in the question
/// # Example query
/// POST requests to this route have an json body attached so we just create the question we need
/// `/questions`
/// `{
///     "id": "4",
///     "title": "New Question",
///     "content": "This is the contents of the new question",
///     "tags": ["sample", "tags", "example"]
/// }`
pub async fn add_question(
    State(store): State<Arc<RwLock<Store>>>,
    Json(question): Json<Question>,
) -> Response {
    store.write().await.add_question(question).await;
    (StatusCode::CREATED, "Question added".to_string()).into_response()
}

// READ OPERATION

/// Fetch a specific question from the `questions` route based on the id passed in the route
/// # Example query
/// GET requests to this route have an id attached so we just return the question we need
/// `/questions/3`
pub async fn get_question(
    State(store): State<Arc<RwLock<Store>>>,
    Path(id): Path<i32>,
) -> Response {
    // Request read access to the database and call its get_question method with specified id
    match store.read().await.get_question(&id).await {
        // If we get a good result, wrap the question in a json response
        Ok(q) => (StatusCode::OK, Json(q)).into_response(),
        Err(sqlx::Error::RowNotFound) => {
            (StatusCode::NOT_FOUND, "Question not found".to_string()).into_response()
        },
        Err(e) => (StatusCode::BAD_REQUEST, e.to_string()).into_response(),
    }
}

// UPDATE OPERATION

/// Update a specific question from the `questions` route based on the id passed in the route
/// and a json body specifying the new data in the question
/// # Example query
/// PUT requests to this route have an id attached so we just update the question we need with the
/// json body passed
/// `/questions/1`
/// `{
///     "id": "1",
///     "title": "Updated Question",
///     "content": "This is the new contents of the question",
///     "tags": ["sample", "tags", "example"]
/// }`
pub async fn update_question(
    State(store): State<Arc<RwLock<Store>>>,
    Path(id): Path<i32>,
    Json(question): Json<Question>,
) -> Response {
    // Request write access to the database and call its update_question method with specified id
    match store.write().await.update_question(&id, question).await {
        // If we get a good result, send a response informing the user
        Ok(_) => (StatusCode::OK, "Question updated".to_string()).into_response(),
        // If we get an error, return a response with an error message
        Err(Err::QuestionNotFound) => {
            (StatusCode::NOT_FOUND, "Question not found".to_string()).into_response()
        }
        Err(_) => (StatusCode::BAD_REQUEST, "Bad request".to_string()).into_response(),
    }
}

// DELETE OPERATION

/// Delete a specific question from the `questions` route based on the id passed in the route
/// # Example query
/// DELETE requests to this route have an id attached so we just delete the question we need
/// `/questions/3`
pub async fn delete_question(
    State(store): State<Arc<RwLock<Store>>>,
    Path(id): Path<i32>,
) -> Response {
    // Request write access to the database and call its delete_question method with specified id
    match store.write().await.delete_question(&id).await {
        // If we get a good result, send a response informing the user
        Ok(_) => (StatusCode::OK, "Question deleted".to_string()).into_response(),
        // If we get an error, return a response with an error message
        Err(Err::QuestionNotFound) => {
            (StatusCode::NOT_FOUND, "Question not found".to_string()).into_response()
        }
        Err(_) => (StatusCode::BAD_REQUEST, "Bad request".to_string()).into_response(),
    }
}
