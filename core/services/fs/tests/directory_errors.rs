// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License.

#![cfg(unix)]

use opendal_core::{ErrorKind, Operator};
use opendal_service_fs::Fs;

#[tokio::test]
async fn write_to_directory_returns_is_a_directory() {
    let root = tempfile::tempdir().unwrap();
    let op = Operator::new(Fs::default().root(root.path().to_str().unwrap())).unwrap();
    op.write("dir/file", "x").await.unwrap();

    let err = op.write("dir", "y").await.unwrap_err();
    assert_eq!(err.kind(), ErrorKind::IsADirectory);
    assert!(!err.is_temporary());
}

#[tokio::test]
async fn read_directory_without_slash_returns_is_a_directory() {
    let root = tempfile::tempdir().unwrap();
    let op = Operator::new(Fs::default().root(root.path().to_str().unwrap())).unwrap();
    op.write("dir/file", "x").await.unwrap();

    let err = op.read("dir").await.unwrap_err();
    assert_eq!(err.kind(), ErrorKind::IsADirectory);
    assert!(!err.is_temporary());
}
