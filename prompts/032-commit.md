Add --data-dir and --out-dir options to build command

Allow `bbn build` to run without a config file by specifying
`--data-dir` and `--out-dir` directly on the command line.
The config file is loaded only when these options are not provided.
