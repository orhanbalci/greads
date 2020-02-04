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
}

fn main() {
    let sd = ShellData {
        rt: Arc::new(Mutex::new(Runtime::new().unwrap())),
    };

    let mut shell = Shell::new(sd);
    shell.new_command_noargs("hello", "Say 'hello' to the world", |io, _sd| {
        writeln!(io, "Hello World !!!")?;
        Ok(())
    });

    shell.new_command_noargs("authorshow", "show author info", |io, sd| {
        let sp = Spinner::new(Spinners::Dots9, "Fetching author".into());
        let mut runtime = sd.rt.lock().unwrap();

        let author = runtime
            .block_on(GreadsClient::new().author_show(1285555))
            .unwrap();
        sp.stop();
        write!(io, "{}", ansi_escapes::EraseLines(1)).expect("Can not clear terminal");
        author.to_table(io);
        Ok(())
    });

    shell.new_command_noargs("authorlist", "list authors books", |io, sd| {
        let sp = Spinner::new(Spinners::Dots9, "Fetching author books".into());

        let mut runtime = sd.rt.lock().unwrap();

        let books = runtime
            .block_on(GreadsClient::new().books().get_by_author_id(1285555, 1))
            .unwrap();

        sp.stop();
        write!(io, "{}", ansi_escapes::EraseLines(1)).expect("Can not write result to output");
        books.to_table(io);
        //table.print(io).expect("Can not print authors book result");

        Ok(())
    });

    shell.new_command_noargs("bookisbn", "list book by isbn", |io, sd| {
        let sp = Spinner::new(Spinners::Dots9, "Fetching author books".into());

        let mut runtime = sd.rt.lock().unwrap();

        let books = runtime
            .block_on(GreadsClient::new().books().get_by_isbn("9755109285", 1))
            .unwrap();

        sp.stop();
        write!(io, "{}", ansi_escapes::EraseLines(1)).expect("Can not write result to output");

        let mut table = Table::new();
        table.add_row(row![
            "Book Id",
            "Title",
            "Number Of Pages",
            "Average Rating",
            "Description"
        ]);
        books.into_iter().for_each(|book| {
            table.add_row(row![
                book.id,
                book.title,
                book.num_pages.unwrap_or(0),
                book.average_rating.unwrap_or(0.0f32),
                book.description
                    .unwrap_or("".to_owned())
                    .chars()
                    .take(30)
                    .collect::<String>()
            ]);
        });
        table.print(io).expect("Can not print authors book result");

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
