# About
This crate allows asking global (system-wide) key state (pressed or not) even when the app has no focus.<br>
Windows and Linux supported.
# Examples
<pre>
use system_wide_key_state::*;

// asking if Escape key is pressed right now
let answer = is_key_pressed(KeyCode::KEsc);

// asking if keyboard key T is pressed right now
let answer = is_key_pressed(KeyCode::KT);

// get key name
assert_eq!("T", get_key_name(KeyCode::KT));

// convert text to keycode
assert_eq!(string_to_key("T"), KeyCode::KT);
assert_eq!(string_to_key("some invalid text"), KeyCode::None);
</pre>
