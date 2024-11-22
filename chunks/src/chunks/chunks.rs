use chrono::Local;
use chunks_rs::{
    position::{Edge, EdgeConfig, Layer},
    utils::tag,
    widgets::Chunk,
    Application, Internal,
};

pub struct Chunks {}

impl Chunks {
    pub fn clock(factory: &Application) {
        let tag = tag("clock");
        let margins = vec![(Edge::Top, 20), (Edge::Right, 20)];
        let anchors = EdgeConfig::TOP_RIGHT.to_vec();

        let date_closure = || {
            let current_time = Local::now();

            let date = format!(
                "<span background='#000000' foreground='#FFFFFF' size='large'>{}</span>\n<span foreground='#fabbc2' size='small'>{}  </span><span foreground='#FF0110' weight='bold' size='small'>{}</span>",
                current_time.format("%a"),
                current_time.format("%b"),
                current_time.format("%d"),
            );

            date
        };

        let time_closure = || {
            let current_time = Local::now();

            let time = format!(
                "<span foreground='#FFFFFF' size='large'>{}</span><span foreground='#FF0110' weight='bold' size='small'>  {}</span>\n<span foreground='#FFFFFF' size='large'> {}</span>",
                current_time.format("%I"),
                current_time.format("%p"),
                current_time.format("%M"),
            );

            time
        };

        Internal::static_to_update(&tag, date_closure, 2, time_closure, 1);

        Chunk::new(
            factory.clone(),
            "Clock".to_string(),
            tag,
            margins,
            anchors,
            Layer::Bottom,
        )
        .build();
    }

    pub fn storage(factory: &Application) {
        let tag = tag("storage");
        let margins = vec![(Edge::Top, 20), (Edge::Right, 160)];
        let anchors = EdgeConfig::TOP_RIGHT.to_vec();

        let storage_closure = || {
            let text = format!(
                "<span foreground='#FF0000' size='large'>/ </span><span foreground='#FFFFFF' size='large'>{:.0}%</span>",
                Internal::get_storage(),
            );

            text
        };

        Internal::update_storage(&tag, storage_closure);

        Chunk::new(
            factory.clone(),
            "Storage".to_string(),
            tag,
            margins,
            anchors,
            Layer::Bottom,
        )
        .build();
    }
}
