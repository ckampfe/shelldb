use clap::{Parser, Subcommand};
use rusqlite::params;
use std::io::Write;
use std::path::PathBuf;
use uuid::Uuid;

/// record shell history in a SQLite database
#[derive(Parser)]
struct Options {
    #[command(subcommand)]
    subcommand: Command,
    /// set this to override the default history database path.
    #[arg(env = "SHELLDB_HISTORY_DB_PATH")]
    database_path: Option<PathBuf>,
}

#[derive(Subcommand)]
enum Command {
    /// record the start of a shell command and
    /// the directory it was executed in
    Start {
        /// the command to record
        #[arg(long)]
        command: String,
        /// the working directory of the command
        #[arg(long)]
        working_directory: String,
    },
    /// record a command's exit code
    Finish {
        /// the exit code of a command that just completed
        #[arg(long)]
        exit_code: isize,
        /// the ID of the since-completed command, so
        /// we can record its exit code
        #[arg(long)]
        id: Uuid,
    },
}

fn main() -> anyhow::Result<()> {
    let options = Options::parse();

    let database_path = if let Some(database_path) = options.database_path {
        database_path
    } else {
        let mut database_path = directories::ProjectDirs::from("", "", "shelldb")
            .expect("must be able to find home directory")
            .data_local_dir()
            .to_path_buf();

        std::fs::create_dir_all(&database_path)?;
        database_path.push("history.db");
        database_path
    };

    let conn = rusqlite::Connection::open(database_path)?;

    // there really should not be any possibility
    // of long-running write transactions that would result in write lock contention,
    // but this is a "just in case" setting
    conn.pragma_update(None, "BUSY_TIMEOUT", 5_000)?;

    conn.execute(
        "create table if not exists history (
        id text primary key,
        command text not null,
        exit_code integer,
        working_directory text not null,
        started_at datetime default current_timestamp not null,
        ended_at datetime
    )",
        [],
    )?;

    match options.subcommand {
        Command::Start {
            working_directory,
            command,
        } => {
            let id = uuid::Uuid::new_v4();

            conn.execute(
                "insert into history (id, working_directory, command) values (?1, ?2, ?3)",
                [id.to_string(), working_directory, command],
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
