fn generate_page(pages, index) {
    let name = if index < 8 {
        "User " + (index + 1)
    } else {
        "Factory " + (index - 7)
    };
    let page = create_page(name);

    fill_page(page, index + 1)
}

fn fill_page(page, channel) {
    page = add_buttons(page, channel);
    page = add_faders(page, channel);
    page = add_move_buttons(page, channel);

    page
}

fn add_buttons(page, channel) {
    for button_index in range(0, 4) {
        let button_number = button_index + 1;
        let button_id = "button-" + (channel - 1) + "-" + button_index;
        page += note("Pad " + button_number, 9 + button_index)
            .channel(channel)
            .output(button_id);
    }
    for button_index in range(0, 4) {
        let button_name = button_index + 5;
        let button_id = "button-" + (channel - 1) + "-" + (button_index + 4);
        page += note("Pad " + button_name, 25 + button_index)
            .channel(channel)
            .output(button_id);
    }

    page
}

fn add_faders(page, channel) {
    for fader_index in range(0, 8) {
        let fader_name = fader_index + 1;
        page += cc("Knob 1-" + fader_name, 21 + fader_index).channel(channel);
    }

    for fader_index in range(0, 8) {
        let fader_name = fader_index + 1;
        page += cc("Knob 2-" + fader_name, 41 + fader_index).channel(channel);
    }

    page
}

fn add_move_buttons(page, channel) {
    let button_id = "button-" + (channel - 1) + "-";
    page += note("Down", 115).channel(channel).output(button_id + "9");
    page += note("Up", 114).channel(channel).output(button_id + "8");
    page += note("Left", 116).channel(channel).output(button_id + "10");
    page += note("Right", 117).channel(channel).output(button_id + "11");

    page
}

let pages = [];

for index in range(0, 16) {
    pages += [generate_page(pages, index)];
}

pages
