use crate::*;

/// Pagination struct that is being extracted from the query params
/// NOTE: `start` and `end` do not relate to the ids used by the questions
#[derive(Debug, Deserialize)]
pub struct Pagination {
    start: usize,
    end: usize,
}

/// Error struct used for matching custom errors
#[derive(Debug)]
pub enum Error {
    ParseInt(std::num::ParseIntError),
    MissingParameters,
    QuestionNotFound,
}

/// Implements error messages for the custom Error struct
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Error::ParseInt(ref err) => {
                write!(f, "Cannot parse parameter: {}", err)
            }
            Error::MissingParameters => write!(f, "Missing parameter"),
            Error::QuestionNotFound => write!(f, "Question not found"),
        }
    }
}

/// Extract query parameters from the `questions` route
fn extract_pagination(params: HashMap<String, String>) -> Result<Pagination, Error> {
    // Checks to see if the parameters passed contains the required fields
    if params.contains_key("start") && params.contains_key("end") {
        // Parse the arguments into integers, otherwise return an error
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
    // If any of the required fields are missing, return an error
    Err(Error::MissingParameters)
}

/// Fetch questions from the `questions` route
/// # Example query
/// GET requests to this route can have a pagination attached so we just return the questions we need
/// `/questions?start=1&end=3`
pub async fn get_questions(
    State(store): State<Arc<RwLock<Store>>>,
    Query(params): Query<HashMap<String, String>>,
) -> Response {
    // If parameters are passed, parse them
    if !params.is_empty() {
        // Extract the parameters
        match extract_pagination(params) {
            // If the parameters are good, request read access to the database, get the questions,
            // and store them in a vector. Then return the specified slice in a response.
            Ok(p) => {
                let pagination = p;

                let res: Vec<Question> = store
                    .read()
                    .await
                    .get_questions()
                    .values()
                    .cloned()
                    .collect();

                // Edge cases to make sure the values in pagination make sense
                if pagination.start > pagination.end {
                    return (
                        StatusCode::RANGE_NOT_SATISFIABLE,
                        "Invalid range passed".to_string(),
                    )
                        .into_response();
                }

                if pagination.end > res.len() {
                    return (
                        StatusCode::RANGE_NOT_SATISFIABLE,
                        "Range is greater than length".to_string(),
                    )
                        .into_response();
                }

                let res = &res[pagination.start..pagination.end];
                (StatusCode::OK, Json(res)).into_response()
            }
            // If we get an error, return a response with an error message
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
    }
    // If no parameters are passed, return the entire database
    else {
        let res: Vec<Question> = store
            .read()
            .await
            .get_questions()
            .values()
            .cloned()
            .collect();

        let res = &res;
        (StatusCode::OK, Json(res)).into_response()
    }
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
    store.write().await.add_question(question);
    (StatusCode::CREATED, "Question added".to_string()).into_response()
}

// READ OPERATION

/// Fetch a specific question from the `questions` route based on the id passed in the route
/// # Example query
/// GET requests to this route have an id attached so we just return the question we need
/// `/questions/3`
pub async fn get_question(
    State(store): State<Arc<RwLock<Store>>>,
    Path(id): Path<String>,
) -> Response {
    // Request read access to the database and call its get_question method with specified id
    match store.read().await.get_question(&id) {
        // If we get a good result, wrap the question in a json response
        Ok(q) => (StatusCode::OK, Json(q)).into_response(),
        // If we get an error, return a response with an error message
        Err(Error::QuestionNotFound) => {
            (StatusCode::NOT_FOUND, "Question not found".to_string()).into_response()
        }
        Err(_) => (StatusCode::BAD_REQUEST, "Bad request".to_string()).into_response(),
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
    Path(id): Path<String>,
    Json(question): Json<Question>,
) -> Response {
    // Request write access to the database and call its update_question method with specified id
    match store.write().await.update_question(&id, question) {
        // If we get a good result, send a response informing the user
        Ok(_) => (StatusCode::OK, "Question updated".to_string()).into_response(),
        // If we get an error, return a response with an error message
        Err(Error::QuestionNotFound) => {
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
    Path(id): Path<String>,
) -> Response {
    // Request write access to the database and call its delete_question method with specified id
    match store.write().await.delete_question(&id) {
        // If we get a good result, send a response informing the user
        Ok(_) => (StatusCode::OK, "Question deleted".to_string()).into_response(),
        // If we get an error, return a response with an error message
        Err(Error::QuestionNotFound) => {
            (StatusCode::NOT_FOUND, "Question not found".to_string()).into_response()
        }
        Err(_) => (StatusCode::BAD_REQUEST, "Bad request".to_string()).into_response(),
    }
}
