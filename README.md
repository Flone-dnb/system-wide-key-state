# About
This crate allows asking global (system-wide) key state (pressed or not) even when the app has no focus.<br>
Windows and Linux supported.
# Examples
// asking if Escape key is pressed right now<br>
let answer = is_key_pressed(KeyCode::KEsc);<br>
<br>
// asking if keyboard key T is pressed right now<br>
let answer = is_key_pressed(KeyCode::KT);<br>
