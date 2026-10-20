// Windows'da release yig'ishda qo'shimcha konsol oynasi ochilmaydi.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    edux_desktop_lib::run()
}
