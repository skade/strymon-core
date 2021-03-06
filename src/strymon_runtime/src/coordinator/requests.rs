// Copyright 2017 ETH Zurich. All rights reserved.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use model::*;
use strymon_communication::rpc::Request;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Placement {
    Random(usize, usize), // (num executors, num workers)
    Fixed(Vec<ExecutorId>, usize), // (executors, num workers)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Submission {
    pub query: QueryProgram,
    pub name: Option<String>,
    pub placement: Placement,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum SubmissionError {
    ExecutorsNotFound,
    ExecutorUnreachable,
    SpawnError(::executor::requests::SpawnError),
}

impl Request for Submission {
    type Success = QueryId;
    type Error = SubmissionError;

    fn name() -> &'static str {
        "Submission"
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AddExecutor {
    pub host: String,
    pub ports: (u16, u16),
    pub format: ExecutionFormat,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ExecutorError;

impl Request for AddExecutor {
    type Success = ExecutorId;
    type Error = ExecutorError;

    fn name() -> &'static str {
        "AddExecutor"
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct QueryToken {
    pub id: QueryId,
    pub auth: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AddWorkerGroup {
    pub query: QueryId,
    pub group: usize,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum WorkerGroupError {
    InvalidWorkerGroup,
    SpawningAborted,
    PeerFailed,
}

impl Request for AddWorkerGroup {
    type Success = QueryToken;
    type Error = WorkerGroupError;

    fn name() -> &'static str {
        "AddWorkerGroup"
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Subscribe {
    pub name: String,
    pub blocking: bool,
    pub token: QueryToken,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum SubscribeError {
    TopicNotFound,
    AuthenticationFailure,
}

impl Request for Subscribe {
    type Success = Topic;
    type Error = SubscribeError;

    fn name() -> &'static str {
        "Subscribe"
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Unsubscribe {
    pub topic: TopicId,
    pub token: QueryToken,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum UnsubscribeError {
    InvalidTopicId,
    AuthenticationFailure,
}

impl Request for Unsubscribe {
    type Success = ();
    type Error = UnsubscribeError;

    fn name() -> &'static str {
        "Unsubscribe"
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Publish {
    pub name: String,
    pub addr: (String, u16),
    pub schema: TopicSchema,
    pub token: QueryToken,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum PublishError {
    TopicAlreadyExists,
    AuthenticationFailure,
}

impl Request for Publish {
    type Success = Topic;
    type Error = PublishError;

    fn name() -> &'static str {
        "Publish"
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Unpublish {
    pub topic: TopicId,
    pub token: QueryToken,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum UnpublishError {
    InvalidTopicId,
    AuthenticationFailure,
}

impl Request for Unpublish {
    type Success = ();
    type Error = UnpublishError;

    fn name() -> &'static str {
        "Unpublish"
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Lookup {
    pub name: String,
}

impl Request for Lookup {
    type Success = Topic;
    type Error = ();

    fn name() -> &'static str {
        "Lookup"
    }
}

/// Add a worker of a Keeper.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AddKeeperWorker {
    pub name: String,
    pub worker_num: usize,
    pub addr: (String, u16),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum AddKeeperWorkerError {
    WorkerAlreadyExists,
}

impl Request for AddKeeperWorker {
    type Success = ();
    type Error = AddKeeperWorkerError;

    fn name() -> &'static str {
        "AddKeeperWorker"
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GetKeeperAddress {
    pub name: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum GetKeeperAddressError {
    KeeperNotFound,
    KeeperHasNoWorkers,
}

impl Request for GetKeeperAddress {
    type Success = (String, u16);
    type Error = GetKeeperAddressError;

    fn name() -> &'static str {
        "GetKeeperAddress"
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RemoveKeeperWorker {
    pub name: String,
    pub worker_num: usize,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum RemoveKeeperWorkerError {
    KeeperDoesntExist,
    KeeperWorkerDoesntExist,
}

impl Request for RemoveKeeperWorker {
    type Success = ();
    type Error = RemoveKeeperWorkerError;

    fn name() -> &'static str {
        "RemoveKeeperWorker"
    }
}
