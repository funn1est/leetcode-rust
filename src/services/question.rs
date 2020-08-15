use serde_json;

use crate::services::{gql_fetch, ServiceResult};
use crate::types::query::{
    GqlQuery, QuestionDataRsp, QUESTION_DATA_QUERY, QUESTION_DATA_QUERY_OPERATION,
};

pub async fn get_question_data(question_name: &str) -> ServiceResult<QuestionDataRsp> {
    gql_fetch(&GqlQuery {
        operation_name: QUESTION_DATA_QUERY_OPERATION.to_owned(),
        variables: serde_json::json!({ "titleSlug": question_name }),
        query: QUESTION_DATA_QUERY.to_owned(),
    })
    .await
}
