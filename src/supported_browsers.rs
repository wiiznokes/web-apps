use crate::browser::{BinaryLocation, Browser, BrowserType};

#[allow(dead_code)]
pub fn native_browsers() -> Vec<Browser> {
    vec![
        Browser::new(
            BrowserType::Firefox,
            BinaryLocation::System,
            "Firefox",
            "firefox",
            ".local/share/quick-webapps/firefox",
        ),
        Browser::new(
            BrowserType::Firefox,
            BinaryLocation::System,
            "Firefox Developer Edition",
            "firefox-developer-edition",
            ".local/share/quick-webapps/firefox",
        ),
        Browser::new(
            BrowserType::Firefox,
            BinaryLocation::System,
            "Firefox Nightly",
            "firefox-nightly",
            ".local/share/quick-webapps/firefox",
        ),
        Browser::new(
            BrowserType::Firefox,
            BinaryLocation::System,
            "Firefox ESR",
            "firefox-esr",
            ".local/share/quick-webapps/firefox",
        ),
        Browser::new(
            BrowserType::Chromium,
            BinaryLocation::System,
            "Brave Browser",
            "brave-browser",
            ".local/share/quick-webapps/chromium",
        ),
        Browser::new(
            BrowserType::Chromium,
            BinaryLocation::System,
            "Brave (bin)",
            "brave-bin",
            ".local/share/quick-webapps/chromium",
        ),
        Browser::new(
            BrowserType::Chromium,
            BinaryLocation::System,
            "Chrome",
            "google-chrome-stable",
            ".local/share/quick-webapps/chromium",
        ),
        Browser::new(
            BrowserType::Chromium,
            BinaryLocation::System,
            "Chrome Beta",
            "google-chrome-beta",
            ".local/share/quick-webapps/chromium",
        ),
        Browser::new(
            BrowserType::Chromium,
            BinaryLocation::System,
            "Chromium",
            "chromium",
            ".local/share/quick-webapps/chromium",
        ),
        Browser::new(
            BrowserType::Chromium,
            BinaryLocation::System,
            "Chromium Browser",
            "chromium-browser",
            ".local/share/quick-webapps/chromium",
        ),
        Browser::new(
            BrowserType::Chromium,
            BinaryLocation::System,
            "Chromium (bin)",
            "chromium-bin",
            ".local/share/quick-webapps/chromium",
        ),
        Browser::new(
            BrowserType::Chromium,
            BinaryLocation::System,
            "Cromite",
            "cromite",
            ".local/share/quick-webapps/chromium",
        ),
        Browser::new(
            BrowserType::Chromium,
            BinaryLocation::System,
            "Thorium",
            "thorium-browser",
            ".local/share/quick-webapps/chromium",
        ),
        Browser::new(
            BrowserType::Firefox,
            BinaryLocation::System,
            "Librewolf",
            "librewolf",
            ".local/share/quick-webapps/firefox",
        ),
        Browser::new(
            BrowserType::Firefox,
            BinaryLocation::System,
            "Waterfox",
            "waterfox",
            ".local/share/quick-webapps/firefox",
        ),
        Browser::new(
            BrowserType::Firefox,
            BinaryLocation::System,
            "Waterfox (current)",
            "waterfox-current",
            ".local/share/quick-webapps/firefox",
        ),
        Browser::new(
            BrowserType::Firefox,
            BinaryLocation::System,
            "Waterfox (classic)",
            "waterfox-classic",
            ".local/share/quick-webapps/firefox",
        ),
        Browser::new(
            BrowserType::Firefox,
            BinaryLocation::System,
            "Waterfox 3rd Generation",
            "waterfox-g3",
            ".local/share/quick-webapps/firefox",
        ),
        Browser::new(
            BrowserType::Firefox,
            BinaryLocation::System,
            "Waterfox 4rd Generation",
            "waterfox-g4",
            ".local/share/quick-webapps/firefox",
        ),
        Browser::new(
            BrowserType::Zen,
            BinaryLocation::System,
            "Zen Browser",
            "zen-browser",
            ".local/share/quick-webapps/firefox",
        ),
        Browser::new(
            BrowserType::Chromium,
            BinaryLocation::System,
            "Vivaldi",
            "vivaldi-stable",
            ".local/share/quick-webapps/chromium",
        ),
        Browser::new(
            BrowserType::Chromium,
            BinaryLocation::System,
            "Vivaldi Snapshot",
            "vivaldi-snapshot",
            ".local/share/quick-webapps/chromium",
        ),
        Browser::new(
            BrowserType::Chromium,
            BinaryLocation::System,
            "Microsoft Edge",
            "microsoft-edge-stable",
            ".local/share/quick-webapps/chromium",
        ),
        Browser::new(
            BrowserType::Chromium,
            BinaryLocation::System,
            "Microsoft Edge Beta",
            "microsoft-edge-beta",
            ".local/share/quick-webapps/chromium",
        ),
        Browser::new(
            BrowserType::Chromium,
            BinaryLocation::System,
            "Microsoft Edge Dev",
            "microsoft-edge-dev",
            ".local/share/quick-webapps/chromium",
        ),
        Browser::new(
            BrowserType::Chromium,
            BinaryLocation::System,
            "FlashPeak Slimjet",
            "flashpeak-slimjet",
            ".local/share/quick-webapps/chromium",
        ),
        Browser::new(
            BrowserType::Chromium,
            BinaryLocation::System,
            "Yandex",
            "yandex-browser",
            ".local/share/quick-webapps/chromium",
        ),
        Browser::new(
            BrowserType::Chromium,
            BinaryLocation::System,
            "Naver Whale",
            "naver-whale-stable",
            ".local/share/quick-webapps/chromium",
        ),
        Browser::new(
            BrowserType::Chromium,
            BinaryLocation::System,
            "Brave",
            "brave",
            ".local/share/quick-webapps/chromium",
        ),
        Browser::new(
            BrowserType::Falkon,
            BinaryLocation::System,
            "Falkon",
            "falkon",
            ".local/share/quick-webapps/falkon",
        ),
    ]
}

pub fn flatpak_browsers() -> Vec<Browser> {
    vec![
        Browser::new(
            BrowserType::FirefoxFlatpak,
            BinaryLocation::FlatpakLocal,
            "Firefox",
            "org.mozilla.firefox",
            ".var/app/org.mozilla.firefox/data/profiles",
        ),
        Browser::new(
            BrowserType::ChromiumFlatpak,
            BinaryLocation::FlatpakLocal,
            "Chrome",
            "com.google.Chrome",
            ".var/app/com.google.Chrome/data/profiles",
        ),
        Browser::new(
            BrowserType::FirefoxFlatpak,
            BinaryLocation::FlatpakLocal,
            "Librewolf",
            "io.gitlab.librewolf-community",
            ".var/app/io.gitlab.librewolf-community/data/profiles",
        ),
        Browser::new(
            BrowserType::FirefoxFlatpak,
            BinaryLocation::FlatpakLocal,
            "Waterfox",
            "net.waterfox.waterfox",
            ".var/app/net.waterfox.waterfox/data/profiles",
        ),
        Browser::new(
            BrowserType::ChromiumFlatpak,
            BinaryLocation::FlatpakLocal,
            "Vivaldi",
            "com.vivaldi.Vivaldi",
            ".var/app/com.vivaldi.Vivaldi/data/profiles",
        ),
        Browser::new(
            BrowserType::ChromiumFlatpak,
            BinaryLocation::FlatpakLocal,
            "Ungoogled Chromium",
            "io.github.ungoogled_software.ungoogled_chromium",
            ".var/app/io.github.ungoogled_software.ungoogled_chromium/data/profiles",
        ),
        Browser::new(
            BrowserType::ChromiumFlatpak,
            BinaryLocation::FlatpakLocal,
            "Chromium",
            "org.chromium.Chromium",
            ".var/app/org.chromium.Chromium/data/profiles",
        ),
        Browser::new(
            BrowserType::ChromiumFlatpak,
            BinaryLocation::FlatpakLocal,
            "Microsoft Edge",
            "com.microsoft.Edge",
            ".var/app/com.microsoft.Edge/data/profiles",
        ),
        Browser::new(
            BrowserType::ChromiumFlatpak,
            BinaryLocation::FlatpakLocal,
            "Brave",
            "com.brave.Browser",
            ".var/app/com.brave.Browser/data/profiles",
        ),
        Browser::new(
            BrowserType::FalkonFlatpak,
            BinaryLocation::FlatpakLocal,
            "Falkon",
            "org.kde.falkon",
            ".var/app/org.kde.falkon/data/profiles",
        ),
        Browser::new(
            BrowserType::ChromiumFlatpak,
            BinaryLocation::FlatpakLocal,
            "Yandex",
            "ru.yandex.Browser",
            ".var/app/ru.yandex.Browser/data/profiles",
        ),
        Browser::new(
            BrowserType::FirefoxFlatpak,
            BinaryLocation::FlatpakLocal,
            "Floorp",
            "one.ablaze.floorp",
            ".var/app/one.ablaze.floorp/data/profiles",
        ),
        Browser::new(
            BrowserType::ZenFlatpak,
            BinaryLocation::FlatpakLocal,
            "Zen Browser",
            "io.github.zen_browser.zen",
            ".var/app/io.github.zen_browser.zen/data/profiles",
        ),
    ]
}

pub fn nix_browsers() -> Vec<Browser> {
    vec![
        Browser::new(
            BrowserType::Firefox,
            BinaryLocation::Nix,
            "Firefox",
            "firefox",
            ".local/share/quick-webapps/firefox",
        ),
        Browser::new(
            BrowserType::Firefox,
            BinaryLocation::Nix,
            "Floorp",
            "floorp",
            ".local/share/quick-webapps/firefox",
        ),
        Browser::new(
            BrowserType::Chromium,
            BinaryLocation::Nix,
            "Brave",
            "brave",
            ".local/share/quick-webapps/chromium",
        ),
        Browser::new(
            BrowserType::Chromium,
            BinaryLocation::Nix,
            "Chrome",
            "google-chrome-stable",
            ".local/share/quick-webapps/chromium",
        ),
        Browser::new(
            BrowserType::Chromium,
            BinaryLocation::Nix,
            "Chromium",
            "chromium",
            ".local/share/quick-webapps/chromium",
        ),
        Browser::new(
            BrowserType::Firefox,
            BinaryLocation::Nix,
            "Librewolf",
            "librewolf",
            ".local/share/quick-webapps/firefox",
        ),
        Browser::new(
            BrowserType::Chromium,
            BinaryLocation::Nix,
            "Vivaldi",
            "vivaldi-stable",
            ".local/share/quick-webapps/chromium",
        ),
        Browser::new(
            BrowserType::Chromium,
            BinaryLocation::Nix,
            "Microsoft Edge",
            "microsoft-edge-stable",
            ".local/share/quick-webapps/chromium",
        ),
        Browser::new(
            BrowserType::Chromium,
            BinaryLocation::Nix,
            "Microsoft Edge Beta",
            "microsoft-edge-beta",
            ".local/share/quick-webapps/chromium",
        ),
        Browser::new(
            BrowserType::Chromium,
            BinaryLocation::Nix,
            "Microsoft Edge Dev",
            "microsoft-edge-dev",
            ".local/share/quick-webapps/chromium",
        ),
        Browser::new(
            BrowserType::Chromium,
            BinaryLocation::Nix,
            "Yandex",
            "yandex-browser",
            ".local/share/quick-webapps/chromium",
        ),
        Browser::new(
            BrowserType::Falkon,
            BinaryLocation::Nix,
            "Falkon",
            "falkon",
            ".local/share/quick-webapps/falkon",
        ),
    ]
}

pub fn snap_browsers() -> Vec<Browser> {
    vec![
        Browser::new(
            BrowserType::Firefox,
            BinaryLocation::Snap,
            "Firefox",
            "firefox",
            "snap/firefox/common/profiles",
        ),
        Browser::new(
            BrowserType::Chromium,
            BinaryLocation::Snap,
            "Vivaldi",
            "vivaldi.vivaldi-stable",
            "snap/vivaldi/common/profiles",
        ),
        Browser::new(
            BrowserType::Chromium,
            BinaryLocation::Snap,
            "Chromium",
            "chromium",
            "snap/chromium/common/profiles",
        ),
        Browser::new(
            BrowserType::Falkon,
            BinaryLocation::Snap,
            "Falkon",
            "falkon",
            "snap/falkon/common/profiles",
        ),
    ]
}
