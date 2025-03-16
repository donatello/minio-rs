// MinIO Rust Library for Amazon S3 Compatible Cloud Storage
// Copyright 2025 MinIO, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use async_trait::async_trait;
use http::HeaderMap;

use crate::s3::{
    error::Error,
    types::{FromS3Response, S3Request},
};

/// Response of [make_bucket()](crate::s3::client::Client::make_bucket) API
#[derive(Debug, Clone)]
pub struct MakeBucketResponse {
    pub headers: HeaderMap,
    pub bucket: String,
    pub region: String,
}

#[async_trait]
impl FromS3Response for MakeBucketResponse {
    async fn from_s3response<'a>(
        req: S3Request<'a>,
        response: Result<reqwest::Response, Error>,
    ) -> Result<Self, Error> {
        let response = response?;
        let header_map = response.headers();

        Ok(MakeBucketResponse {
            headers: header_map.clone(),
            bucket: req.bucket.unwrap().to_string(),
            region: req.region.unwrap().to_string(),
        })
    }
}
