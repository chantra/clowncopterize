use clap::Parser;

#[clowncopterize::clowncopterize]
#[derive(Parser, Debug)]
struct Cli {
    /// Optional name to operate on
    name: Option<String>,

    /// Turn debugging information on
    #[arg(long)]
    clowntown_this: bool,

    /// lists test values
    #[arg(long)]
    clowntown_that: bool,
}

#[test]
fn test_default_to_false() {
    let cli = Cli::try_parse_from(vec!["prog"]).unwrap();
    assert_eq!(cli.clowntown_this, false);
    assert_eq!(cli.clowntown_that, false);
    assert_eq!(cli.clowncopterize, false);
}

#[test]
fn test_set_all_to_true() {
    let cli = Cli::try_parse_from(vec!["prog", "--clowncopterize"]).unwrap();
    assert_eq!(cli.clowntown_this, true);
    assert_eq!(cli.clowntown_that, true);
    assert_eq!(cli.clowncopterize, true);
}
