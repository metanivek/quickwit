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

use prost_types::Timestamp;
use time::OffsetDateTime;

pub fn utc_now_ts() -> Timestamp {
    let now = OffsetDateTime::now_utc();

    Timestamp {
        seconds: now.unix_timestamp() as i64,
        nanos: now.nanosecond() as i32,
    }
}

pub trait TimestampExt {
    fn as_datetime(&self) -> OffsetDateTime;
    fn seconds(&self) -> u64;
    fn millis(&self) -> u64;
    fn nanos(&self) -> u64;
}

impl TimestampExt for Timestamp {
    fn as_datetime(&self) -> OffsetDateTime {
        let mut timestamp_nanos = self.nanos as i128;
        timestamp_nanos += self.seconds as i128 * 1_000_000_000;

        OffsetDateTime::from_unix_timestamp_nanos(timestamp_nanos)
            .expect("The second and nano components should be in range.")
    }

    fn seconds(&self) -> u64 {
        self.seconds as u64
    }

    fn millis(&self) -> u64 {
        self.seconds() * 1_000 + self.nanos() / 1_000_000
    }

    fn nanos(&self) -> u64 {
        self.nanos as u64
    }
}
