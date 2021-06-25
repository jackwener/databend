// Copyright 2020-2021 The Datafuse Authors.
//
// SPDX-License-Identifier: Apache-2.0.

#[test]
fn test_config() -> common_exception::Result<()> {
    use pretty_assertions::assert_eq;

    use crate::configs::Config;

    // Default.
    {
        let expect = Config {
            log_level: "debug".to_string(),
            log_dir: "./_logs".to_string(),
            num_cpus: 8,
            mysql_handler_host: "127.0.0.1".to_string(),
            mysql_handler_port: 3307,
            mysql_handler_thread_num: 256,
            clickhouse_handler_host: "127.0.0.1".to_string(),
            clickhouse_handler_port: 9000,
            clickhouse_handler_thread_num: 256,
            flight_api_address: "127.0.0.1:9090".to_string(),
            http_api_address: "127.0.0.1:8080".to_string(),
            metric_api_address: "127.0.0.1:7070".to_string(),
            store_api_address: "127.0.0.1:9191".to_string(),
            store_api_username: "root".to_string(),
            store_api_password: "root".to_string(),
            config_file: "".to_string(),
        };
        let actual = Config::default();
        assert_eq!(actual, expect);
    }

    // From Args.
    {
        let actual = Config::load_from_args();
        assert_eq!("INFO", actual.log_level);
    }

    // From file NotFound.
    {
        if let Err(e) = Config::load_from_toml("xx.toml") {
            let expect = "Code: 23, displayText = File: xx.toml, err: Os { code: 2, kind: NotFound, message: \"No such file or directory\" }.";
            assert_eq!(expect, format!("{}", e));
        }
    }

    // From file.
    {
        std::env::set_var("FUSE_QUERY_LOG_LEVEL", "DEBUG");
        let path = std::env::current_dir()
            .unwrap()
            .join("conf/fusequery_config_spec.toml")
            .display()
            .to_string();

        let actual = Config::load_from_toml(path.as_str())?;
        assert_eq!("INFO", actual.log_level);
        let env = Config::load_from_env(&actual)?;
        assert_eq!("DEBUG", env.log_level);
        std::env::remove_var("FUSE_QUERY_LOG_LEVEL");
    }

    // From env, defaulting.
    {
        std::env::set_var("FUSE_QUERY_LOG_LEVEL", "DEBUG");
        std::env::set_var("FUSE_QUERY_MYSQL_HANDLER_HOST", "0.0.0.0");
        std::env::set_var("FUSE_QUERY_MYSQL_HANDLER_PORT", "3306");
        std::env::remove_var("FUSE_QUERY_MYSQL_HANDLER_THREAD_NUM");
        let default = Config::default();
        let configured = Config::load_from_env(&default)?;
        assert_eq!("DEBUG", configured.log_level);
        assert_eq!("0.0.0.0", configured.mysql_handler_host);
        assert_eq!(3306, configured.mysql_handler_port);

        // not set, so keep it as original value
        assert_eq!(256, configured.mysql_handler_thread_num);

        // clean up
        std::env::remove_var("FUSE_QUERY_LOG_LEVEL");
        std::env::remove_var("FUSE_QUERY_MYSQL_HANDLER_HOST");
        std::env::remove_var("FUSE_QUERY_MYSQL_HANDLER_PORT");
    }

    // From env, load config file and ignore the rest settings.
    {
        std::env::set_var("FUSE_QUERY_LOG_LEVEL", "DEBUG");
        let config_path = std::env::current_dir()
            .unwrap()
            .join("conf/fusequery_config_spec.toml")
            .display()
            .to_string();
        std::env::set_var("CONFIG_FILE", config_path);
        let config = Config::load_from_env(&Config::default())?;
        assert_eq!(config.log_level, "INFO");
        std::env::remove_var("FUSE_QUERY_LOG_LEVEL");
        std::env::remove_var("CONFIG_FILE");
    }
    Ok(())
}

#[test]
fn test_fuse_commit_version() -> anyhow::Result<()> {
    let v = &crate::configs::config::FUSE_COMMIT_VERSION;
    assert!(v.len() > 0);
    Ok(())
}
