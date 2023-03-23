// Copyright (C) 2023 Quickwit, Inc.
//
// Quickwit is offered under the AGPL v3.0 and as commercial software.
// For commercial licensing, contact us at hello@quickwit.io.
//
// AGPL:
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <http://www.gnu.org/licenses/>.

// TODO: The shard API is declared in this crate to work around circular dependencies between the
// `quickwit-ingest` and `quickwit-metastore` crates. Find a way to declare the shard API in the
// `quickwit-ingest` crate and leave the implementation in this one.

#[path = "codegen/shard_service.rs"]
mod shard_service;

use prost_types::Timestamp;
use quickwit_common::timestamp::utc_now_ts;
pub use shard_service::*;

use crate::checkpoint;

#[derive(Debug, Clone, thiserror::Error, PartialEq, Eq)]
pub enum ShardServiceError {}

pub type Result<T> = std::result::Result<T, ShardServiceError>;

impl Shard {
    /// Instantiates a new open shard.
    pub fn new(
        index_id: String,
        source_id: String,
        shard_id: u64,
        leader_id: String,
        follower_id: String,
    ) -> Self {
        Self {
            index_id,
            source_id,
            shard_id,
            leader_id,
            follower_id,
            shard_status: ShardStatus::Open,
            consumer_position: Some(Position {
                offset: String::new(),
            }),
            closed_position: None,
            closed_timestamp: None,
            create_timestamp: Some(utc_now_ts()),
        }
    }

    pub fn consumer_position(&self) -> checkpoint::Position {
        self.consumer_position
            .map(|position| checkpoint::Position::from(position.offset))
            .expect("The field `consumer_position` is required and should always be set.")
    }

    pub fn create_timestamp(&self) -> Timestamp {
        self.create_timestamp
            .expect("The field `create_timestamp` is required and should always be set.")
    }
}
