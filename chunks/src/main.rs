mod chunks;
mod plates;
mod slabs;

use chunks::Chunks;
use chunks_rs::{utils::load_css, Factory, GtkApp};
use plates::Plates;
use slabs::Slabs;

const STYLE: &str = "
window {
    background-color: transparent;
}

#clock, #storage, #volume, #weather, #picture, #welcome {
    font-size: 34px;
    font-family: feather;
    font-family: Iosevka;
    background-color: #000000;
    color: #FFFFFF;
    padding: 10px;
    border: 2px solid black;
    border-radius: 20px;
}

#storage, #volume, #weather{
    font-size: 24px;
}
";

fn main() {
    let factory = Factory::new("chunk.factory");

    let chunks = move |factory: GtkApp| {
        Chunks::storage(&factory);
        Chunks::clock(&factory);

        // Slabs::volume(&factory);

        // Plates::welcome(&factory);

        load_css(STYLE);
    };

    factory.pollute(chunks);
}
