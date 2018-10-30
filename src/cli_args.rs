use clap::*;

pub fn build_cli() -> App<'static, 'static> {
	App::new(env!("CARGO_PKG_NAME"))
		.version(env!("CARGO_PKG_VERSION"))
		.author(env!("CARGO_PKG_AUTHORS"))
		.about(env!("CARGO_PKG_DESCRIPTION"))
		.setting(AppSettings::SubcommandRequired)
		.subcommand(SubCommand::with_name("install")
			.about("Downloads a package by name and builds it.")
			.arg(Arg::with_name("offline").long("offline").short("o")
				.help("forbid internet access while building packages"))
			.arg(Arg::with_name("TARGET")
				.help("target package")
				.required(true)
				.index(1)))
		.subcommand(SubCommand::with_name("jailbuild")
			.about("Builds package, using PKGBUILD and wrapping in jail")
			.arg(Arg::with_name("offline").long("offline").short("o")
				.help("forbid internet access while building packages"))
			.arg(Arg::with_name("DIR")
				.help("directory to build")
				.required(false)
				.default_value(".")
				.index(1)))
		.subcommand(SubCommand::with_name("tarcheck")
			.about("Checks *.tar archive")
			.arg(Arg::with_name("TARGET")
				.help("archive to check")
				.required(true)
				.index(1)))
}
