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

use http::Method;

use crate::s3::{
    Client,
    error::Error,
    response::MakeBucketResponse,
    types::{S3Api, S3Request, ToS3Request},
    utils::{Multimap, check_bucket_name, merge},
};

#[derive(Clone, Debug, Default)]
/// Argument for [make_bucket()](crate::s3::client::Client::make_bucket) API
pub struct MakeBucket {
    client: Option<Client>,

    pub extra_headers: Option<Multimap>,
    pub extra_query_params: Option<Multimap>,
    pub region: Option<String>,
    pub bucket: String,
    pub enable_object_lock: bool,
}

impl MakeBucket {
    pub fn new(bucket: &str) -> Self {
        Self {
            bucket: bucket.to_string(),
            ..Default::default()
        }
    }

    pub fn client(mut self, client: &Client) -> Self {
        self.client = Some(client.clone());
        self
    }

    pub fn extra_headers(mut self, extra_headers: Option<Multimap>) -> Self {
        self.extra_headers = extra_headers;
        self
    }

    pub fn extra_query_params(mut self, extra_query_params: Option<Multimap>) -> Self {
        self.extra_query_params = extra_query_params;
        self
    }

    pub fn region(mut self, region: Option<&str>) -> Self {
        self.region = region.map(|r| r.to_string());
        self
    }

    /// Enable object lock on the new bucket. It is not enabled by default.
    pub fn enable_object_lock(mut self, enable_object_lock: bool) -> Self {
        self.enable_object_lock = enable_object_lock;
        self
    }
}

// internal helpers
impl MakeBucket {
    fn get_headers(&self) -> Multimap {
        let mut headers = Multimap::new();
        if self.enable_object_lock {
            headers.insert(
                String::from("x-amz-bucket-object-lock-enabled"),
                String::from("true"),
            );
        }
        headers
    }
}

impl S3Api for MakeBucket {
    type S3Response = MakeBucketResponse;
}

impl ToS3Request for MakeBucket {
    fn to_s3request(&self) -> Result<S3Request, Error> {
        check_bucket_name(&self.bucket, true)?;

        let client: &Client = self.client.as_ref().ok_or(Error::NoClientProvided)?;

        let mut headers = Multimap::new();
        if let Some(v) = &self.extra_headers {
            merge(&mut headers, v);
        }
        merge(&mut headers, &self.get_headers());

        let mut query_params = Multimap::new();
        if let Some(v) = &self.extra_query_params {
            merge(&mut query_params, v);
        }

        let mut region = "us-east-1";
        if let Some(r) = &self.region {
            if !client.base_url.region.is_empty() {
                if client.base_url.region != *r {
                    return Err(Error::RegionMismatch(
                        client.base_url.region.clone(),
                        r.clone(),
                    ));
                }
            }
            region = r;
        }

        let body = match region {
            "us-east-1" => None,
            _ => Some(format!(
                "<CreateBucketConfiguration xmlns=\"http://s3.amazonaws.com/doc/2006-03-01/\">
                          <LocationConstraint>{}</LocationConstraint>
                         </CreateBucketConfiguration>",
                region
            )),
        };

        let req = S3Request::new(client, Method::PUT)
            .region(Some(region))
            .bucket(Some(&self.bucket))
            .headers(headers)
            .query_params(query_params)
            .body(body.map(|s| s.into()));

        client
            .region_map
            .insert(self.bucket.to_string(), region.to_string());

        Ok(req)
    }
}
