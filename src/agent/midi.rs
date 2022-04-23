use midir::{MidiInput, Ignore};
use std::path::Path;

pub fn listen_midi(){
    println!("Starting MIDI listener");
    println!("Reading cached devicename...");
    // get cached devicename
    let device = crate::state::get_device();

    // connect to the midi device
    println!("Connecting to MIDI device...");
    let mut midi_in = MidiInput::new("midir reading input").expect("Failed to open input");
    midi_in.ignore(Ignore::None);
    let in_ports = midi_in.ports();
    let in_port = in_ports.get(1).expect("Failed to get MIDI port");
    let input_port_name = midi_in.port_name(in_port).expect("Failed to get port name");

    // listen for keypresses
    let _conn_in = midi_in.connect(in_port, "midir-read-input", move |stamp, message, _| {
        // https://www.recordingblogs.com/wiki/status-byte-of-a-midi-message
        let message_status = message[0];
        // ignore MIDI clock messages
        if message_status != 248 && message.len() > 1 {
            println!("Message Length: {}", message.len());
            let message_data = message[1];
            println!("Status message: {}", message_status);
            println!("Message Data: {}", message_data);
            let converted = crate::midi::convert::byte_to_enum(message_data);
        }
    }, ()).expect("Failed to connect to device");
    println!("Connection open, reading data from {}", input_port_name);
    loop {
        if Path::new("/tmp/rbxmidi.midi_worker_stop").exists() {
            println!("Exiting, got shutdown message");
            std::fs::remove_file("/tmp/rbxmidi.midi_worker_stop").expect("Failed to remove stop file");
            break;
        }
    }
}