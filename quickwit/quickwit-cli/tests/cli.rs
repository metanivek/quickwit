// Copyright (C) 2022 Quickwit, Inc.
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

#![allow(clippy::bool_assert_comparison)]

mod helpers;

use std::collections::HashSet;
use std::path::Path;
use std::str::FromStr;

use anyhow::Result;
use clap::ErrorKind;
use helpers::{TestEnv, TestStorageType};
use quickwit_cli::cli::build_cli;
use quickwit_cli::index::{
    create_index_cli, delete_index_cli, garbage_collect_index_cli, ingest_docs_cli, search_index,
    CreateIndexArgs, DeleteIndexArgs, GarbageCollectIndexArgs, IngestDocsArgs, SearchIndexArgs,
};
use quickwit_cli::service::RunCliCommand;
use quickwit_common::fs::get_cache_directory_path;
use quickwit_common::rand::append_random_suffix;
use quickwit_common::service::QuickwitService;
use quickwit_common::uri::Uri;
use quickwit_common::ChecklistError;
use quickwit_config::CLI_INGEST_SOURCE_ID;
use quickwit_indexing::actors::INDEXING_DIR_NAME;
use quickwit_metastore::{quickwit_metastore_uri_resolver, Metastore, MetastoreError, SplitState};
use serde_json::{json, Number, Value};
use tokio::time::{sleep, Duration};

use crate::helpers::{create_test_env, wait_port_ready, PACKAGE_BIN_NAME};

async fn create_logs_index(test_env: &TestEnv) -> anyhow::Result<()> {
    let args = CreateIndexArgs {
        config_uri: test_env.config_uri.clone(),
        index_config_uri: test_env.index_config_uri.clone(),
        overwrite: false,
        assume_yes: true,
    };
    create_index_cli(args).await
}

async fn ingest_docs(input_path: &Path, test_env: &TestEnv) -> anyhow::Result<()> {
    let args = IngestDocsArgs {
        config_uri: test_env.config_uri.clone(),
        index_id: test_env.index_id.clone(),
        input_path_opt: Some(input_path.to_path_buf()),
        overwrite: false,
        clear_cache: true,
    };

    ingest_docs_cli(args).await
}

#[test]
fn test_cmd_help() {
    let cmd = build_cli();
    let error = cmd
        .clone()
        .try_get_matches_from(vec![PACKAGE_BIN_NAME, "--help"])
        .unwrap_err();
    // on `--help` clap returns an error.
    assert_eq!(error.kind(), ErrorKind::DisplayHelp);
}

#[tokio::test]
async fn test_cmd_create() {
    let index_id = append_random_suffix("test-create-cmd");
    let test_env = create_test_env(index_id, TestStorageType::LocalFileSystem).unwrap();
    create_logs_index(&test_env).await.unwrap();

    let index_metadata = test_env.index_metadata().await.unwrap();
    assert_eq!(index_metadata.index_id(), test_env.index_id);

    // Create without giving `index-uri`.
    let index_id = append_random_suffix("test-create-cmd-no-index-uri");
    let test_env = create_test_env(index_id, TestStorageType::LocalFileSystem).unwrap();

    let index_config_without_uri = Uri::from_str(&test_env.index_config_without_uri()).unwrap();
    let args = CreateIndexArgs {
        config_uri: test_env.config_uri.clone(),
        index_config_uri: index_config_without_uri,
        overwrite: false,
        assume_yes: true,
    };

    create_index_cli(args).await.unwrap();

    let index_metadata = test_env.index_metadata().await.unwrap();
    assert_eq!(index_metadata.index_id(), test_env.index_id);
    assert_eq!(index_metadata.index_uri(), &test_env.index_uri);

    // Create non existing index with --overwrite.
    let index_id = append_random_suffix("test-create-non-existing-index-with-overwrite");
    let test_env = create_test_env(index_id, TestStorageType::LocalFileSystem).unwrap();

    let index_config_without_uri = Uri::from_str(&test_env.index_config_without_uri()).unwrap();
    let args = CreateIndexArgs {
        config_uri: test_env.config_uri.clone(),
        index_config_uri: index_config_without_uri,
        overwrite: true,
        assume_yes: true,
    };

    create_index_cli(args).await.unwrap();

    let index_metadata = test_env.index_metadata().await.unwrap();
    assert_eq!(index_metadata.index_id(), &test_env.index_id);
    assert_eq!(index_metadata.index_uri(), &test_env.index_uri);

    // Attempt to create with ill-formed new command.
    let app = build_cli();
    let result = app.try_get_matches_from(vec![PACKAGE_BIN_NAME, "index", "create"]);
    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err().kind(),
        ErrorKind::MissingRequiredArgument
    );
}

#[tokio::test]
async fn test_cmd_create_on_existing_index() {
    let index_id = append_random_suffix("test-create-cmd--index-already-exists");
    let test_env = create_test_env(index_id.clone(), TestStorageType::LocalFileSystem).unwrap();
    create_logs_index(&test_env).await.unwrap();

    let args = CreateIndexArgs {
        config_uri: test_env.config_uri,
        index_config_uri: test_env.index_config_uri,
        overwrite: false,
        assume_yes: false,
    };

    let error = create_index_cli(args).await.unwrap_err();
    assert_eq!(
        error.root_cause().downcast_ref::<MetastoreError>().unwrap(),
        &MetastoreError::IndexAlreadyExists { index_id }
    );
}

#[tokio::test]
async fn test_cmd_ingest_on_non_existing_index() {
    let index_id = append_random_suffix("index-does-not-exist");
    let test_env = create_test_env(index_id, TestStorageType::LocalFileSystem).unwrap();

    let args = IngestDocsArgs {
        config_uri: test_env.config_uri,
        index_id: "index-does-not-exist".to_string(),
        input_path_opt: Some(test_env.resource_files["logs"].clone()),
        overwrite: false,
        clear_cache: true,
    };

    let error = ingest_docs_cli(args).await.unwrap_err();

    assert_eq!(
        error.root_cause().downcast_ref::<MetastoreError>().unwrap(),
        &MetastoreError::IndexDoesNotExist {
            index_id: "index-does-not-exist".to_string()
        }
    );
}

#[tokio::test]
async fn test_cmd_ingest_on_non_existing_file() {
    let index_id = append_random_suffix("test-new-cmd--file-does-not-exist");
    let test_env = create_test_env(index_id, TestStorageType::LocalFileSystem).unwrap();
    create_logs_index(&test_env).await.unwrap();

    let args = IngestDocsArgs {
        config_uri: test_env.config_uri,
        index_id: test_env.index_id,
        input_path_opt: Some(test_env.data_dir_path.join("file-does-not-exist.json")),
        overwrite: false,
        clear_cache: true,
    };

    let error = ingest_docs_cli(args).await.unwrap_err();

    assert!(matches!(
        error.root_cause().downcast_ref::<ChecklistError>().unwrap(),
        ChecklistError {
            errors
        } if errors.len() == 1 && errors[0].0 == CLI_INGEST_SOURCE_ID
    ));
}

#[tokio::test]
async fn test_ingest_docs_cli_keep_cache() {
    let index_id = append_random_suffix("test-index-keep-cache");
    let test_env = create_test_env(index_id.clone(), TestStorageType::LocalFileSystem).unwrap();
    create_logs_index(&test_env).await.unwrap();

    let args = IngestDocsArgs {
        config_uri: test_env.config_uri,
        index_id,
        input_path_opt: Some(test_env.resource_files["logs"].clone()),
        overwrite: false,
        clear_cache: false,
    };

    ingest_docs_cli(args).await.unwrap();
    // Ensure cache directory is not empty.
    let cache_directory_path = get_cache_directory_path(&test_env.data_dir_path);
    assert!(cache_directory_path.read_dir().unwrap().next().is_some());
}

#[tokio::test]
async fn test_ingest_docs_cli() {
    let index_id = append_random_suffix("test-index-simple");
    let test_env = create_test_env(index_id.clone(), TestStorageType::LocalFileSystem).unwrap();
    create_logs_index(&test_env).await.unwrap();

    let args = IngestDocsArgs {
        config_uri: test_env.config_uri.clone(),
        index_id: index_id.clone(),
        input_path_opt: Some(test_env.resource_files["logs"].clone()),
        overwrite: false,
        clear_cache: true,
    };

    ingest_docs_cli(args).await.unwrap();

    let splits: Vec<_> = test_env
        .metastore()
        .await
        .unwrap()
        .list_all_splits(&index_id)
        .await
        .unwrap();

    assert_eq!(splits.len(), 1);
    assert_eq!(splits[0].split_metadata.num_docs, 5);

    // Ensure cache directory is empty.
    let cache_directory_path = get_cache_directory_path(&test_env.data_dir_path);

    assert!(cache_directory_path.read_dir().unwrap().next().is_none());
}

#[tokio::test]
async fn test_cmd_search_aggregation() -> Result<()> {
    let index_id = append_random_suffix("test-search-cmd");
    let test_env = create_test_env(index_id, TestStorageType::LocalFileSystem)?;
    create_logs_index(&test_env).await.unwrap();

    ingest_docs(test_env.resource_files["logs"].as_path(), &test_env)
        .await
        .unwrap();

    let aggregation: Value = json!(
    {
      "range_buckets": {
        "range": {
          "field": "ts",
          "ranges": [
            { "to": 72057597000000f64 },
            { "from": 72057597000000f64, "to": 72057600000000f64 },
            { "from": 72057600000000f64, "to": 72057604000000f64 },
            { "from": 72057604000000f64 },
          ]
        },
        "aggs": {
          "average_ts": {
            "avg": { "field": "ts" }
          }
        }
      }
    });

    // search with aggregation
    let args = SearchIndexArgs {
        index_id: test_env.index_id,
        query: "paris OR tokio OR london".to_string(),
        aggregation: Some(serde_json::to_string(&aggregation).unwrap()),
        max_hits: 10,
        start_offset: 0,
        search_fields: Some(vec!["city".to_string()]),
        snippet_fields: None,
        start_timestamp: None,
        end_timestamp: None,
        config_uri: Uri::from_str(&test_env.resource_files["config"].display().to_string())
            .unwrap(),
        sort_by_score: false,
    };
    let search_response = search_index(args).await?;

    let aggregation_res: Value =
        serde_json::from_str(&search_response.aggregation.unwrap()).unwrap();

    assert_eq!(
        aggregation_res,
        json!({
          "range_buckets": {
            "buckets": [
              {
                "key": "*-1972-04-13T23:59:57Z",
                "doc_count": 0,
                "average_ts": {
                    "value": null,
                },
                "to": 72057597000000f64,
                "to_as_string": "1972-04-13T23:59:57Z"
              },
              {
                "key": "1972-04-13T23:59:57Z-1972-04-14T00:00:00Z",
                "doc_count": 2,
                "average_ts": {
                  "value": 72057597500000f64,
                },
                "from": 72057597000000f64,
                "to": 72057600000000f64,
                "from_as_string": "1972-04-13T23:59:57Z",
                "to_as_string": "1972-04-14T00:00:00Z"
              },
              {
                "key": "1972-04-14T00:00:00Z-1972-04-14T00:00:04Z",
                "doc_count": 0,
                "average_ts": {
                  "value": null,
                },
                "from": 72057600000000f64,
                "to": 72057604000000f64,
                "from_as_string": "1972-04-14T00:00:00Z",
                "to_as_string": "1972-04-14T00:00:04Z"
              },
              {
                "key": "1972-04-14T00:00:04Z-*",
                "doc_count": 3,
                "average_ts": {
                  "value": 72057606333333.33f64,
                },
                "from": 72057604000000f64,
                "from_as_string": "1972-04-14T00:00:04Z"
              }
            ]
          }
        })
    );

    Ok(())
}

#[tokio::test]
async fn test_cmd_search_with_snippets() -> Result<()> {
    let index_id = append_random_suffix("test-search-cmd");
    let test_env = create_test_env(index_id, TestStorageType::LocalFileSystem)?;
    create_logs_index(&test_env).await.unwrap();

    ingest_docs(test_env.resource_files["logs"].as_path(), &test_env)
        .await
        .unwrap();

    // search with snippets
    let args = SearchIndexArgs {
        index_id: test_env.index_id,
        query: "event:baz".to_string(),
        aggregation: None,
        max_hits: 10,
        start_offset: 0,
        search_fields: None,
        snippet_fields: Some(vec!["event".to_string()]),
        start_timestamp: None,
        end_timestamp: None,
        config_uri: Uri::from_str(&test_env.resource_files["config"].display().to_string())
            .unwrap(),
        sort_by_score: false,
    };
    let search_response = search_index(args).await?;
    assert_eq!(search_response.hits.len(), 1);
    let hit = &search_response.hits[0];
    assert_eq!(
        serde_json::from_str::<Value>(&hit.json).unwrap(),
        json!({"event": "baz", "ts": 72057604})
    );
    assert_eq!(
        serde_json::from_str::<Value>(hit.snippet.as_ref().unwrap()).unwrap(),
        json!({
            "event": [ "<b>baz</b>"]
        })
    );
    Ok(())
}

#[tokio::test]
async fn test_search_index_cli() {
    let index_id = append_random_suffix("test-search-cmd");
    let test_env = create_test_env(index_id.clone(), TestStorageType::LocalFileSystem).unwrap();
    create_logs_index(&test_env).await.unwrap();

    let create_search_args = |query: &str| SearchIndexArgs {
        config_uri: test_env.config_uri.clone(),
        index_id: index_id.clone(),
        query: query.to_string(),
        aggregation: None,
        max_hits: 20,
        start_offset: 0,
        search_fields: None,
        snippet_fields: None,
        start_timestamp: None,
        end_timestamp: None,
        sort_by_score: false,
    };

    ingest_docs(test_env.resource_files["logs"].as_path(), &test_env)
        .await
        .unwrap();

    let args = create_search_args("level:info");

    // search_index_cli calls search_index and prints the SearchResponse
    let search_res = search_index(args).await.unwrap();
    assert_eq!(search_res.num_hits, 2);

    // search with tag pruning
    let args = create_search_args("+level:info +city:paris");

    // search_index_cli calls search_index and prints the SearchResponse
    let search_res = search_index(args).await.unwrap();
    assert_eq!(search_res.num_hits, 1);

    // search with tag pruning
    let args = create_search_args("level:info AND city:conakry");

    // search_index_cli calls search_index and prints the SearchResponse
    let search_res = search_index(args).await.unwrap();
    assert_eq!(search_res.num_hits, 0);
}

#[tokio::test]
async fn test_delete_index_cli_dry_run() {
    let index_id = append_random_suffix("test-delete-cmd--dry-run");
    let test_env = create_test_env(index_id.clone(), TestStorageType::LocalFileSystem).unwrap();
    create_logs_index(&test_env).await.unwrap();

    let refresh_metastore = |metastore| {
        // In this test we rely on the file backed metastore
        // and the file backed metastore caches results.
        // Therefore we need to force reading the disk to fetch updates.
        //
        // We do that by dropping and recreating our metastore.
        drop(metastore);
        quickwit_metastore_uri_resolver().resolve(&test_env.metastore_uri)
    };

    let create_delete_args = |dry_run| DeleteIndexArgs {
        config_uri: test_env.config_uri.clone(),
        index_id: index_id.clone(),
        dry_run,
    };

    let metastore = quickwit_metastore_uri_resolver()
        .resolve(&test_env.metastore_uri)
        .await
        .unwrap();

    assert!(metastore.index_exists(&index_id).await.unwrap());
    // On empty index.
    let args = create_delete_args(true);

    delete_index_cli(args).await.unwrap();
    // On dry run index should still exist
    let metastore = refresh_metastore(metastore).await.unwrap();
    assert!(metastore.index_exists(&index_id).await.unwrap());

    ingest_docs(test_env.resource_files["logs"].as_path(), &test_env)
        .await
        .unwrap();

    // On non-empty index
    let args = create_delete_args(true);

    delete_index_cli(args).await.unwrap();
    // On dry run index should still exist
    let metastore = refresh_metastore(metastore).await.unwrap();
    assert!(metastore.index_exists(&index_id).await.unwrap());

    let args = create_delete_args(false);

    delete_index_cli(args).await.unwrap();
    let metastore = refresh_metastore(metastore).await.unwrap();
    assert!(!metastore.index_exists(&index_id).await.unwrap());
}

#[tokio::test]
async fn test_delete_index_cli() {
    let index_id = append_random_suffix("test-delete-cmd");
    let test_env = create_test_env(index_id.clone(), TestStorageType::LocalFileSystem).unwrap();
    create_logs_index(&test_env).await.unwrap();

    ingest_docs(test_env.resource_files["logs"].as_path(), &test_env)
        .await
        .unwrap();

    let args = DeleteIndexArgs {
        config_uri: test_env.config_uri.clone(),
        index_id: index_id.clone(),
        dry_run: false,
    };

    delete_index_cli(args).await.unwrap();

    assert!(test_env
        .metastore()
        .await
        .unwrap()
        .index_metadata(&test_env.index_id)
        .await
        .is_err());

    assert!(!test_env
        .data_dir_path
        .join(INDEXING_DIR_NAME)
        .join(test_env.index_id)
        .as_path()
        .try_exists()
        .unwrap());
}

#[tokio::test]
async fn test_garbage_collect_cli_no_grace() {
    let index_id = append_random_suffix("test-gc-cmd--no-grace-period");
    let test_env = create_test_env(index_id.clone(), TestStorageType::LocalFileSystem).unwrap();
    create_logs_index(&test_env).await.unwrap();
    ingest_docs(test_env.resource_files["logs"].as_path(), &test_env)
        .await
        .unwrap();

    let metastore = quickwit_metastore_uri_resolver()
        .resolve(&test_env.metastore_uri)
        .await
        .unwrap();

    let refresh_metastore = |metastore| {
        // In this test we rely on the file backed metastore and write on
        // a different process. The file backed metastore caches results.
        // Therefore we need to force reading the disk.
        //
        // We do that by dropping and recreating our metastore.
        drop(metastore);
        quickwit_metastore_uri_resolver().resolve(&test_env.metastore_uri)
    };

    let create_gc_args = |dry_run| GarbageCollectIndexArgs {
        config_uri: test_env.config_uri.clone(),
        index_id: index_id.clone(),
        grace_period: Duration::from_secs(3600),
        dry_run,
    };

    let splits = metastore.list_all_splits(&test_env.index_id).await.unwrap();
    assert_eq!(splits.len(), 1);

    let args = create_gc_args(false);

    garbage_collect_index_cli(args).await.unwrap();

    // On gc splits within grace period should still exist.
    let index_path = test_env.indexes_dir_path.join(&test_env.index_id);
    assert_eq!(index_path.try_exists().unwrap(), true);

    let split_ids = [splits[0].split_id()];
    let metastore = refresh_metastore(metastore).await.unwrap();
    metastore
        .mark_splits_for_deletion(&test_env.index_id, &split_ids)
        .await
        .unwrap();

    let args = create_gc_args(true);

    garbage_collect_index_cli(args).await.unwrap();

    // On `dry_run = true` splits `MarkedForDeletion` should still exist.
    for split_id in split_ids {
        let split_file = quickwit_common::split_file(split_id);
        let split_filepath = index_path.join(&split_file);
        assert_eq!(split_filepath.try_exists().unwrap(), true);
    }

    let args = create_gc_args(false);

    garbage_collect_index_cli(args).await.unwrap();

    // If split is `MarkedForDeletion` it should be deleted after gc run
    for split_id in split_ids {
        let split_file = quickwit_common::split_file(split_id);
        let split_filepath = index_path.join(&split_file);
        assert_eq!(split_filepath.try_exists().unwrap(), false);
    }

    let metastore = refresh_metastore(metastore).await.unwrap();
    assert_eq!(
        metastore
            .list_all_splits(&test_env.index_id)
            .await
            .unwrap()
            .len(),
        0
    );

    let args = DeleteIndexArgs {
        config_uri: test_env.config_uri.clone(),
        index_id,
        dry_run: false,
    };

    delete_index_cli(args).await.unwrap();

    assert_eq!(index_path.try_exists().unwrap(), false);
}

#[tokio::test]
async fn test_garbage_collect_index_cli() {
    let index_id = append_random_suffix("test-gc-cmd");
    let test_env = create_test_env(index_id.clone(), TestStorageType::LocalFileSystem).unwrap();
    create_logs_index(&test_env).await.unwrap();
    ingest_docs(test_env.resource_files["logs"].as_path(), &test_env)
        .await
        .unwrap();

    let refresh_metastore = |metastore| {
        // In this test we rely on the file backed metastore and
        // modify it but the file backed metastore caches results.
        // Therefore we need to force reading the disk to update split info.
        //
        // We do that by dropping and recreating our metastore.
        drop(metastore);
        quickwit_metastore_uri_resolver().resolve(&test_env.metastore_uri)
    };

    let create_gc_args = |grace_period_secs| GarbageCollectIndexArgs {
        config_uri: test_env.config_uri.clone(),
        index_id: index_id.clone(),
        grace_period: Duration::from_secs(grace_period_secs),
        dry_run: false,
    };

    let metastore = quickwit_metastore_uri_resolver()
        .resolve(&test_env.metastore_uri)
        .await
        .unwrap();

    let splits = metastore.list_all_splits(&test_env.index_id).await.unwrap();
    assert_eq!(splits.len(), 1);

    let index_path = test_env.indexes_dir_path.join(&test_env.index_id);
    let split_filename = quickwit_common::split_file(splits[0].split_metadata.split_id.as_str());
    let split_path = index_path.join(&split_filename);
    assert_eq!(split_path.try_exists().unwrap(), true);

    let args = create_gc_args(3600);

    garbage_collect_index_cli(args).await.unwrap();

    // Split should still exists within grace period.
    let metastore = refresh_metastore(metastore).await.unwrap();
    let splits = metastore.list_all_splits(&test_env.index_id).await.unwrap();
    assert_eq!(splits.len(), 1);

    // The following steps help turn an existing published split into a staged one
    // without deleting the files.
    let split = splits[0].clone();
    let split_ids = [split.split_metadata.split_id.as_str()];
    metastore
        .mark_splits_for_deletion(&test_env.index_id, &split_ids)
        .await
        .unwrap();
    metastore
        .delete_splits(&test_env.index_id, &split_ids)
        .await
        .unwrap();
    metastore
        .stage_splits(&test_env.index_id, vec![split.split_metadata])
        .await
        .unwrap();
    assert_eq!(split_path.try_exists().unwrap(), true);

    let metastore = refresh_metastore(metastore).await.unwrap();
    let splits = metastore.list_all_splits(&test_env.index_id).await.unwrap();
    assert_eq!(splits[0].split_state, SplitState::Staged);

    let args = create_gc_args(3600);

    garbage_collect_index_cli(args).await.unwrap();

    assert_eq!(split_path.try_exists().unwrap(), true);
    // Staged splits should still exist within grace period.
    let metastore = refresh_metastore(metastore).await.unwrap();
    let splits = metastore.list_all_splits(&test_env.index_id).await.unwrap();
    assert_eq!(splits.len(), 1);
    assert_eq!(splits[0].split_state, SplitState::Staged);

    // Wait for grace period.
    // TODO: edit split update timestamps and remove this sleep.
    sleep(Duration::from_secs(2)).await;

    let args = create_gc_args(1);

    garbage_collect_index_cli(args).await.unwrap();

    let metastore = refresh_metastore(metastore).await.unwrap();
    let splits = metastore.list_all_splits(&test_env.index_id).await.unwrap();
    // Splits should be deleted from both metastore and file system.
    assert_eq!(splits.len(), 0);
    assert_eq!(split_path.try_exists().unwrap(), false);
}

/// testing the api via cli commands
#[tokio::test]
async fn test_all_local_index() {
    quickwit_common::setup_logging_for_tests();
    let index_id = append_random_suffix("test-all");
    let test_env = create_test_env(index_id.clone(), TestStorageType::LocalFileSystem).unwrap();
    create_logs_index(&test_env).await.unwrap();

    let metadata_file_exists = test_env
        .storage
        .exists(&Path::new(&test_env.index_id).join("metastore.json"))
        .await
        .unwrap();
    assert!(metadata_file_exists);

    ingest_docs(test_env.resource_files["logs"].as_path(), &test_env)
        .await
        .unwrap();

    // serve & api-search
    let run_cli_command = RunCliCommand {
        config_uri: test_env.config_uri.clone(),
        services: Some(HashSet::from([
            QuickwitService::Searcher,
            QuickwitService::Metastore,
        ])),
    };

    let service_task = tokio::spawn(async move { run_cli_command.execute().await.unwrap() });

    wait_port_ready(test_env.rest_listen_port).await.unwrap();

    let query_response = reqwest::get(format!(
        "http://127.0.0.1:{}/api/v1/{}/search?query=level:info",
        test_env.rest_listen_port, test_env.index_id
    ))
    .await
    .unwrap()
    .text()
    .await
    .unwrap();

    let result: Value =
        serde_json::from_str(&query_response).expect("Couldn't deserialize response.");
    assert_eq!(result["num_hits"], Value::Number(Number::from(2i64)));

    let search_stream_response = reqwest::get(format!(
        "http://127.0.0.1:{}/api/v1/{}/search/stream?query=level:info&output_format=csv&fast_field=ts",
        test_env.rest_listen_port,
        test_env.index_id
    ))
    .await
    .unwrap()
    .text()
    .await
    .unwrap();
    assert_eq!(search_stream_response, "72057597000000\n72057608000000\n");

    service_task.abort();

    let args = DeleteIndexArgs {
        config_uri: test_env.config_uri.clone(),
        index_id,
        dry_run: false,
    };

    delete_index_cli(args).await.unwrap();

    let metadata_file_exists = test_env
        .storage
        .exists(&Path::new(&test_env.index_id).join("quickwit.json"))
        .await
        .unwrap();
    assert_eq!(metadata_file_exists, false);
}

/// testing the api via cli commands
#[tokio::test]
#[cfg_attr(not(feature = "ci-test"), ignore)]
async fn test_all_with_s3_localstack_cli() {
    let index_id = append_random_suffix("test-all--cli-s3-localstack");
    let test_env = create_test_env(index_id.clone(), TestStorageType::S3).unwrap();
    create_logs_index(&test_env).await.unwrap();

    ingest_docs(test_env.resource_files["logs"].as_path(), &test_env)
        .await
        .unwrap();

    // Cli search
    let args = SearchIndexArgs {
        config_uri: test_env.config_uri.clone(),
        index_id: index_id.clone(),
        query: "level:info".to_string(),
        aggregation: None,
        max_hits: 20,
        start_offset: 0,
        search_fields: None,
        snippet_fields: None,
        start_timestamp: None,
        end_timestamp: None,
        sort_by_score: false,
    };

    let search_res = search_index(args).await.unwrap();
    assert_eq!(search_res.num_hits, 2);

    // serve & api-search
    // TODO: ditto.
    let run_cli_command = RunCliCommand {
        config_uri: test_env.config_uri.clone(),
        services: Some(HashSet::from([
            QuickwitService::Searcher,
            QuickwitService::Metastore,
        ])),
    };
    let service_task = tokio::spawn(async move { run_cli_command.execute().await.unwrap() });

    wait_port_ready(test_env.rest_listen_port).await.unwrap();

    let query_response = reqwest::get(format!(
        "http://127.0.0.1:{}/api/v1/{}/search?query=level:info",
        test_env.rest_listen_port, test_env.index_id,
    ))
    .await
    .unwrap()
    .text()
    .await
    .unwrap();

    let result: Value =
        serde_json::from_str(&query_response).expect("Couldn't deserialize response.");
    assert_eq!(result["num_hits"], Value::Number(Number::from(2i64)));

    service_task.abort();

    let args = DeleteIndexArgs {
        config_uri: test_env.config_uri.clone(),
        index_id: index_id.clone(),
        dry_run: false,
    };

    delete_index_cli(args).await.unwrap();

    assert_eq!(
        test_env
            .storage
            .exists(Path::new(&test_env.index_id))
            .await
            .unwrap(),
        false
    );
}
