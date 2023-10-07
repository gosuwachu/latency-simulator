use ncurses::addstr;

pub fn draw_progress_bar(percentage: i32, length: i32, task_name: &str) {
    let filled_amount = (percentage as f32 / 100.0 * length as f32) as i32;
    addstr("[");
    for i in 0..length {
        if i < filled_amount {
            addstr("=");
        } else {
            addstr(" ");
        }
    }
    addstr("]");
    addstr(format!(" {:3}% {}", percentage, task_name).as_str());
}
