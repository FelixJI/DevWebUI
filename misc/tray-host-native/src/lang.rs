//! The host's own UI strings, one table per locale.
//!
//! The host is generic across the kit's apps, so ITS strings (the menu, the balloons) live here
//! while each app's own labels (`menuOpenLabel`, the action item) stay in that app's tray config,
//! where they can be per-locale maps (see Config::LocalizedText). The two lookups meet in
//! `main.rs`, which asks this module for the ACTIVE locale — the runtime pointer's `locale` field
//! (written by the daemon from the web UI's language picker) when the daemon knows one, else the
//! Windows UI language, else English.
//!
//! Seven locales, matching the web UI's shipped set (web/src/i18n/locales). English is the base:
//! every field of every other table is a translation of EN's, and an unknown locale falls back to
//! it, so a missing table can never produce an empty menu item.

/// One locale's strings. Placeholders — `{APP}` (display name), `{HINT}` (a menu-item hint),
/// `{LOG}` (a log file name), `{URL}` — are substituted by [`template`].
pub struct Lang {
    pub code: &'static str,
    pub restart: &'static str,
    pub restart_normally: &'static str,
    pub restart_safe: &'static str,
    pub quit: &'static str,
    /// Dev-tree only; no accelerator letter outside English on purpose (the && mnemonic needs a
    /// Latin letter the label may not have).
    pub rebuild_restart: &'static str,
    /// Appended to the icon tooltip while the daemon runs in safe mode.
    pub safe_suffix: &'static str,
    pub tip_running: &'static str,
    pub tip_unclean: &'static str,
    pub not_running: &'static str,
    pub stopped_unexpectedly: &'static str,
    pub keeps_crashing: &'static str,
    pub hint_safe: &'static str,
    pub hint_plain: &'static str,
    pub restarted_not_ready: &'static str,
    pub build_failed: &'static str,
    pub already_starting: &'static str,
    pub already_serving: &'static str,
    pub could_not_start: &'static str,
}

pub const EN: Lang = Lang {
    code: "en",
    restart: "Restart",
    restart_normally: "Restart Normally",
    restart_safe: "Restart in Safe Mode",
    quit: "Quit",
    rebuild_restart: "Rebuild && Restart",
    safe_suffix: " (Safe Mode)",
    tip_running: "Running in the tray - right-click for options.",
    tip_unclean: "{APP} did not shut down cleanly last time. If it misbehaves, right-click > Restart in Safe Mode.",
    not_running: "{APP} isn't running.",
    stopped_unexpectedly: "{APP} stopped unexpectedly - restarting.",
    keeps_crashing: "{APP} keeps crashing - auto-restart paused. Use {HINT}.",
    hint_safe: "Restart or Restart in Safe Mode",
    hint_plain: "Restart to try again",
    restarted_not_ready: "Restarted, but {APP} isn't answering yet.",
    build_failed: "Build failed. See misc\\{LOG}.",
    already_starting: "{APP} is already starting in the tray. Wait a moment, then open it from the tray icon.",
    already_serving: "{APP} is already serving at {URL}, but the tray icon is not running. Stop that process, then run the shortcut again.",
    could_not_start: "{APP} could not start its background process.",
};

pub const ZH_CN: Lang = Lang {
    code: "zh-CN",
    restart: "重新启动",
    restart_normally: "正常重启",
    restart_safe: "以安全模式重启",
    quit: "退出",
    rebuild_restart: "重新构建并重启",
    safe_suffix: "（安全模式）",
    tip_running: "正在系统托盘中运行——右键查看选项。",
    tip_unclean: "{APP} 上次未正常退出。如表现异常，请右键选择“以安全模式重启”。",
    not_running: "{APP} 未在运行。",
    stopped_unexpectedly: "{APP} 意外停止——正在重启。",
    keeps_crashing: "{APP} 反复崩溃——自动重启已暂停。请使用{HINT}。",
    hint_safe: "“重新启动”或“以安全模式重启”",
    hint_plain: "“重新启动”再试一次",
    restarted_not_ready: "已重启，但 {APP} 尚未响应。",
    build_failed: "构建失败。参见 misc\\{LOG}。",
    already_starting: "{APP} 已正在托盘中启动。请稍候，然后从托盘图标打开。",
    already_serving: "{APP} 已在 {URL} 提供服务，但托盘图标未运行。请先停止该进程，再运行快捷方式。",
    could_not_start: "{APP} 无法启动其后台进程。",
};

pub const ZH_TW: Lang = Lang {
    code: "zh-TW",
    restart: "重新啟動",
    restart_normally: "正常重新啟動",
    restart_safe: "以安全模式重新啟動",
    quit: "結束",
    rebuild_restart: "重新建置並重新啟動",
    safe_suffix: "（安全模式）",
    tip_running: "正在系統匣中執行——右鍵查看選項。",
    tip_unclean: "{APP} 上次未正常結束。若運作異常，請右鍵選擇「以安全模式重新啟動」。",
    not_running: "{APP} 未在執行。",
    stopped_unexpectedly: "{APP} 意外停止——正在重新啟動。",
    keeps_crashing: "{APP} 持續當機——自動重新啟動已暫停。請使用{HINT}。",
    hint_safe: "「重新啟動」或「以安全模式重新啟動」",
    hint_plain: "「重新啟動」再試一次",
    restarted_not_ready: "已重新啟動，但 {APP} 尚未回應。",
    build_failed: "建置失敗。請參閱 misc\\{LOG}。",
    already_starting: "{APP} 已正在系統匣中啟動。請稍候，再從系統匣圖示開啟。",
    already_serving: "{APP} 已在 {URL} 提供服務，但系統匣圖示未執行。請先停止該程序，再執行捷徑。",
    could_not_start: "{APP} 無法啟動其背景程序。",
};

pub const JA: Lang = Lang {
    code: "ja",
    restart: "再起動",
    restart_normally: "通常モードで再起動",
    restart_safe: "セーフモードで再起動",
    quit: "終了",
    rebuild_restart: "再ビルドして再起動",
    safe_suffix: "（セーフモード）",
    tip_running: "トレイで実行中です。右クリックでオプションを表示します。",
    tip_unclean: "{APP} は前回正常に終了しませんでした。動作がおかしい場合は右クリックから「セーフモードで再起動」してください。",
    not_running: "{APP} は実行されていません。",
    stopped_unexpectedly: "{APP} が予期せず停止しました——再起動します。",
    keeps_crashing: "{APP} が繰り返しクラッシュしています——自動再起動を一時停止しました。{HINT}を実行してください。",
    hint_safe: "「再起動」または「セーフモードで再起動」",
    hint_plain: "「再起動」でお試しください",
    restarted_not_ready: "再起動しましたが、{APP} がまだ応答しません。",
    build_failed: "ビルドに失敗しました。misc\\{LOG} を参照してください。",
    already_starting: "{APP} はすでにトレイで起動中です。しばらく待ってからトレイアイコンから開いてください。",
    already_serving: "{APP} は {URL} で稼働中ですが、トレイアイコンが実行されていません。そのプロセスを停止してからショートカットを実行してください。",
    could_not_start: "{APP} はバックグラウンドプロセスを開始できませんでした。",
};

pub const ES: Lang = Lang {
    code: "es",
    restart: "Reiniciar",
    restart_normally: "Reiniciar normalmente",
    restart_safe: "Reiniciar en modo seguro",
    quit: "Salir",
    rebuild_restart: "Recompilar y reiniciar",
    safe_suffix: " (modo seguro)",
    tip_running: "Ejecutándose en la bandeja: clic derecho para opciones.",
    tip_unclean: "{APP} no se cerró correctamente la última vez. Si falla, usa clic derecho > Reiniciar en modo seguro.",
    not_running: "{APP} no se está ejecutando.",
    stopped_unexpectedly: "{APP} se detuvo inesperadamente: reiniciando.",
    keeps_crashing: "{APP} sigue fallando: el reinicio automático está en pausa. Usa {HINT}.",
    hint_safe: "Reiniciar o Reiniciar en modo seguro",
    hint_plain: "Reiniciar para volver a intentarlo",
    restarted_not_ready: "Reiniciado, pero {APP} aún no responde.",
    build_failed: "Falló la compilación. Consulta misc\\{LOG}.",
    already_starting: "{APP} ya se está iniciando en la bandeja. Espera un momento y ábrelo desde el icono de la bandeja.",
    already_serving: "{APP} ya está sirviendo en {URL}, pero el icono de la bandeja no se está ejecutando. Detén ese proceso y vuelve a ejecutar el acceso directo.",
    could_not_start: "{APP} no pudo iniciar su proceso en segundo plano.",
};

pub const DE: Lang = Lang {
    code: "de",
    restart: "Neu starten",
    restart_normally: "Normal neu starten",
    restart_safe: "Im abgesicherten Modus neu starten",
    quit: "Beenden",
    rebuild_restart: "Neu bauen und neu starten",
    safe_suffix: " (abgesicherter Modus)",
    tip_running: "Läuft im Infobereich – Rechtsklick für Optionen.",
    tip_unclean: "{APP} wurde beim letzten Mal nicht sauber beendet. Bei Fehlverhalten: Rechtsklick > Im abgesicherten Modus neu starten.",
    not_running: "{APP} läuft nicht.",
    stopped_unexpectedly: "{APP} wurde unerwartet beendet – wird neu gestartet.",
    keeps_crashing: "{APP} stürzt wiederholt ab – der automatische Neustart ist pausiert. Verwende {HINT}.",
    hint_safe: "„Neu starten“ oder „Im abgesicherten Modus neu starten“",
    hint_plain: "„Neu starten“, um es erneut zu versuchen",
    restarted_not_ready: "Neu gestartet, aber {APP} antwortet noch nicht.",
    build_failed: "Build fehlgeschlagen. Siehe misc\\{LOG}.",
    already_starting: "{APP} startet bereits im Infobereich. Kurz warten, dann über das Tray-Symbol öffnen.",
    already_serving: "{APP} bedient bereits {URL}, aber das Tray-Symbol läuft nicht. Beenden Sie diesen Prozess und führen Sie die Verknüpfung erneut aus.",
    could_not_start: "{APP} konnte seinen Hintergrundprozess nicht starten.",
};

pub const FR: Lang = Lang {
    code: "fr",
    restart: "Redémarrer",
    restart_normally: "Redémarrer normalement",
    restart_safe: "Redémarrer en mode sans échec",
    quit: "Quitter",
    rebuild_restart: "Recompiler et redémarrer",
    safe_suffix: " (mode sans échec)",
    tip_running: "En cours d'exécution dans la zone de notification – clic droit pour les options.",
    tip_unclean: "{APP} ne s'est pas fermé correctement la dernière fois. En cas de problème : clic droit > Redémarrer en mode sans échec.",
    not_running: "{APP} n'est pas en cours d'exécution.",
    stopped_unexpectedly: "{APP} s'est arrêté de manière inattendue — redémarrage.",
    keeps_crashing: "{APP} plante sans arrêt — le redémarrage automatique est en pause. Utilisez {HINT}.",
    hint_safe: "Redémarrer ou Redémarrer en mode sans échec",
    hint_plain: "Redémarrer pour réessayer",
    restarted_not_ready: "Redémarré, mais {APP} ne répond pas encore.",
    build_failed: "Échec de la compilation. Voir misc\\{LOG}.",
    already_starting: "{APP} démarre déjà dans la zone de notification. Patientez un instant, puis ouvrez-le depuis l'icône.",
    already_serving: "{APP} sert déjà à {URL}, mais l'icône de notification ne fonctionne pas. Arrêtez ce processus, puis relancez le raccourci.",
    could_not_start: "{APP} n'a pas pu démarrer son processus en arrière-plan.",
};

const LANGS: [&Lang; 7] = [&EN, &ZH_CN, &ZH_TW, &JA, &ES, &DE, &FR];

/// The table for a locale code: an exact match, then the primary subtag ("zh-HK" → zh-CN,
/// "de-AT" → de), then English. Case-insensitive throughout; BCP 47 casing is a convention,
/// not a promise, once a code has been through a settings file.
pub fn for_code(code: &str) -> &'static Lang {
    let key = code.trim().to_lowercase();
    if key.is_empty() {
        return &EN;
    }
    if let Some(l) = LANGS.iter().find(|l| l.code.to_lowercase() == key) {
        return l;
    }
    let primary = key.split(['-', '_']).next().unwrap_or("");
    LANGS.iter()
        .find(|l| l.code.to_lowercase().split(['-', '_']).next() == Some(primary))
        .copied()
        .unwrap_or(&EN)
}

/// The Windows UI locale, e.g. "zh-CN" ( GetUserDefaultLocaleName, Vista+). Empty when the OS
/// declines to say, which falls straight through to English.
pub fn os_locale() -> String {
    let mut buf = [0u16; 85]; // LOCALE_NAME_MAX_LENGTH
    let n = unsafe { crate::win::GetUserDefaultLocaleName(buf.as_mut_ptr(), buf.len() as i32) };
    if n <= 0 {
        return String::new();
    }
    String::from_utf16_lossy(&buf[..(n as usize).saturating_sub(1)])
}

/// Substitute `{PLACEHOLDER}`s. Each pair's first element is the FULL placeholder as it appears
/// in the text (`"{APP}"`, not `"APP"`) — matching how the config's own `{APP}` texts have always
/// been replaced, so a call site cannot mix the two conventions. Unknown placeholders are left
/// verbatim rather than stripped, so a typo in a table is visible in the balloon instead of
/// silently eating a word.
pub fn template(text: &str, vars: &[(&str, &str)]) -> String {
    let mut out = text.to_string();
    for (placeholder, value) in vars {
        out = out.replace(placeholder, value);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exact_then_primary_then_english() {
        assert_eq!(for_code("zh-CN").code, "zh-CN");
        assert_eq!(for_code("ZH-cn").code, "zh-CN"); // case is a convention
        // A region the tables don't carry rides its primary language.
        assert_eq!(for_code("zh-HK").code, "zh-CN");
        assert_eq!(for_code("de-AT").code, "de");
        assert_eq!(for_code("pt-BR").code, "en"); // unsupported falls back, never panics
        assert_eq!(for_code("").code, "en");
    }

    #[test]
    fn templates_substitute_known_placeholders_only() {
        let out = template("{APP} said {WHAT}?", &[("{APP}", "DevWebUI")]);
        assert_eq!(out, "DevWebUI said {WHAT}?");
    }

    /// Every non-English table is a full translation: a field left empty in one locale would
    /// render an empty menu item, which is worse than English. Compile-time types force the
    /// fields to EXIST; this forces them to be non-empty.
    #[test]
    fn every_locale_fills_every_field() {
        // Compare a few representative fields rather than all eighteen by name: the menu items
        // and the crash balloon are the ones a user cannot avoid seeing.
        for l in LANGS {
            assert!(!l.restart.is_empty(), "{}: restart", l.code);
            assert!(!l.quit.is_empty(), "{}: quit", l.code);
            assert!(!l.keeps_crashing.is_empty(), "{}: keeps_crashing", l.code);
            assert!(
                l.keeps_crashing.contains("{APP}") && l.keeps_crashing.contains("{HINT}"),
                "{}: keeps_crashing keeps its placeholders",
                l.code
            );
            assert!(l.build_failed.contains("{LOG}"), "{}: build_failed", l.code);
        }
    }
}
