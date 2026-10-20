// EduX ilovasining o'z sahifasi: ulanish, server manzilini tanlash va xato oynasi.
(function () {
    const STR = {
        uz: {
            connecting: 'Serverga ulanmoqda…',
            settings_title: 'Server manzili',
            settings_hint: 'Tashkilotingiz EduX manzilini kiriting. Manzil saqlanadi va keyingi safar avtomatik ochiladi.',
            server_label: 'Manzil (subdomen yoki domen)',
            server_examples: 'Masalan: markaz (→ markaz.edux.uz), app.edux.uz yoki maktab.uz',
            lang_label: 'Ilova tili',
            lang_auto: 'Tizim tili',
            cancel: 'Bekor qilish',
            continue: 'Davom etish',
            checking: 'Tekshirilmoqda…',
            change_server: "Server manzilini o'zgartirish",
            retry: 'Qayta urinish',
            err_unreachable_title: 'Serverga ulanib bo\'lmadi',
            err_unreachable: 'Internet aloqasini tekshiring yoki birozdan keyin qayta urinib ko\'ring.',
            err_not_edux_title: 'Bu manzilda EduX topilmadi',
            err_not_edux: 'Manzil to\'g\'riligini tekshiring yoki tashkilotingiz administratoridan so\'rang.',
            err_invalid: 'Manzil noto\'g\'ri kiritilgan.',
            err_empty: 'Manzilni kiriting.',
            version: 'Ilova versiyasi',
        },
        ru: {
            connecting: 'Подключение к серверу…',
            settings_title: 'Адрес сервера',
            settings_hint: 'Введите адрес EduX вашей организации. Адрес сохранится и в следующий раз откроется автоматически.',
            server_label: 'Адрес (поддомен или домен)',
            server_examples: 'Например: markaz (→ markaz.edux.uz), app.edux.uz или maktab.uz',
            lang_label: 'Язык приложения',
            lang_auto: 'Язык системы',
            cancel: 'Отмена',
            continue: 'Продолжить',
            checking: 'Проверка…',
            change_server: 'Изменить адрес сервера',
            retry: 'Повторить',
            err_unreachable_title: 'Не удалось подключиться к серверу',
            err_unreachable: 'Проверьте подключение к интернету или повторите попытку позже.',
            err_not_edux_title: 'По этому адресу EduX не найден',
            err_not_edux: 'Проверьте адрес или уточните его у администратора организации.',
            err_invalid: 'Адрес введён неверно.',
            err_empty: 'Введите адрес.',
            version: 'Версия приложения',
        },
        en: {
            connecting: 'Connecting to the server…',
            settings_title: 'Server address',
            settings_hint: "Enter your organization's EduX address. It will be saved and opened automatically next time.",
            server_label: 'Address (subdomain or domain)',
            server_examples: 'For example: markaz (→ markaz.edux.uz), app.edux.uz or maktab.uz',
            lang_label: 'App language',
            lang_auto: 'System language',
            cancel: 'Cancel',
            continue: 'Continue',
            checking: 'Checking…',
            change_server: 'Change server address',
            retry: 'Retry',
            err_unreachable_title: 'Could not connect to the server',
            err_unreachable: 'Check your internet connection or try again a little later.',
            err_not_edux_title: 'EduX was not found at this address',
            err_not_edux: 'Check the address or ask your organization administrator.',
            err_invalid: 'The address is not valid.',
            err_empty: 'Enter the address.',
            version: 'App version',
        },
    };

    const $ = (id) => document.getElementById(id);
    const invoke = (cmd, args) => window.__TAURI_INTERNALS__.invoke(cmd, args);
    let settings = null;
    let lang = 'uz';

    const tr = (key) => (STR[lang] && STR[lang][key]) || STR.uz[key] || key;

    function applyTexts() {
        document.documentElement.lang = lang;
        document.querySelectorAll('[data-t]').forEach((el) => { el.textContent = tr(el.dataset.t); });
        $('foot').textContent = settings ? `${tr('version')}: ${settings.version}` : '';
    }

    function show(view) {
        ['boot', 'settings', 'error'].forEach((v) => { $('view-' + v).hidden = v !== view; });
    }

    function hostOf(url) {
        try { return new URL(url).host; } catch (e) { return url || ''; }
    }

    function showError(code) {
        const key = code === 'not_edux' ? 'not_edux' : 'unreachable';
        $('error-title').textContent = tr(`err_${key}_title`);
        $('error-text').textContent = tr(`err_${key}`);
        $('error-server').textContent = settings && settings.server ? hostOf(settings.server) : '';
        show('error');
        $('retry').focus();
    }

    async function boot() {
        show('boot');
        $('boot-server').textContent = settings.server ? hostOf(settings.server) : '';
        try {
            const res = await invoke('boot');
            if (res && res.setup) openSettings(false);
            // aks holda oyna serverga o'tadi
        } catch (e) {
            if (String(e) === 'setup') openSettings(false);
            else showError(String(e));
        }
    }

    function openSettings(cancellable) {
        $('server').value = settings.server || settings.default_server || '';
        $('lang').value = settings.lang || '';
        $('settings-error').hidden = true;
        $('cancel').hidden = !(cancellable && settings.server);
        show('settings');
        $('server').focus();
        $('server').select();
    }

    $('lang').addEventListener('change', () => {
        lang = $('lang').value || settings.system_lang || 'uz';
        applyTexts();
    });

    $('settings-form').addEventListener('submit', async (e) => {
        e.preventDefault();
        const input = $('server').value.trim();
        const err = $('settings-error');
        err.hidden = true;
        if (!input) {
            err.textContent = tr('err_empty');
            err.hidden = false;
            return;
        }
        const button = $('save');
        button.disabled = true;
        button.firstElementChild.textContent = tr('checking');
        try {
            const res = await invoke('check_server', { input });
            await invoke('save_settings', { server: res.url, lang: $('lang').value || null });
            settings = await invoke('get_settings');
            boot();
        } catch (code) {
            const c = String(code);
            err.textContent = c === 'invalid' ? tr('err_invalid') : c === 'not_edux' ? `${tr('err_not_edux_title')}. ${tr('err_not_edux')}` : `${tr('err_unreachable_title')}. ${tr('err_unreachable')}`;
            err.hidden = false;
        } finally {
            button.disabled = false;
            button.firstElementChild.textContent = tr('continue');
        }
    });

    $('cancel').addEventListener('click', () => boot());
    $('retry').addEventListener('click', () => boot());
    $('change').addEventListener('click', () => openSettings(true));

    async function start() {
        settings = await invoke('get_settings');
        lang = settings.effective_lang || 'uz';
        applyTexts();
        const hash = location.hash.replace('#', '');
        if (hash === 'settings') openSettings(true);
        else boot();
    }

    window.addEventListener('hashchange', start);
    start().catch((e) => {
        console.error(e);
        showError('unreachable');
    });
})();
