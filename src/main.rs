use greadslib::GreadsClient;
use shrust::{Shell, ShellIO};
use std::io::prelude::*;
use std::sync::{Arc, Mutex};
use tokio::runtime::Runtime;

fn main() {
    let mut rt = Arc::new(Mutex::new(Runtime::new().unwrap()));
    let mut shell = Shell::new(());
    shell.new_command_noargs("hello", "Say 'hello' to the world", |io, _| {
        writeln!(io, "Hello World !!!")?;
        Ok(())
    });

    shell.new_command_noargs("showauthor", "show author info", move |io, _| {
        let rt_ref = Arc::clone(&rt);
        let mut runtime = rt_ref.lock().unwrap();

        let author = runtime
            .block_on(GreadsClient::new().author_show(1285555))
            .unwrap();
        writeln!(io, "{}", author.id)?;
        writeln!(io, "{}", author.name)?;
        writeln!(io, "{:?}", author.gender)?;
        writeln!(io, "{:?}", author.hometown)?;
        writeln!(io, "{:?}", author.born_at)?;
        writeln!(io, "{:?}", author.died_at)?;
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
