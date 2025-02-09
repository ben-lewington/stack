use wa::{
    parse::{parse_ops, Program},
    tokenise::Tokeniser,
};

fn usage(program: impl AsRef<str>, subcmd: Option<impl AsRef<str>>) -> anyhow::Result<()> {
    println!("usage: {} <subcommand> <arg> -- [flags]", program.as_ref());
    match subcmd {
        Some(sc) => match sc.as_ref() {
            "interpret" | "interp" | "i" => {
                println!("interpret, interp, i: construct and run wa IR");
                println!("  flags: TODO");
            }
            "compile" | "com" | "c" => {
                println!("compile, com, c: compile generated bytecode");
                println!("  flags: TODO");
            }
            "dump" | "d" => {
                println!("dump, d: dump generated bytecode to file");
                println!("  flags: TODO");
            }
            "help" | "h" => println!("prints help information"),
            s => anyhow::bail!("Unknown subcommand {s}"),
        },
        None => {
            println!("    subcommands:");
            println!("      - interpret, interp, i: construct and run wa IR");
            println!("          - <arg> is the path to the wa file");
            println!("      - compile, com, c: compile generated bytecode");
            println!("          - <arg> is the path to the wa file");
            println!("      - dump, d: dump generated bytecode to file");
            println!("          - <arg> is the path to the wa file");
            println!("      - help, h: print help information");
            println!("          - <arg> is a subcommand for more info");
        }
    }
    Ok(())
}

fn parse_program_from_file(file_name: impl AsRef<str>) -> anyhow::Result<Program> {
    let file = std::fs::read(file_name.as_ref())?;
    let ops = Tokeniser::new(file.as_ref())
        .into_iter()
        .collect::<Vec<_>>();
    Ok(parse_ops(ops, file_name.as_ref())?)
}

fn main() -> anyhow::Result<()> {
    let mut args = std::env::args();

    let program = args.next().unwrap_or("wa".into());

    if std::env::args().len() < 3 {
        usage(&program, None::<&str>)?;
        anyhow::bail!("Not enough arguments");
    }

    let subcmd = args.next().unwrap();
    let file_name = args.next().unwrap();

    match subcmd.as_ref() {
        "interpret" | "interp" | "i" => {
            let prog = parse_program_from_file(&file_name)?;
            wa::interp_program(&file_name, prog)?;
        }
        "compile" | "com" | "c" => todo!("compilation"),
        "dump" | "d" => todo!("dump"),
        "help" | "h" => usage(&program, Some(&file_name))?,
        _ => todo!(),
    }

    Ok(())
}
