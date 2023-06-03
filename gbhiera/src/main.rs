slint::include_modules!();

mod gbhiera;

use gbhiera::GbhieraMessage;
use tokio::sync::oneshot;
use tokio::{self, runtime::Runtime};

fn main() {
    let gbhiera_ui = GbhieraUI::new().unwrap();

    let gbhiera_worker = gbhiera::GbhieraWorker::new(&gbhiera_ui);

    gbhiera_ui.on_show_open_dialog({
        let cargo_channel = gbhiera_worker.channel.clone();
        move || cargo_channel.send(GbhieraMessage::ShowOpenDialog).unwrap()
    });
    gbhiera_ui.on_get_line({
        let cargo_channel = gbhiera_worker.channel.clone();
        move |line| {
            let rt = Runtime::new().unwrap();
            rt.block_on(async {
                let (sender, receiver) = oneshot::channel::<String>();
                cargo_channel
                    .send(GbhieraMessage::Expose {line, sender})
                    .unwrap();
                match receiver.await {
                    Ok(s) => s.into(),
                    Err(_) => "".into(),
                }
            })
        }
    });

    gbhiera_ui.run().unwrap();
    gbhiera_worker.join().unwrap();
}
