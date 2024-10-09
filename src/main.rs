use clap::{Parser, Subcommand};
use rusqlite::params;
use std::io::Write;
use uuid::Uuid;

#[derive(Parser)]
struct Options {
    #[command(subcommand)]
    subcommand: Command,
}

#[derive(Subcommand)]
enum Command {
    Start {
        #[arg(long)]
        command: String,
        #[arg(long)]
        cwd: String,
    },
    Finish {
        #[arg(long)]
        exit_code: isize,
        #[arg(long)]
        id: Uuid,
    },
}

fn main() -> anyhow::Result<()> {
    let options = Options::parse();

    let conn = rusqlite::Connection::open("/Users/clark/code/shelldb/history.db")?;

    conn.pragma_update(None, "BUSY_TIMEOUT", 5_000)?;

    conn.execute(
        "create table if not exists history (
        id text primary key,
        cwd text not null,
        command text not null,
        exit_code integer,
        started_at datetime default current_timestamp not null,
        ended_at datetime
    )",
        [],
    )?;

    match options.subcommand {
        Command::Start { cwd, command } => {
            let id = uuid::Uuid::new_v4();

            conn.execute(
                "insert into history (id, cwd, command) values (?1, ?2, ?3)",
                [id.to_string(), cwd, command],
            )?;

            // write the id to stdout so we can use it
            // to link back to this event with the exit code
            let mut stdout = std::io::stdout();
            write!(stdout, "{}", id)?;
            stdout.flush()?;
        }
        Command::Finish { exit_code, id } => {
            conn.execute(
                "
                update history
                set
                    exit_code = ?1,
                    ended_at = current_timestamp
                where id = ?2",
                params![exit_code, id.to_string()],
            )?;
        }
    }

    Ok(())
}
