# set_button_margin
Just an exaple of how to set button's margins

First create a button
let button = Button::with_label("Some Button");

After that you can set it's top, bottom, start, end margins:
button.set_margin_top(5);
button.set_margin_bottom(5);
button.set_margin_start(2);
button.set_margin_end(2);
