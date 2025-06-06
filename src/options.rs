use {super::*, bitcoincore_rpc::Auth};

#[derive(Clone, Default, Debug, Parser)]
#[command(group(
  ArgGroup::new("chains")
    .required(false)
    .args(&["chain_argument", "signet", "regtest", "testnet"]),
))]
pub(crate) struct Options {
  #[arg(long, help = "Load Junkcoin Core data dir from <JUNKCOIN_DATA_DIR>.")]
  pub(crate) junkcoin_data_dir: Option<PathBuf>,
  #[arg(
    long = "chain",
    value_enum,
    default_value = "mainnet",
    help = "Use <CHAIN>."
  )]
  pub(crate) chain_argument: Chain,
  #[arg(long, help = "Load configuration from <CONFIG>.")]
  pub(crate) config: Option<PathBuf>,
  #[arg(long, help = "Load configuration from <CONFIG_DIR>.")]
  pub(crate) config_dir: Option<PathBuf>,
  #[arg(long, help = "Load Junkcoin Core RPC cookie file from <COOKIE_FILE>.")]
  pub(crate) cookie_file: Option<PathBuf>,
  #[arg(long, help = "Use <CSP_ORIGIN> in Content-Security-Policy header. Set this to the public-facing URL of your ord instance.")]
  pub(crate) csp_origin: Option<String>,
  #[arg(long, help = "Store index in <DATA_DIR>.")]
  pub(crate) data_dir: Option<PathBuf>,
  #[arg(
    long,
    help = "Set index cache to <DB_CACHE_SIZE> bytes. By default takes 1/4 of available RAM."
  )]
  pub(crate) db_cache_size: Option<usize>,
  #[arg(
    long,
    help = "Don't look for inscriptions below <FIRST_INSCRIPTION_HEIGHT>."
  )]
  pub(crate) first_inscription_height: Option<u32>,
  #[arg(
    long,
    help = "Don't look for junes below <FIRST_JUNE_HEIGHT>."
  )]
  pub(crate) first_june_height: Option<u32>,
  #[arg(
    long,
    help = "Don't look for junk20 tokens below <FIRST_JUNK20_HEIGHT>."
  )]
  pub(crate) first_junk20_height: Option<u32>,
  #[arg(long, help = "Limit index to <HEIGHT_LIMIT> blocks.")]
  pub(crate) height_limit: Option<u32>,
  #[arg(long, help = "Use index at <INDEX>.")]
  pub(crate) index: Option<PathBuf>,
  #[arg(long, help = "Track junk20 tokens and balances.")]
  pub(crate) index_junk20: bool,
  #[arg(
    long,
    help = "Track location of junes. JUNES ARE IN AN UNFINISHED PRE-ALPHA STATE AND SUBJECT TO CHANGE AT ANY TIME."
  )]
  pub(crate) index_junes: bool,
  #[arg(long, help = "Track location of all satoshis.")]
  pub(crate) index_sats: bool,
  #[arg(long, help = "Store transactions in index.")]
  pub(crate) index_transactions: bool,
  #[arg(long, short, help = "Use regtest. Equivalent to `--chain regtest`.")]
  pub(crate) regtest: bool,
  #[arg(long, help = "Connect to Junkcoin Core RPC at <RPC_URL>.")]
  pub(crate) rpc_url: Option<String>,
  #[arg(
    long,
    help = "Number of parallel requests to junkcoin node."
  )]
  pub(crate) nr_parallel_requests: Option<usize>,
  #[arg(long, short, help = "Use signet. Equivalent to `--chain signet`.")]
  pub(crate) signet: bool,
  #[arg(long, short, help = "Use testnet. Equivalent to `--chain testnet`.")]
  pub(crate) testnet: bool,
  #[arg(long, default_value = "ord", help = "Use wallet named <WALLET>.")]
  pub(crate) wallet: String,
}

impl Options {
  pub(crate) fn chain(&self) -> Chain {
    if self.signet {
      Chain::Signet
    } else if self.regtest {
      Chain::Regtest
    } else if self.testnet {
      Chain::Testnet
    } else {
      self.chain_argument
    }
  }

  pub(crate) fn csp_origin(&self) -> Option<String> {
    self.csp_origin.clone()
  }

  pub(crate) fn first_inscription_height(&self) -> u32 {
    if self.chain() == Chain::Regtest {
      self.first_inscription_height.unwrap_or(0)
    } else if integration_test() {
      0
    } else {
      self
        .first_inscription_height
        .unwrap_or_else(|| self.chain().first_inscription_height())
    }
  }

  pub(crate) fn first_june_height(&self) -> u32 {
    if self.chain() == Chain::Regtest {
      self.first_june_height.unwrap_or(0)
    } else if integration_test() {
      0
    } else {
      self
          .first_june_height
          .unwrap_or_else(|| self.chain().first_june_height())
    }
  }

  pub(crate) fn first_junk20_height(&self) -> u32 {
    if self.chain() == Chain::Regtest {
      self.first_junk20_height.unwrap_or(0)
    } else if integration_test() {
      0
    } else {
      self
        .first_junk20_height
        .unwrap_or_else(|| self.chain().first_junk20_height())
    }
  }

  pub(crate) fn index_junes(&self) -> bool {
    self.index_junes
  }

  pub(crate) fn rpc_url(&self) -> String {
    self.rpc_url.clone().unwrap_or_else(|| {
      format!(
        "127.0.0.1:{}/{}",
        self.chain().default_rpc_port(),
        self.wallet
      )
    })
  }

  pub(crate) fn nr_parallel_requests(&self) -> usize {
    self.nr_parallel_requests.clone().unwrap_or(12)
  }

  pub(crate) fn cookie_file(&self) -> Result<PathBuf> {
    if let Some(cookie_file) = &self.cookie_file {
      return Ok(cookie_file.clone());
    }

    let path = if let Some(junkcoin_data_dir) = &self.junkcoin_data_dir {
      junkcoin_data_dir.clone()
    } else if cfg!(target_os = "linux") {
      dirs::home_dir()
        .ok_or_else(|| anyhow!("failed to retrieve home dir"))?
        .join(".junkcoin")
    } else {
      dirs::data_dir()
        .ok_or_else(|| anyhow!("failed to retrieve data dir"))?
        .join("Junkcoin")
    };

    let path = self.chain().join_with_data_dir(&path);

    Ok(path.join(".cookie"))
  }

  pub(crate) fn data_dir(&self) -> Result<PathBuf> {
    let base = match &self.data_dir {
      Some(base) => base.clone(),
      None => dirs::data_dir()
        .ok_or_else(|| anyhow!("failed to retrieve data dir"))?
        .join("ord"),
    };

    Ok(self.chain().join_with_data_dir(&base))
  }

  pub(crate) fn load_config(&self) -> Result<Config> {
    match &self.config {
      Some(path) => Ok(serde_yaml::from_reader(File::open(path)?)?),
      None => match &self.config_dir {
        Some(dir) if dir.join("ord.yaml").exists() => {
          Ok(serde_yaml::from_reader(File::open(dir.join("ord.yaml"))?)?)
        }
        Some(_) | None => Ok(Default::default()),
      },
    }
  }

  fn format_junkcoin_core_version(version: usize) -> String {
    format!(
      "{}.{}.{}.{}",
      version / 1000000,
      version % 1000000 / 10000,
      version % 10000 / 100,
      version % 100
    )
  }

  pub(crate) fn junkcoin_rpc_client(&self) -> Result<Client> {
    let cookie_file = self
      .cookie_file()
      .map_err(|err| anyhow!("failed to get cookie file path: {err}"))?;

    let rpc_url = self.rpc_url();

    log::info!(
      "Connecting to Junkcoin Core RPC server at {rpc_url} using credentials from `{}`",
      cookie_file.display()
    );

    let client =
      Client::new(&rpc_url, Auth::CookieFile(cookie_file.clone())).with_context(|| {
        format!(
          "failed to connect to Junkcoin Core RPC at {rpc_url} using cookie file {}",
          cookie_file.display()
        )
      })?;

    let rpc_chain = match client.get_blockchain_info()?.chain.as_str() {
      "main" => Chain::Mainnet,
      "test" => Chain::Testnet,
      "regtest" => Chain::Regtest,
      "signet" => Chain::Signet,
      other => bail!("Junkcoin RPC server on unknown chain: {other}"),
    };

    let ord_chain = self.chain();

    if rpc_chain != ord_chain {
      bail!("Junkcoin RPC server is on {rpc_chain} but ord is on {ord_chain}");
    }

    Ok(client)
  }

  pub(crate) fn junkcoin_rpc_client_for_wallet_command(&self, create: bool) -> Result<Client> {
    let client = self.junkcoin_rpc_client()?;

    const MIN_VERSION: usize = 1140600;

    let junkcoin_version = client.version()?;
    if junkcoin_version < MIN_VERSION {
      bail!(
        "Junkcoin Core {} or newer required, current version is {}",
        Self::format_junkcoin_core_version(MIN_VERSION),
        Self::format_junkcoin_core_version(junkcoin_version),
      );
    }

    if !create {
      if !client.list_wallets()?.contains(&self.wallet) {
        client.load_wallet(&self.wallet)?;
      }

      let descriptors = client.list_descriptors(None)?.descriptors;

      let tr = descriptors
        .iter()
        .filter(|descriptor| descriptor.desc.starts_with("tr("))
        .count();

      let rawtr = descriptors
        .iter()
        .filter(|descriptor| descriptor.desc.starts_with("rawtr("))
        .count();

      if tr != 2 || descriptors.len() != 2 + rawtr {
        bail!("wallet \"{}\" contains unexpected output descriptors, and does not appear to be an `ord` wallet, create a new wallet with `ord wallet create`", self.wallet);
      }
    }

    Ok(client)
  }
}

#[cfg(test)]
mod tests {
  use {super::*, bitcoin::Network, std::path::Path};

  #[test]
  fn rpc_url_overrides_network() {
    assert_eq!(
      Arguments::try_parse_from(["ord", "--rpc-url=127.0.0.1:1234", "--chain=signet", "index"])
        .unwrap()
        .options
        .rpc_url(),
      "127.0.0.1:1234"
    );
  }

  #[test]
  fn cookie_file_overrides_network() {
    assert_eq!(
      Arguments::try_parse_from(["ord", "--cookie-file=/foo/bar", "--chain=signet", "index"])
        .unwrap()
        .options
        .cookie_file()
        .unwrap(),
      Path::new("/foo/bar")
    );
  }

  #[test]
  fn use_default_network() {
    let arguments = Arguments::try_parse_from(["ord", "index"]).unwrap();

    assert_eq!(arguments.options.rpc_url(), "127.0.0.1:22555/wallet/ord");

    assert!(arguments
      .options
      .cookie_file()
      .unwrap()
      .ends_with(".cookie"));
  }

  #[test]
  fn uses_network_defaults() {
    let arguments = Arguments::try_parse_from(["ord", "--chain=signet", "index"]).unwrap();

    assert_eq!(arguments.options.rpc_url(), "127.0.0.1:38332/wallet/ord");

    assert!(arguments
      .options
      .cookie_file()
      .unwrap()
      .display()
      .to_string()
      .ends_with(if cfg!(windows) {
        r"\signet\.cookie"
      } else {
        "/signet/.cookie"
      }));
  }

  #[test]
  fn mainnet_cookie_file_path() {
    let cookie_file = Arguments::try_parse_from(["ord", "index"])
      .unwrap()
      .options
      .cookie_file()
      .unwrap()
      .display()
      .to_string();

    assert!(cookie_file.ends_with(if cfg!(target_os = "linux") {
      "/.junkcoin/.cookie"
    } else if cfg!(windows) {
      r"\Junkcoin\.cookie"
    } else {
      "/Junkcoin/.cookie"
    }))
  }

  #[test]
  fn othernet_cookie_file_path() {
    let arguments = Arguments::try_parse_from(["ord", "--chain=signet", "index"]).unwrap();

    let cookie_file = arguments
      .options
      .cookie_file()
      .unwrap()
      .display()
      .to_string();

    assert!(cookie_file.ends_with(if cfg!(target_os = "linux") {
      "/.junkcoin/signet/.cookie"
    } else if cfg!(windows) {
      r"\Junkcoin\signet\.cookie"
    } else {
      "/Junkcoin/signet/.cookie"
    }));
  }

  #[test]
  fn cookie_file_defaults_to_junkcoin_data_dir() {
    let arguments =
      Arguments::try_parse_from(["ord", "--junkcoin-data-dir=foo", "--chain=signet", "index"])
        .unwrap();

    let cookie_file = arguments
      .options
      .cookie_file()
      .unwrap()
      .display()
      .to_string();

    assert!(cookie_file.ends_with(if cfg!(windows) {
      r"foo\signet\.cookie"
    } else {
      "foo/signet/.cookie"
    }));
  }

  #[test]
  fn mainnet_data_dir() {
    let data_dir = Arguments::try_parse_from(["ord", "index"])
      .unwrap()
      .options
      .data_dir()
      .unwrap()
      .display()
      .to_string();
    assert!(
      data_dir.ends_with(if cfg!(windows) { r"\ord" } else { "/ord" }),
      "{data_dir}"
    );
  }

  #[test]
  fn othernet_data_dir() {
    let data_dir = Arguments::try_parse_from(["ord", "--chain=signet", "index"])
      .unwrap()
      .options
      .data_dir()
      .unwrap()
      .display()
      .to_string();
    assert!(
      data_dir.ends_with(if cfg!(windows) {
        r"\ord\signet"
      } else {
        "/ord/signet"
      }),
      "{data_dir}"
    );
  }

  #[test]
  fn network_is_joined_with_data_dir() {
    let data_dir =
      Arguments::try_parse_from(["ord", "--chain=signet", "--data-dir", "foo", "index"])
        .unwrap()
        .options
        .data_dir()
        .unwrap()
        .display()
        .to_string();
    assert!(
      data_dir.ends_with(if cfg!(windows) {
        r"foo\signet"
      } else {
        "foo/signet"
      }),
      "{data_dir}"
    );
  }

  #[test]
  fn network_accepts_aliases() {
    fn check_network_alias(alias: &str, suffix: &str) {
      let data_dir = Arguments::try_parse_from(["ord", "--chain", alias, "index"])
        .unwrap()
        .options
        .data_dir()
        .unwrap()
        .display()
        .to_string();

      assert!(data_dir.ends_with(suffix), "{data_dir}");
    }

    check_network_alias("main", "ord");
    check_network_alias("mainnet", "ord");
    check_network_alias(
      "regtest",
      if cfg!(windows) {
        r"ord\regtest"
      } else {
        "ord/regtest"
      },
    );
    check_network_alias(
      "signet",
      if cfg!(windows) {
        r"ord\signet"
      } else {
        "ord/signet"
      },
    );
    check_network_alias(
      "test",
      if cfg!(windows) {
        r"ord\testnet3"
      } else {
        "ord/testnet3"
      },
    );
    check_network_alias(
      "testnet",
      if cfg!(windows) {
        r"ord\testnet3"
      } else {
        "ord/testnet3"
      },
    );
  }

  #[test]
  fn rpc_server_chain_must_match() {
    let rpc_server = test_bitcoincore_rpc::builder()
      .network(Network::Testnet)
      .build();

    let tempdir = TempDir::new().unwrap();

    let cookie_file = tempdir.path().join(".cookie");
    fs::write(&cookie_file, "username:password").unwrap();

    let options = Options::try_parse_from([
      "ord",
      "--cookie-file",
      cookie_file.to_str().unwrap(),
      "--rpc-url",
      &rpc_server.url(),
    ])
    .unwrap();

    assert_eq!(
      options.junkcoin_rpc_client().unwrap_err().to_string(),
      "Junkcoin RPC server is on testnet but ord is on mainnet"
    );
  }

  #[test]
  fn chain_flags() {
    Arguments::try_parse_from(["ord", "--signet", "--chain", "signet", "index"]).unwrap_err();
    assert_eq!(
      Arguments::try_parse_from(["ord", "--signet", "index"])
        .unwrap()
        .options
        .chain(),
      Chain::Signet
    );
    assert_eq!(
      Arguments::try_parse_from(["ord", "-s", "index"])
        .unwrap()
        .options
        .chain(),
      Chain::Signet
    );

    Arguments::try_parse_from(["ord", "--regtest", "--chain", "signet", "index"]).unwrap_err();
    assert_eq!(
      Arguments::try_parse_from(["ord", "--regtest", "index"])
        .unwrap()
        .options
        .chain(),
      Chain::Regtest
    );
    assert_eq!(
      Arguments::try_parse_from(["ord", "-r", "index"])
        .unwrap()
        .options
        .chain(),
      Chain::Regtest
    );

    Arguments::try_parse_from(["ord", "--testnet", "--chain", "signet", "index"]).unwrap_err();
    assert_eq!(
      Arguments::try_parse_from(["ord", "--testnet", "index"])
        .unwrap()
        .options
        .chain(),
      Chain::Testnet
    );
    assert_eq!(
      Arguments::try_parse_from(["ord", "-t", "index"])
        .unwrap()
        .options
        .chain(),
      Chain::Testnet
    );
  }

  #[test]
  fn wallet_flag_overrides_default_name() {
    assert_eq!(
      Arguments::try_parse_from(["ord", "wallet", "create"])
        .unwrap()
        .options
        .wallet,
      "ord"
    );

    assert_eq!(
      Arguments::try_parse_from(["ord", "--wallet", "foo", "wallet", "create"])
        .unwrap()
        .options
        .wallet,
      "foo"
    )
  }

  #[test]
  fn default_config_is_returned_if_config_option_is_not_passed() {
    assert_eq!(
      Arguments::try_parse_from(["ord", "index"])
        .unwrap()
        .options
        .load_config()
        .unwrap(),
      Default::default()
    );
  }

  #[test]
  fn config_is_loaded_from_config_option_path() {
    let id = "8d363b28528b0cb86b5fd48615493fb175bdf132d2a3d20b4251bba3f130a5abi0"
      .parse::<InscriptionId>()
      .unwrap();

    let tempdir = TempDir::new().unwrap();
    let path = tempdir.path().join("ord.yaml");
    fs::write(&path, format!("hidden:\n- \"{id}\"")).unwrap();

    assert_eq!(
      Arguments::try_parse_from(["ord", "--config", path.to_str().unwrap(), "index",])
        .unwrap()
        .options
        .load_config()
        .unwrap(),
      Config {
        hidden: iter::once(id).collect(),
      }
    );
  }

  #[test]
  fn config_is_loaded_from_config_dir_option_path() {
    let id = "8d363b28528b0cb86b5fd48615493fb175bdf132d2a3d20b4251bba3f130a5abi0"
      .parse::<InscriptionId>()
      .unwrap();

    let tempdir = TempDir::new().unwrap();

    fs::write(
      tempdir.path().join("ord.yaml"),
      format!("hidden:\n- \"{id}\""),
    )
    .unwrap();

    assert_eq!(
      Arguments::try_parse_from([
        "ord",
        "--config-dir",
        tempdir.path().to_str().unwrap(),
        "index",
      ])
      .unwrap()
      .options
      .load_config()
      .unwrap(),
      Config {
        hidden: iter::once(id).collect(),
      }
    );
  }

  #[test]
  fn index_junes_only_returns_true_if_index_junes_flag_is_passed_and_not_on_mainnnet() {
    assert!(Arguments::try_parse_from([
      "ord",
      "--chain=signet",
      "--index-junes",
      "index",
      "update"
    ])
    .unwrap()
    .options
    .index_junes(),);
    assert!(!Arguments::try_parse_from([
      "ord",
      "--index-junes",
      "index",
      "update"
    ])
    .unwrap()
    .options
    .index_junes(),);
    assert!(!Arguments::try_parse_from(["ord", "index", "update"])
      .unwrap()
      .options
      .index_junes(),);
  }
}
