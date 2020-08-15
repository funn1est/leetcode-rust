use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Debug, Serialize, Deserialize)]
pub struct GqlQuery {
    #[serde(rename = "operationName")]
    pub operation_name: String,
    pub variables: serde_json::Value,
    pub query: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GqlRsp<T> {
    pub data: T,
}

pub const QUESTION_DATA_QUERY_OPERATION: &str = "questionData";
pub const QUESTION_DATA_QUERY: &str = r#"
query questionData($titleSlug: String!) {
    question(titleSlug: $titleSlug) {
        content
        stats
        codeDefinition
        sampleTestCase
        questionFrontendId
        metaData
    }
}"#;

#[derive(Debug, Serialize, Deserialize)]
pub struct QuestionData {
    pub question: QuestionInfo,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QuestionInfo {
    pub content: String,
    pub stats: String,
    #[serde(rename = "codeDefinition")]
    pub code_definition: String,
    #[serde(rename = "sampleTestCase")]
    pub sample_test_case: String,
    #[serde(rename = "questionFrontendId")]
    pub question_frontend_id: String,
    #[serde(rename = "metaData")]
    pub meta_data: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CodeDefinition {
    pub value: String,
    pub text: String,
    #[serde(rename = "defaultCode")]
    pub default_code: String,
}

pub type QuestionDataRsp = GqlRsp<QuestionData>;
