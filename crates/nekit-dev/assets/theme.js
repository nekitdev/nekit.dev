const THEME = "theme";
const LIGHT = "light";
const DARK = "dark";

try {
    const theme = localStorage.getItem(THEME);

    if (theme === DARK || theme === LIGHT) {
        document.documentElement.dataset.theme = theme;
    }
} catch {}
