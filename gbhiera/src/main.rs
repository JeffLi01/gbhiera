slint::include_modules!();

mod gbhiera;

use gbhiera::GbhieraMessage;

fn main() {
    let mainview = GbhieraUI::new().unwrap();

    let gbhiera_worker = gbhiera::GbhieraWorker::new(&mainview);

    mainview.on_show_open_dialog({
        let cargo_channel = gbhiera_worker.channel.clone();
        move || cargo_channel.send(GbhieraMessage::ShowOpenDialog).unwrap()
    });

    mainview.run().unwrap();
    gbhiera_worker.join().unwrap();
}
