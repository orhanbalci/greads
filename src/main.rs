use greadslib::GreadsClient;
use shrust::{Shell, ShellIO};
use std::io::prelude::*;
use std::sync::{Arc, Mutex};
use tokio::runtime::Runtime;

#[macro_use]
extern crate prettytable;
use prettytable::{Cell, Row, Table};

fn main() {
    let rt = Arc::new(Mutex::new(Runtime::new().unwrap()));
    let rt_clone = rt.clone();
    let mut shell = Shell::new(());
    shell.new_command_noargs("hello", "Say 'hello' to the world", |io, _| {
        writeln!(io, "Hello World !!!")?;
        Ok(())
    });

    shell.new_command_noargs("authorshow", "show author info", move |io, _| {
        let mut runtime = rt_clone.lock().unwrap();

        let author = runtime
            .block_on(GreadsClient::new().author_show(1285555))
            .unwrap();
        // Create the table
        let mut table = Table::new();

        // Add a row per time
        table.add_row(row!["Author Id", author.id]);
        table.add_row(row!["Name", author.name]);
        table.add_row(row!["Gender", author.gender.unwrap_or("".to_owned())]);
        table.add_row(row!["Hometown", author.hometown.unwrap_or("".to_owned())]);
        table.add_row(row!["Born at", author.born_at.unwrap_or("".to_owned())]);
        table.add_row(row!["Died at", author.died_at.unwrap_or("".to_owned())]);
        table.print(io);

        Ok(())
    });

    let rt_clone = rt.clone();
    shell.new_command_noargs("authorlist", "list authors books", move |io, _| {
        let mut runtime = rt_clone.lock().unwrap();

        let books = runtime
            .block_on(GreadsClient::new().author_list(1285555, 1))
            .unwrap();
        books.into_iter().for_each(|book| {
            writeln!(io, "");
            // Create the table
            let mut table = Table::new();

            // Add a row per time
            table.add_row(row!["Book Id", book.id]);
            table.add_row(row!["Title", book.title]);
            table.add_row(row!["Number of Pages", book.num_pages.unwrap_or(0)]);
            table.add_row(row![
                "Average Rating",
                book.average_rating.unwrap_or(0.0f32)
            ]);
            table.add_row(row![
                "Description",
                book.description.unwrap_or("".to_owned()).chars().take(30).collect::<String>()
            ]);
            table.print(io);
        });

        Ok(())
    });
    // shell.new_command_noargs("showauthor", "show author info", author_show_command);
    shell.run_loop(&mut ShellIO::default());
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
