// EduX ilovasining o'z sahifasi: yuklanish, «Server bilan aloqa yo'q» va server manzili (kengaytirilgan).
// 1.4.0: keng oynada chapda lojuvard panel (veb 1.41 kirish sahifasi) — matnlari ham shu lug'atda.
(function () {
    const STR = {
        uz: {
            panel_title: 'Testlar, to\'lovlar, davomat va jurnal — bitta oynada.',
            m_debt: 'o\'quvchining qarzi bor',
            m_debt_hint: 'Ota-onaga eslatma ketdi',
            m_tests: 'test natijasi ota-onaga yuborildi',
            m_lesson: 'ING-5, davomat bir bosishda',
            p_orgs: 'O\'quv markaz, maktab va OTM uchun',
            p_same: 'Telefon va kompyuterda bir xil ishlaydi',
            p_parents: 'Ota-onalar Telegram yoki SMS orqali xabardor',
            boot_title: 'EduX ochilmoqda',
            boot_text: 'Server bilan bog\'lanilmoqda:',
            boot_slow: 'Internet sekin bo\'lsa, bu biroz vaqt olishi mumkin.',
            advanced: 'Kengaytirilgan',
            settings_title: 'Server manzili',
            settings_hint: 'Faqat EduX o\'z serveriga o\'rnatilgan tashkilotlar uchun. Odatda bu yerda hech narsani o\'zgartirish shart emas.',
            server_label: 'Server manzili',
            server_examples: 'Masalan: markaz (→ markaz.{domain}), maktab.uz yoki 192.168.1.10:8000',
            lang_label: 'Ilova tili',
            lang_auto: 'Kompyuter tili',
            default_note: 'Asosiy server: {host}',
            reset_default: 'Asosiy serverga qaytish',
            cancel: 'Bekor qilish',
            save: 'Saqlash va ulanish',
            checking: 'Tekshirilmoqda…',
            change_server: 'Server manzilini o\'zgartirish (kengaytirilgan)',
            retry: 'Qayta urinish',
            st_internet: 'Internet',
            st_server: 'Server',
            st_connected: 'Ulangan',
            st_disconnected: 'Ulanmagan',
            st_no_answer: 'Javob bermadi',
            st_not_edux: 'EduX topilmadi',
            st_unknown: 'Tekshirib bo\'lmaydi',
            err_offline_title: 'Internet aloqasi yo\'q',
            err_offline: 'Kompyuter internetga ulanmagan. Wi-Fi yoki kabelni tekshiring — aloqa tiklanishi bilan EduX o\'zi ochiladi.',
            err_unreachable_title: 'Server bilan aloqa yo\'q',
            err_unreachable: 'EduX serveri hozir javob bermayapti. Internetni tekshiring yoki birozdan keyin qayta urinib ko\'ring.',
            err_not_edux_title: 'Bu manzilda EduX topilmadi',
            err_not_edux: 'Server manzili noto\'g\'ri bo\'lishi mumkin. Tashkilotingiz administratoridan to\'g\'ri manzilni so\'rang.',
            err_invalid: 'Manzil noto\'g\'ri yozilgan. Masalan: maktab.uz',
            err_empty: 'Server manzilini kiriting.',
            auto_in: '{n} soniyadan keyin o\'zi qayta urinadi',
            auto_online: 'Internet qaytishi bilan o\'zi qayta ulanadi',
            tech_details: 'Texnik ma\'lumot (yordam uchun)',
            version: 'EduX {v}, kompyuter ilovasi',
            help_title: 'Yordam:',
            help_problems: 'Muammolar va yechimlar',
            help_desktop: 'Kompyuter ilovasi',
        },
        ru: {
            panel_title: 'Тесты, оплаты, посещаемость и журнал — в одном окне.',
            m_debt: 'учеников с долгом',
            m_debt_hint: 'Родителям отправлено напоминание',
            m_tests: 'результат теста отправлен родителю',
            m_lesson: 'ING-5, посещаемость в одно нажатие',
            p_orgs: 'Для учебных центров, школ и вузов',
            p_same: 'Одинаково работает на телефоне и компьютере',
            p_parents: 'Родители узнают обо всём через Telegram или SMS',
            boot_title: 'EduX открывается',
            boot_text: 'Подключение к серверу:',
            boot_slow: 'При медленном интернете это может занять немного времени.',
            advanced: 'Дополнительно',
            settings_title: 'Адрес сервера',
            settings_hint: 'Только для организаций, установивших EduX на свой сервер. Обычно здесь ничего менять не нужно.',
            server_label: 'Адрес сервера',
            server_examples: 'Например: markaz (→ markaz.{domain}), maktab.uz или 192.168.1.10:8000',
            lang_label: 'Язык приложения',
            lang_auto: 'Язык компьютера',
            default_note: 'Основной сервер: {host}',
            reset_default: 'Вернуть основной сервер',
            cancel: 'Отмена',
            save: 'Сохранить и подключиться',
            checking: 'Проверка…',
            change_server: 'Изменить адрес сервера (дополнительно)',
            retry: 'Повторить',
            st_internet: 'Интернет',
            st_server: 'Сервер',
            st_connected: 'Подключён',
            st_disconnected: 'Нет подключения',
            st_no_answer: 'Не отвечает',
            st_not_edux: 'EduX не найден',
            st_unknown: 'Не проверить',
            err_offline_title: 'Нет подключения к интернету',
            err_offline: 'Компьютер не подключён к интернету. Проверьте Wi-Fi или кабель — EduX откроется сам, как только связь появится.',
            err_unreachable_title: 'Нет связи с сервером',
            err_unreachable: 'Сервер EduX сейчас не отвечает. Проверьте интернет или повторите попытку чуть позже.',
            err_not_edux_title: 'По этому адресу EduX не найден',
            err_not_edux: 'Возможно, адрес сервера неверный. Уточните правильный адрес у администратора организации.',
            err_invalid: 'Адрес введён неверно. Например: maktab.uz',
            err_empty: 'Введите адрес сервера.',
            auto_in: 'Повторная попытка через {n} с',
            auto_online: 'Подключится сам, как только появится интернет',
            tech_details: 'Технические сведения (для поддержки)',
            version: 'EduX {v}, приложение для компьютера',
            help_title: 'Помощь:',
            help_problems: 'Проблемы и решения',
            help_desktop: 'Приложение для компьютера',
        },
        en: {
            panel_title: 'Tests, payments, attendance and journal — in one window.',
            m_debt: 'students owe money',
            m_debt_hint: 'Parents have been reminded',
            m_tests: 'test result sent to the parent',
            m_lesson: 'ING-5, attendance in one tap',
            p_orgs: 'For learning centers, schools and universities',
            p_same: 'Works the same on phone and computer',
            p_parents: 'Parents stay informed via Telegram or SMS',
            boot_title: 'Opening EduX',
            boot_text: 'Connecting to the server:',
            boot_slow: 'On a slow connection this may take a little while.',
            advanced: 'Advanced',
            settings_title: 'Server address',
            settings_hint: 'Only for organizations that host EduX on their own server. Usually there is nothing to change here.',
            server_label: 'Server address',
            server_examples: 'For example: markaz (→ markaz.{domain}), maktab.uz or 192.168.1.10:8000',
            lang_label: 'App language',
            lang_auto: 'Computer language',
            default_note: 'Main server: {host}',
            reset_default: 'Back to the main server',
            cancel: 'Cancel',
            save: 'Save and connect',
            checking: 'Checking…',
            change_server: 'Change server address (advanced)',
            retry: 'Try again',
            st_internet: 'Internet',
            st_server: 'Server',
            st_connected: 'Connected',
            st_disconnected: 'Not connected',
            st_no_answer: 'No response',
            st_not_edux: 'EduX not found',
            st_unknown: 'Cannot check',
            err_offline_title: 'No internet connection',
            err_offline: 'This computer is not connected to the internet. Check Wi-Fi or the cable — EduX will open by itself once the connection is back.',
            err_unreachable_title: 'Cannot reach the server',
            err_unreachable: 'The EduX server is not responding right now. Check the internet or try again a little later.',
            err_not_edux_title: 'EduX was not found at this address',
            err_not_edux: 'The server address may be wrong. Ask your organization administrator for the correct address.',
            err_invalid: 'The address is not valid. For example: maktab.uz',
            err_empty: 'Enter the server address.',
            auto_in: 'Trying again in {n} s',
            auto_online: 'Will reconnect by itself once the internet is back',
            tech_details: 'Technical details (for support)',
            version: 'EduX {v}, desktop app',
            help_title: 'Help:',
            help_problems: 'Problems and solutions',
            help_desktop: 'Desktop app',
        },
    };

    const ICONS = {
        // Wi-Fi o'chiq
        offline: '<path d="M2 2l20 20"/><path d="M8.5 16.5a5 5 0 0 1 7 0"/><path d="M5 12.86a10 10 0 0 1 5.17-2.69"/><path d="M19 12.86a10 10 0 0 0-2.01-1.48"/><path d="M2 8.82a15 15 0 0 1 4.17-2.65"/><path d="M10.66 5c4.01-.36 8.14.9 11.34 3.76"/><path d="M12 20h.01"/>',
        // Server javob bermadi
        unreachable: '<rect x="3" y="4" width="18" height="7" rx="2"/><rect x="3" y="13" width="18" height="7" rx="2"/><path d="M7 7.5h.01"/><path d="M7 16.5h.01"/><path d="M15 15l4 4"/><path d="M19 15l-4 4"/>',
        // Manzil noto'g'ri
        not_edux: '<circle cx="11" cy="11" r="7"/><path d="m21 21-4.3-4.3"/><path d="M11 8v3"/><path d="M11 14h.01"/>',
    };

    const $ = (id) => document.getElementById(id);
    const invoke = (cmd, args) => window.__TAURI_INTERNALS__.invoke(cmd, args);
    let settings = null;
    let lang = 'uz';
    let lastCode = null;
    let retryTimer = null;
    let retryLeft = 0;
    let retryStep = 0;
    let slowTimer = null;
    let booting = false;

    const tr = (key, vars) => {
        let s = (STR[lang] && STR[lang][key]) || STR.uz[key] || key;
        if (vars) Object.keys(vars).forEach((k) => { s = s.split(`{${k}}`).join(vars[k]); });
        return s;
    };

    function hostOf(url) {
        try { return new URL(url).host; } catch (e) { return url || ''; }
    }

    function siteDomain(host) {
        const h = String(host || '').split(':')[0];
        if (!h || /^[\d.]+$/.test(h) || h === 'localhost') return h || 'edux.uz';
        const parts = h.split('.');
        return parts.length <= 2 ? h : parts.slice(-2).join('.');
    }

    function current() {
        const visible = ['boot', 'settings', 'error'].find((v) => !$('view-' + v).hidden);
        return visible || 'boot';
    }

    function applyTexts() {
        document.documentElement.lang = lang;
        document.querySelectorAll('[data-t]').forEach((el) => { el.textContent = tr(el.dataset.t); });
        $('foot').textContent = settings ? tr('version', { v: settings.version }) : '';
        if (settings) {
            const defHost = hostOf(settings.default_server);
            $('server-hint').textContent = tr('server_examples', { domain: siteDomain(defHost) });
            $('default-note').textContent = tr('default_note', { host: defHost });
        }
        if (current() === 'error' && lastCode) renderError(lastCode);
    }

    function show(view) {
        ['boot', 'settings', 'error'].forEach((v) => { $('view-' + v).hidden = v !== view; });
        if (view !== 'error') stopRetry();
        if (view !== 'boot') clearTimeout(slowTimer);
    }

    // ---------------------------------------------------------------- aloqa yo'q

    function setStatus(id, state, text) {
        $(id).dataset.state = state;
        $(id).textContent = text;
        $(id + '-dot').dataset.state = state;
    }

    function renderError(code) {
        const offline = navigator.onLine === false && code !== 'not_edux';
        const kind = offline ? 'offline' : code === 'not_edux' ? 'not_edux' : 'unreachable';
        const card = $('view-error');
        card.dataset.tone = kind === 'offline' ? 'warning' : 'danger';
        $('error-icon').innerHTML = ICONS[kind];
        $('error-title').textContent = tr(`err_${kind}_title`);
        $('error-text').textContent = tr(`err_${kind}`);
        $('error-server').textContent = settings && settings.server ? hostOf(settings.server) : '';
        if (offline) {
            setStatus('st-net', 'bad', tr('st_disconnected'));
            setStatus('st-srv', 'idle', tr('st_unknown'));
        } else {
            setStatus('st-net', 'ok', tr('st_connected'));
            setStatus('st-srv', 'bad', tr(kind === 'not_edux' ? 'st_not_edux' : 'st_no_answer'));
        }
        const tech = settings && settings.last_error;
        $('tech').hidden = !tech;
        $('tech-text').textContent = tech || '';
        renderRetry();
    }

    function renderRetry() {
        const el = $('auto-retry');
        if (lastCode === 'not_edux') { el.textContent = ''; return; }
        if (navigator.onLine === false) el.textContent = tr('auto_online');
        else el.textContent = retryLeft > 0 ? tr('auto_in', { n: retryLeft }) : '';
    }

    function stopRetry() {
        clearInterval(retryTimer);
        retryTimer = null;
        retryLeft = 0;
    }

    /** Server javob bermasa — o'zi qayta urinadi: 15, 30, 60 soniya (keyin har 60). Noto'g'ri manzilda — yo'q. */
    function scheduleRetry() {
        stopRetry();
        if (lastCode === 'not_edux') return;
        retryLeft = [15, 30, 60][Math.min(retryStep, 2)];
        retryStep += 1;
        retryTimer = setInterval(() => {
            retryLeft -= 1;
            if (retryLeft <= 0) {
                stopRetry();
                boot();
                return;
            }
            renderRetry();
        }, 1000);
    }

    async function showError(code) {
        lastCode = code === 'not_edux' ? 'not_edux' : 'unreachable';
        try { settings = await invoke('get_settings'); } catch (e) { /* oxirgi ma'lumot qoladi */ }
        show('error');
        scheduleRetry();
        renderError(lastCode);
        $('retry').focus();
    }

    // ---------------------------------------------------------------- ulanish

    async function boot() {
        if (booting) return;
        booting = true;
        show('boot');
        $('boot-server').textContent = settings.server ? hostOf(settings.server) : '';
        $('boot-slow').hidden = true;
        clearTimeout(slowTimer);
        slowTimer = setTimeout(() => { $('boot-slow').hidden = false; }, 5000);
        try {
            const res = await invoke('boot');
            if (res && res.setup) openSettings();
            else retryStep = 0; // oyna serverga o'tadi
        } catch (e) {
            if (String(e) === 'setup') openSettings();
            else await showError(String(e));
        } finally {
            booting = false;
        }
    }

    window.addEventListener('online', () => {
        if (current() === 'error') boot();
    });
    window.addEventListener('offline', () => {
        if (current() === 'error') renderError(lastCode);
    });

    // ---------------------------------------------------------------- server manzili

    function fieldError(text) {
        const err = $('settings-error');
        err.textContent = text || '';
        err.hidden = !text;
        $('server').setAttribute('aria-invalid', text ? 'true' : 'false');
    }

    function openSettings() {
        // https:// yashiriladi (kiritishda o'zi qo'shiladi), http:// — ko'rinadi.
        $('server').value = settings.custom ? String(settings.server || '').replace(/^https:\/\//, '') : '';
        $('server').placeholder = hostOf(settings.default_server);
        $('lang').value = settings.lang || '';
        fieldError('');
        $('reset').hidden = !settings.custom;
        show('settings');
        $('server').focus();
        $('server').select();
    }

    $('lang').addEventListener('change', () => {
        lang = $('lang').value || settings.system_lang || 'uz';
        applyTexts();
    });

    $('server').addEventListener('input', () => fieldError(''));

    $('settings-form').addEventListener('submit', async (e) => {
        e.preventDefault();
        const typed = $('server').value.trim();
        // Bo'sh qoldirilsa va hozir asosiy server bo'lsa — faqat til saqlanadi.
        const input = typed || (settings.custom ? '' : settings.default_server);
        if (!input) {
            fieldError(tr('err_empty'));
            return;
        }
        const button = $('save');
        button.disabled = true;
        button.firstElementChild.textContent = tr('checking');
        try {
            const res = await invoke('check_server', { input });
            await invoke('save_settings', { server: res.url, lang: $('lang').value || null, reset: false });
            settings = await invoke('get_settings');
            retryStep = 0;
            boot();
        } catch (code) {
            const c = String(code);
            fieldError(c === 'invalid' ? tr('err_invalid')
                : c === 'not_edux' ? `${tr('err_not_edux_title')}. ${tr('err_not_edux')}`
                    : `${tr('err_unreachable_title')}. ${tr('err_unreachable')}`);
        } finally {
            button.disabled = false;
            button.firstElementChild.textContent = tr('save');
        }
    });

    $('cancel').addEventListener('click', () => boot());
    $('reset').addEventListener('click', async () => {
        await invoke('save_settings', { server: null, lang: $('lang').value || null, reset: true });
        settings = await invoke('get_settings');
        retryStep = 0;
        boot();
    });
    $('retry').addEventListener('click', () => { retryStep = 0; boot(); });
    $('change').addEventListener('click', () => openSettings());
    // Ochiq yordam markazi maqolasi brauzerda: <server>/yordam/<slug>?lang=..
    document.querySelectorAll('[data-help]').forEach((el) => {
        el.addEventListener('click', () => {
            const server = String((settings && (settings.server || settings.default_server)) || '').replace(/\/+$/, '');
            if (!server) return;
            invoke('open_external', { url: `${server}/yordam/${el.dataset.help}?lang=${lang}` }).catch(() => {});
        });
    });

    async function start() {
        settings = await invoke('get_settings');
        lang = settings.effective_lang || 'uz';
        applyTexts();
        const hash = location.hash.replace('#', '');
        if (hash === 'settings') openSettings();
        else boot();
    }

    window.addEventListener('hashchange', start);
    start().catch((e) => {
        console.error(e);
        showError('unreachable');
    });
})();
