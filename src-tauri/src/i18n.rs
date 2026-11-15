// Menyu, dialog va banner matnlari (uz/ru/en; 1.5.0 dan oz — o'zbekcha kirill, yl — yangi lotin).
// Mahalliy sahifa matnlari ui/app.js da. oz/yl o'zbekcha matndan veb qoidalari bilan o'girilgan
// (backend resources/js/i18n/script.js) va qo'lda tekshirilgan.

pub fn t(lang: &str, key: &str) -> String {
    let (uz, ru, en) = match key {
        "menu.app" => ("EduX", "EduX", "EduX"),
        "menu.advanced" => ("Kengaytirilgan", "Дополнительно", "Advanced"),
        "menu.server" => ("Server manzili…", "Адрес сервера…", "Server address…"),
        "menu.browser" => ("Brauzerda ochish", "Открыть в браузере", "Open in browser"),
        "menu.about" => ("EduX haqida", "О программе EduX", "About EduX"),
        "menu.quit" => ("Chiqish", "Выход", "Quit"),
        "menu.edit" => ("Tahrirlash", "Правка", "Edit"),
        "edit.undo" => ("Bekor qilish", "Отменить", "Undo"),
        "edit.redo" => ("Qaytarish", "Повторить", "Redo"),
        "edit.cut" => ("Kesib olish", "Вырезать", "Cut"),
        "edit.copy" => ("Nusxa olish", "Копировать", "Copy"),
        "edit.paste" => ("Qo'yish", "Вставить", "Paste"),
        "edit.select_all" => ("Hammasini belgilash", "Выделить всё", "Select all"),
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
        "menu.faq" => ("Ko'p beriladigan savollar", "Частые вопросы", "FAQ"),
        "menu.support" => ("Murojaat yuborish", "Написать в поддержку", "Contact support"),
        "menu.updates" => ("Yangilanishni tekshirish", "Проверить обновления", "Check for updates"),
        "about.title" => ("EduX haqida", "О программе EduX", "About EduX"),
        "about.app" => ("Ilova versiyasi", "Версия приложения", "App version"),
        "about.server" => ("Server", "Сервер", "Server"),
        "about.server_version" => ("Server versiyasi", "Версия сервера", "Server version"),
        "about.unknown" => ("noma'lum", "неизвестно", "unknown"),
        "update.title" => ("Yangi versiya mavjud", "Доступна новая версия", "A new version is available"),
        "update.download" => ("Yuklab olish", "Скачать", "Download"),
        "update.text" => (
            "EduX {new} chiqdi, sizda {current}. Yuklab olib o'rnating — sozlamalar saqlanib qoladi.",
            "Вышла EduX {new}, у вас {current}. Скачайте и установите — настройки сохранятся.",
            "EduX {new} is out, you have {current}. Download and install it — your settings are kept.",
        ),
        "update.later" => ("Keyinroq", "Позже", "Later"),
        "update.close" => ("Yopish", "Закрыть", "Close"),
        "update.none" => ("Sizda eng so'nggi versiya o'rnatilgan.", "У вас установлена последняя версия.", "You have the latest version."),
        "offline.text" => (
            "Internet aloqasi yo'q. Sahifa ochiq qoladi — aloqa tiklanishi bilan ish davom etadi.",
            "Нет подключения к интернету. Страница останется открытой — работа продолжится, когда связь восстановится.",
            "No internet connection. The page stays open — work continues once the connection is back.",
        ),
        "save.title" => ("Faylni saqlash", "Сохранить файл", "Save file"),
        "save.done" => ("Fayl saqlandi", "Файл сохранён", "File saved"),
        "save.failed" => ("Faylni saqlab bo'lmadi", "Не удалось сохранить файл", "Could not save the file"),
        _ => (key, key, key),
    };
    match lang {
        "ru" => ru.to_string(),
        "en" => en.to_string(),
        "oz" | "yl" => script(lang, key).map(str::to_string).unwrap_or_else(|| uz.to_string()),
        _ => uz.to_string(),
    }
}

/// O'zbekcha kirill (oz) va yangi lotin (yl) matnlari.
fn script(lang: &str, key: &str) -> Option<&'static str> {
    let (oz, yl) = match key {
        "menu.app" => ("EduX", "EduX"),
        "menu.advanced" => ("Кенгайтирилган", "Kengaytirilgan"),
        "menu.server" => ("Сервер манзили…", "Server manzili…"),
        "menu.browser" => ("Браузерда очиш", "Brauzerda oçiş"),
        "menu.about" => ("EduX ҳақида", "EduX haqida"),
        "menu.quit" => ("Чиқиш", "Çiqiş"),
        "menu.edit" => ("Таҳрирлаш", "Tahrirlaş"),
        "edit.undo" => ("Бекор қилиш", "Bekor qiliş"),
        "edit.redo" => ("Қайтариш", "Qaytariş"),
        "edit.cut" => ("Кесиб олиш", "Kesib oliş"),
        "edit.copy" => ("Нусха олиш", "Nusxa oliş"),
        "edit.paste" => ("Қўйиш", "Qóyiş"),
        "edit.select_all" => ("Ҳаммасини белгилаш", "Hammasini belgilaş"),
        "menu.view" => ("Кўриниш", "Kóriniş"),
        "menu.reload" => ("Қайта юклаш", "Qayta yuklaş"),
        "menu.back" => ("Орқага", "Orqaga"),
        "menu.forward" => ("Олдинга", "Oldinga"),
        "menu.zoom_in" => ("Катталаштириш", "Kattalaştiriş"),
        "menu.zoom_out" => ("Кичиклаштириш", "Kiçiklaştiriş"),
        "menu.zoom_reset" => ("Асл ўлчам", "Asl ólçam"),
        "menu.fullscreen" => ("Тўлиқ экран", "Tóliq ekran"),
        "menu.help" => ("Ёрдам", "Yordam"),
        "menu.help_center" => ("Ёрдам маркази", "Yordam markazi"),
        "menu.faq" => ("Кўп бериладиган саволлар", "Kóp beriladigan savollar"),
        "menu.support" => ("Мурожаат юбориш", "Murojaat yuboriş"),
        "menu.updates" => ("Янгиланишни текшириш", "Yangilanişni tekşiriş"),
        "about.title" => ("EduX ҳақида", "EduX haqida"),
        "about.app" => ("Илова версияси", "Ilova versiyasi"),
        "about.server" => ("Сервер", "Server"),
        "about.server_version" => ("Сервер версияси", "Server versiyasi"),
        "about.unknown" => ("номаълум", "nomaʼlum"),
        "update.title" => ("Янги версия мавжуд", "Yangi versiya mavjud"),
        "update.download" => ("Юклаб олиш", "Yuklab oliş"),
        "update.text" => ("EduX {new} чиқди, сизда {current}. Юклаб олиб ўрнатинг — созламалар сақланиб қолади.", "EduX {new} çiqdi, sizda {current}. Yuklab olib órnating — sozlamalar saqlanib qoladi."),
        "update.later" => ("Кейинроқ", "Keyinroq"),
        "update.close" => ("Ёпиш", "Yopiş"),
        "update.none" => ("Сизда энг сўнгги версия ўрнатилган.", "Sizda eng sónggi versiya órnatilgan."),
        "offline.text" => ("Интернет алоқаси йўқ. Саҳифа очиқ қолади — алоқа тикланиши билан иш давом этади.", "Internet aloqasi yóq. Sahifa oçiq qoladi — aloqa tiklanişi bilan iş davom etadi."),
        "save.title" => ("Файлни сақлаш", "Faylni saqlaş"),
        "save.done" => ("Файл сақланди", "Fayl saqlandi"),
        "save.failed" => ("Файлни сақлаб бўлмади", "Faylni saqlab bólmadi"),
        _ => return None,
    };
    Some(if lang == "oz" { oz } else { yl })
}

#[cfg(test)]
mod tests {
    use super::t;

    /// Menyu va dialog kalitlari uchala tilda to'ldirilgan (kalitning o'zi qaytmaydi), ruscha va inglizcha — o'zbekchadan farqli.
    #[test]
    fn menu_keys_translated() {
        let keys = [
            "menu.app", "menu.advanced", "menu.server", "menu.browser", "menu.about", "menu.quit", "menu.edit", "menu.view",
            "menu.reload", "menu.back", "menu.forward", "menu.zoom_in", "menu.zoom_out", "menu.zoom_reset", "menu.fullscreen",
            "menu.help", "menu.help_center", "menu.faq", "menu.support", "menu.updates", "offline.text", "update.text",
        ];
        for key in keys {
            for lang in ["uz", "ru", "en", "oz", "yl"] {
                assert_ne!(t(lang, key), key, "{lang}: {key}");
            }
            if key != "menu.app" {
                assert_ne!(t("ru", key), t("uz", key), "ru: {key}");
            }
        }
        assert_eq!(t("ru", "menu.help"), "Справка");
        assert_eq!(t("ru", "menu.help_center"), "Центр помощи");
        assert_eq!(t("ru", "menu.faq"), "Частые вопросы");
        assert_eq!(t("xx", "menu.support"), "Murojaat yuborish");
    }

    /// 1.5.0: kirill va yangi lotin — har kalit o'girilgan, kirill matn kirill harflarda.
    #[test]
    fn kirill_va_yangi_lotin() {
        assert_eq!(t("oz", "menu.help"), "Ёрдам");
        assert_eq!(t("oz", "menu.quit"), "Чиқиш");
        assert_eq!(t("yl", "menu.quit"), "Çiqiş");
        assert_eq!(t("yl", "menu.view"), "Kóriniş");
        assert_eq!(t("oz", "menu.about"), "EduX ҳақида");
        assert!(t("oz", "update.text").contains("{new}") && t("yl", "update.text").contains("{current}"));
        for key in ["menu.edit", "menu.reload", "offline.text", "save.failed", "update.later"] {
            assert_ne!(t("oz", key), t("uz", key), "oz: {key}");
            assert!(t("oz", key).chars().any(|c| ('\u{0400}'..='\u{04FF}').contains(&c)), "oz kirillda emas: {key}");
        }
    }
}
