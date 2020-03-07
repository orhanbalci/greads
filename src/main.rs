use config::Config;
use greadslib::GreadsClient;
use shrust::{Shell, ShellIO};
use spinners::{Spinner, Spinners};
use std::io::prelude::*;
use std::sync::{Arc, Mutex};
use tokio::runtime::Runtime;

#[macro_use]
extern crate prettytable;
use prettytable::Table;

#[derive(Clone)]
struct ShellData {
    rt: Arc<Mutex<Runtime>>,
    pub last_command: Option<GReadsCommand>,
    pub settings: config::Config,
}

#[derive(Clone)]
struct GReadsCommand {
    command: String,
    args: Vec<String>,
}

fn main() {
    let mut settings = config::Config::default();
    settings.merge(config::File::with_name("Settings")).unwrap();
    let sd = ShellData {
        rt: Arc::new(Mutex::new(Runtime::new().unwrap())),
        last_command: None,
        settings: settings,
    };

    let mut shell = Shell::new(sd);
    shell.new_command_noargs("hello", "Say 'hello' to the world", |io, _sd| {
        writeln!(io, "Hello World !!!")?;
        Ok(())
    });

    shell.new_shell_command("next", "go to next page ", 0, |io, sh, params| {
        {
            let mut page_arg: u32 = 0;
            let mut line = String::new();
            let mut command = String::new();
            match &sh.data().last_command {
                Some(lc) => {
                    page_arg = lc.args[0].parse::<u32>().unwrap();
                    command = lc.command.clone();
                    line = format!("{} {}", lc.command, page_arg + 1);
                    sh.eval(io, &line);
                }
                None => {
                    writeln!(io, "Non repeatable command");
                }
            }
            if !command.is_empty() {
                sh.data().last_command = Some(GReadsCommand {
                    command: command,
                    args: vec![format!("{}", page_arg + 1)],
                });
            }
        }

        Ok(())
    });

    shell.new_shell_command("prev", "go to previous page ", 0, |io, sh, params| {
        {
            let mut page_arg: u32 = 0;
            let mut command = String::new();
            match &sh.data().last_command {
                Some(lc) => {
                    page_arg = lc.args[0].parse::<u32>().unwrap();
                    if page_arg > 1 {
                        command = lc.command.clone();

                        let line = format!("{} {}", lc.command, page_arg - 1);
                        sh.eval(io, &line);
                    }
                }
                None => {
                    writeln!(io, "Non repeatable command");
                }
            }
            if !command.is_empty() {
                sh.data().last_command = Some(GReadsCommand {
                    command: command,
                    args: vec![format!("{}", page_arg - 1)],
                });
            }
        }

        Ok(())
    });

    shell.new_shell_command("authorshow", "show author info", 0, |io, sh, args| {
        let sp = Spinner::new(Spinners::Dots9, "Fetching author".into());
        let data = sh.data();

        let mut runtime = data.rt.lock().unwrap();

        let author = runtime
            .block_on(
                GreadsClient::new(
                    data.settings
                        .get_str("CLIENT_KEY")
                        .expect("can not get key"),
                    data.settings
                        .get_str("CLIENT_SECRET")
                        .expect("can not get secret"),
                )
                .author_show(1285555),
            )
            .unwrap();
        sp.stop();
        write!(io, "{}", ansi_escapes::EraseLines(1)).expect("Can not clear terminal");
        author.to_table(io);
        Ok(())
    });

    shell.new_shell_command("authorlist", "list authors books", 1, |io, sh, args| {
        let sp = Spinner::new(Spinners::Dots9, "Fetching author books".into());
        {
            let data = sh.data();
            let mut runtime = data.rt.lock().unwrap();
            let books = runtime
                .block_on(
                    GreadsClient::new(
                        data.settings
                            .get_str("CLIENT_KEY")
                            .expect("can not get key"),
                        data.settings
                            .get_str("CLIENT_SECRET")
                            .expect("can not get secret"),
                    )
                    .books()
                    .get_by_author_id(1285555, args[0].parse::<u32>().unwrap()),
                )
                .unwrap();
            sp.stop();
            write!(io, "{}", ansi_escapes::EraseLines(1)).expect("Can not write result to output");
            books.to_table(io);
        }

        sh.data().last_command = Some(GReadsCommand {
            command: "authorlist".to_string(),
            args: args.to_vec().iter().map(|s| s.to_string()).collect(),
        });
        //table.print(io).expect("Can not print authors book result");

        Ok(())
    });

    shell.new_shell_command("bookisbn", "list book by isbn", 0, |io, sh, args| {
        let sp = Spinner::new(Spinners::Dots9, "Fetching author books".into());
        let data = sh.data();
        let mut runtime = data.rt.lock().unwrap();

        let book = runtime.block_on(
            GreadsClient::new(
                data.settings
                    .get_str("CLIENT_KEY")
                    .expect("can not get key"),
                data.settings
                    .get_str("CLIENT_SECRET")
                    .expect("can not get secret"),
            )
            .books()
            .get_by_isbn("9755109285"),
        );
        if let Ok(b) = book {
            sp.stop();
            write!(io, "{}", ansi_escapes::EraseLines(1)).expect("Can not write result to output");
            b.to_table(io);
        } else {
            writeln!(io, "Can not find book").expect("Can not write result to output");
        }
        Ok(())
    });

    shell.new_shell_command("bookbyid", "list book by id", 1, |io, sh, args| {
        let sp = Spinner::new(Spinners::Dots9, "Fetching books".into());
        let data = sh.data();
        let mut runtime = data.rt.lock().unwrap();

        let book = runtime
            .block_on(
                GreadsClient::new(
                    data.settings
                        .get_str("CLIENT_KEY")
                        .expect("can not get key"),
                    data.settings
                        .get_str("CLIENT_SECRET")
                        .expect("can not get secret"),
                )
                .books()
                .get_by_book_id(args[0]),
            )
            .unwrap();

        sp.stop();
        write!(io, "{}", ansi_escapes::EraseLines(1)).expect("Can not write result to output");
        book.to_table(io);
        Ok(())
    });

    shell.new_shell_command("bookbytitle", "list book by title", 1, |io, sh, args| {
        let sp = Spinner::new(Spinners::Dots9, "Fetching books".into());
        let data = sh.data();

        let mut runtime = data.rt.lock().unwrap();

        let book = runtime
            .block_on(
                GreadsClient::new(
                    data.settings
                        .get_str("CLIENT_KEY")
                        .expect("can not get key"),
                    data.settings
                        .get_str("CLIENT_SECRET")
                        .expect("can not get secret"),
                )
                .books()
                .get_by_title(args[0], ""),
            )
            .unwrap();

        sp.stop();
        write!(io, "{}", ansi_escapes::EraseLines(1)).expect("Can not write result to output");
        book.to_table(io);
        Ok(())
    });

    shell.new_command_noargs("isbntobookid", "isbn to book id", |io, sd| {
        let sp = Spinner::new(Spinners::Dots9, "Fetching book ids".into());

        let mut runtime = sd.rt.lock().unwrap();

        let book_id = runtime
            .block_on(
                GreadsClient::new(
                    sd.settings.get_str("CLIENT_KEY").expect("can not get key"),
                    sd.settings
                        .get_str("CLIENT_SECRET")
                        .expect("can not get secret"),
                )
                .books()
                .get_book_id_for_isbn(&"9755109285"),
            )
            .unwrap();

        sp.stop();
        write!(io, "{}", ansi_escapes::EraseLines(1)).expect("Can not write result to output");
        writeln!(io, "{:?}", book_id);
        Ok(())
    });

    shell.new_command_noargs("login", "login to goodreads", |io, sd| {
        let sp = Spinner::new(Spinners::Dots9, "Getting tokens".into());

        let mut runtime = sd.rt.lock().unwrap();

        let request_token_key = runtime
            .block_on(
                GreadsClient::new(
                    sd.settings.get_str("CLIENT_KEY").expect("can not get key"),
                    sd.settings
                        .get_str("CLIENT_SECRET")
                        .expect("can not get secret"),
                )
                .request_token(),
            )
            .unwrap();

        sp.stop();
        let token = request_token_key.unwrap();
        let tokens = token.split('&').collect::<Vec<_>>();
        write!(io, "{}", ansi_escapes::EraseLines(1)).expect("Can not write result to output");
        writeln!(io, "{:?}", tokens[0]);
        writeln!(io, "{:?}", tokens[1]);
        let auth_url = GreadsClient::new(
            sd.settings.get_str("CLIENT_KEY").expect("can not get key"),
            sd.settings
                .get_str("CLIENT_SECRET")
                .expect("can not get secret"),
        )
        .request_authorization_url(tokens[0]);
        writeln!(
            io,
            "Please visit  {} and enter resultant url here : ",
            auth_url
        );

        let stdin = std::io::BufReader::new(io.clone());
        let mut iter = stdin.lines().map(|l| l.unwrap());
        if let Some(mut line) = iter.next() {
            let question_position = line.chars().position(|s| s == '?').unwrap();
            let params = line[question_position..].split('&').collect::<Vec<_>>();
            let auth_token = runtime
                .block_on(
                    GreadsClient::new(
                        sd.settings.get_str("CLIENT_KEY").expect("can not get key"),
                        sd.settings
                            .get_str("CLIENT_SECRET")
                            .expect("can not get secret"),
                    )
                    .authorize_token(
                        tokens[0].split('=').collect::<Vec<_>>()[1],
                        tokens[1].split('=').collect::<Vec<_>>()[1],
                    ),
                )
                .unwrap();
            writeln!(io, "{:?}", auth_token);
            // todo token parse edilecek key secret ayrilacak``
            //sd.settings.set("auth_token",auth_token);
        }

        Ok(())
    });

    // shell.new_command_noargs("showauthor", "show author info", author_show_command);
    shell.run_loop(&mut ShellIO::default());
}

pub trait TableDisplay {
    fn to_table(&self, io: &mut ShellIO);
}

impl TableDisplay for greadslib::entity::GAuthor {
    fn to_table(&self, io: &mut ShellIO) {
        let mut table = Table::new();
        table.add_row(row!["Author Id", self.id]);
        table.add_row(row!["Name", self.name]);
        table.add_row(row![
            "Gender",
            self.gender.as_ref().unwrap_or(&"".to_owned())
        ]);
        table.add_row(row![
            "Hometown",
            self.hometown.as_ref().unwrap_or(&"".to_owned())
        ]);
        table.add_row(row![
            "Born at",
            self.born_at.as_ref().unwrap_or(&"".to_owned())
        ]);
        table.add_row(row![
            "Died at",
            self.died_at.as_ref().unwrap_or(&"".to_owned())
        ]);
        table.print(io).expect("Can not print author result");
    }
}

impl TableDisplay for Vec<greadslib::entity::GBook> {
    fn to_table(&self, io: &mut ShellIO) {
        let mut table = Table::new();
        table.add_row(row![
            "Book Id",
            "Title",
            "Number Of Pages",
            "Average Rating",
            "Description"
        ]);
        self.iter().for_each(|book| {
            table.add_row(row![
                book.id,
                book.title,
                book.num_pages.unwrap_or(0),
                book.average_rating.unwrap_or(0.0f32),
                book.description
                    .as_ref()
                    .unwrap_or(&"".to_owned())
                    .chars()
                    .take(30)
                    .collect::<String>()
            ]);
        });
        table.print(io).expect("Can not print book result");
    }
}

impl TableDisplay for greadslib::entity::GBook {
    fn to_table(&self, io: &mut ShellIO) {
        let mut table = Table::new();
        table.add_row(row![
            "Book Id",
            "ISBN",
            "Title",
            "Number Of Pages",
            "Average Rating",
            "Description"
        ]);
        table.add_row(row![
            self.id,
            self.isbn,
            self.title,
            self.num_pages.unwrap_or(0),
            self.average_rating.unwrap_or(0.0f32),
            self.description
                .as_ref()
                .unwrap_or(&"".to_owned())
                .chars()
                .take(30)
                .collect::<String>()
        ]);
        table.print(io).expect("Can not print book info");
    }
}
// pub fn author_show_command(io : &mut ShellIO, _ : &mut ()) -> Result<(), shrust::ExecError> {
//     let author  = futures::executor::block_on(GreadsClient::new().author_show(1285555)).unwrap();
//     writeln!(io, "{}", author.id)?;
//     writeln!(io, "{}", author.name)?;
//     writeln!(io, "{:?}", author.gender)?;
//     writeln!(io, "{:?}", author.hometown)?;
//     writeln!(io, "{:?}", author.born_at)?;
//     writeln!(io, "{:?}", author.died_at)?;
//     Ok(())
// }
