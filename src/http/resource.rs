use sirix_rust_client::{
    synchronous::resource::Resource,
    types::{Json, ReadArgs, RevisionArg, SingleRevision, TwoRevisions, Xml},
};

use crate::parsers::read::RevisionType;

use super::types::{JsonResponse, XmlResponse};

pub fn read_json_resource(
    resource: Resource<Json>,
    node_id: Option<u128>,
    revision: Option<RevisionType>,
    max_level: Option<u64>,
    top_level_limit: Option<u64>,
    top_level_skip_last_node: Option<u64>,
) -> JsonResponse {
    let revision = match revision {
        Some(rev) => match rev {
            RevisionType::Revision { number, end_number } => match end_number {
                Some(end_number) => {
                    Some(RevisionArg::TwoRevisions(TwoRevisions::Number(number, end_number)))
                }
                None => Some(RevisionArg::SingleRevision(SingleRevision::Number(number))),
            },
            RevisionType::Timestamp {
                timestamp,
                end_timestamp,
            } => match end_timestamp {
                Some(end_timestamp) => {
                    Some(RevisionArg::TwoRevisions(TwoRevisions::Timestamp(timestamp, end_timestamp)))
                }
                None => Some(RevisionArg::SingleRevision(SingleRevision::Timestamp(timestamp))),
            },
        },
        None => None,
    };
    let response = resource.read(ReadArgs {
        node_id,
        revision,
        max_level,
        top_level_limit,
        top_level_skip_last_node,
    });
    match response {
        Ok(response) => JsonResponse::Ok(response.body),
        Err(err) => JsonResponse::Err(err),
    }
}

pub fn read_xml_resource(
    resource: Resource<Xml>,
    node_id: Option<u128>,
    revision: Option<RevisionType>,
    max_level: Option<u64>,
    top_level_limit: Option<u64>,
    top_level_skip_last_node: Option<u64>,
) -> XmlResponse {
    let revision = match revision {
        Some(rev) => match rev {
            RevisionType::Revision { number, end_number } => match end_number {
                Some(end_number) => {
                    Some(RevisionArg::TwoRevisions(TwoRevisions::Number(number, end_number)))
                }
                None => Some(RevisionArg::SingleRevision(SingleRevision::Number(number))),
            },
            RevisionType::Timestamp {
                timestamp,
                end_timestamp,
            } => match end_timestamp {
                Some(end_timestamp) => {
                    Some(RevisionArg::TwoRevisions(TwoRevisions::Timestamp(timestamp, end_timestamp)))
                }
                None => Some(RevisionArg::SingleRevision(SingleRevision::Timestamp(timestamp))),
            },
        },
        None => None,
    };
    let response = resource.read(ReadArgs {
        node_id,
        revision,
        max_level,
        top_level_limit,
        top_level_skip_last_node,
    });
    match response {
        Ok(response) => XmlResponse::Ok(response.body),
        Err(err) => XmlResponse::Err(err),
    }
}
