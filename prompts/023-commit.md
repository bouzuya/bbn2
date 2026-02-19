Add out_dir field to Config

Add an optional out_dir field to Config and ConfigJson to allow
configuring the output directory via the config file. Update
Config::new to accept an out_dir parameter and expose it through
Config::out_dir(). Add a --out-dir option to the config subcommand
to set the value.
