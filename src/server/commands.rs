use lspower::lsp;
use serde::{Deserialize, Serialize};

pub enum LspServerCommand {
    InjectTagFilter,
    InjectTagValueFilter,
    InjectFieldFilter,
    InjectMeasurementFilter,
    GetFunctionList,
}

impl TryFrom<String> for LspServerCommand {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "injectTagFilter" => {
                Ok(LspServerCommand::InjectTagFilter)
            }
            "injectTagValueFilter" => {
                Ok(LspServerCommand::InjectTagValueFilter)
            }
            "injectFieldFilter" => {
                Ok(LspServerCommand::InjectTagValueFilter)
            }
            "injectMeasurementFilter" => {
                Ok(LspServerCommand::InjectMeasurementFilter)
            }
            "getFunctionList" => {
                Ok(LspServerCommand::GetFunctionList)
            }
            _ => Err(format!(
                "Received unknown value for LspServerCommand: {}",
                value
            )),
        }
    }
}

impl From<LspServerCommand> for String {
    fn from(value: LspServerCommand) -> Self {
        match value {
            LspServerCommand::InjectTagFilter => {
                "injectTagFilter".into()
            }
            LspServerCommand::InjectTagValueFilter => {
                "injectTagValueFilter".into()
            }
            LspServerCommand::InjectFieldFilter => {
                "injectFieldFilter".into()
            }
            LspServerCommand::InjectMeasurementFilter => {
                "injectMeasurementFilter".into()
            }
            LspServerCommand::GetFunctionList => {
                "getFunctionList".into()
            }
        }
    }
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InjectTagFilterParams {
    pub text_document: lsp::TextDocumentIdentifier,
    pub bucket: String,
    pub name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub start: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub stop: Option<String>,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InjectTagValueFilterParams {
    pub text_document: lsp::TextDocumentIdentifier,
    pub bucket: String,
    pub name: String,
    pub value: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub start: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub stop: Option<String>,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InjectFieldFilterParams {
    pub text_document: lsp::TextDocumentIdentifier,
    pub bucket: String,
    pub name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub start: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub stop: Option<String>,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InjectMeasurementFilterParams {
    pub text_document: lsp::TextDocumentIdentifier,
    pub bucket: String,
    pub name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub start: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub stop: Option<String>,
}
