  // clap::ArgAction::Append 多次执行的值叠加
  /// 这是一串 file 的名字
  // #[arg(short, long, num_args = 1.., action = ArgAction::Append)]
  // files: Vec<String>,

  /// 隐式的 Action Set
  // #[arg(short, long, value_name = "File Path")]
  // info: Option<PathBuf>,

  /// --test 执行次数
  // #[arg(short, long, action = ArgAction::Count)]
  // test: u8,

  /// clap::ArgAction::Set 只能设置一次
  // #[arg(short, long, default_value_t = String::from(""), action = ArgAction::Set)]
  // set: String,
