// Menyu, dialog va banner matnlari (uz/ru/en). Mahalliy sahifa matnlari ui/app.js da.

pub fn t(lang: &str, key: &str) -> String {
    let (uz, ru, en) = match key {
        "menu.app" => ("EduX", "EduX", "EduX"),
        "menu.server" => ("Server manzili…", "Адрес сервера…", "Server address…"),
        "menu.browser" => ("Brauzerda ochish", "Открыть в браузере", "Open in browser"),
        "menu.about" => ("EduX haqida", "О программе EduX", "About EduX"),
        "menu.quit" => ("Chiqish", "Выход", "Quit"),
        "menu.edit" => ("Tahrirlash", "Правка", "Edit"),
        "menu.view" => ("Ko'rinish", "Вид", "View"),
        "menu.reload" => ("Qayta yuklash", "Обновить", "Reload"),
        "menu.back" => ("Orqaga", "Назад", "Back"),
        "menu.forward" => ("Oldinga", "Вперёд", "Forward"),
        "menu.zoom_in" => ("Kattalashtirish", "Увеличить", "Zoom in"),
        "menu.zoom_out" => ("Kichiklashtirish", "Уменьшить", "Zoom out"),
        "menu.zoom_reset" => ("Asl o'lcham", "Исходный размер", "Actual size"),
        "menu.fullscreen" => ("To'liq ekran", "Полноэкранный режим", "Full screen"),
        "menu.help" => ("Yordam", "Справка", "Help"),
        "menu.help_center" => ("Yordam markazi", "Центр помощи", "Help center"),
        "menu.updates" => ("Yangilanishni tekshirish", "Проверить обновления", "Check for updates"),
        "about.title" => ("EduX haqida", "О программе EduX", "About EduX"),
        "about.app" => ("Ilova versiyasi", "Версия приложения", "App version"),
        "about.server" => ("Server", "Сервер", "Server"),
        "about.server_version" => ("Server versiyasi", "Версия сервера", "Server version"),
        "about.unknown" => ("noma'lum", "неизвестно", "unknown"),
        "update.title" => ("Yangi versiya mavjud", "Доступна новая версия", "A new version is available"),
        "update.download" => ("Yuklab olish", "Скачать", "Download"),
        "update.later" => ("Keyinroq", "Позже", "Later"),
        "update.none" => ("Sizda eng so'nggi versiya o'rnatilgan.", "У вас установлена последняя версия.", "You have the latest version."),
        "save.title" => ("Faylni saqlash", "Сохранить файл", "Save file"),
        "save.done" => ("Fayl saqlandi", "Файл сохранён", "File saved"),
        "save.failed" => ("Faylni saqlab bo'lmadi", "Не удалось сохранить файл", "Could not save the file"),
        _ => (key, key, key),
    };
    match lang {
        "ru" => ru,
        "en" => en,
        _ => uz,
    }
    .to_string()
}
