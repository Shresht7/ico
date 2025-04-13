mod cli;

fn main() {
    let cli = cli::parse();
    println!(
        "{} -> {} ({})",
        cli.input.display(),
        cli.output.display(),
        cli.sizes
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(",")
    )
}
