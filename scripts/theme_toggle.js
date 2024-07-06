var themeToggleDarkIcon = document.getElementById("theme-toggle-dark-icon");
var themeToggleLightIcon = document.getElementById("theme-toggle-light-icon");
var themeToggleBtn = document.getElementById("theme-toggle");

// Initial theme check and apply
if (
    localStorage.getItem("theme") === "dark" ||
    (!("theme" in localStorage) &&
        window.matchMedia("(prefers-color-scheme: dark)").matches)
) {
    document.documentElement.classList.add("dark");
    themeToggleLightIcon !== null &&
        themeToggleLightIcon.classList.remove("hidden");
} else {
    document.documentElement.classList.add("light");
    themeToggleDarkIcon !== null &&
        themeToggleDarkIcon.classList.remove("hidden");
}

// Event listener for the toggle button
themeToggleBtn !== null &&
    themeToggleBtn.addEventListener("click", function () {
        themeToggleDarkIcon !== null &&
            themeToggleDarkIcon.classList.toggle("hidden");
        themeToggleLightIcon !== null &&
            themeToggleLightIcon.classList.toggle("hidden");

        var currentTheme = localStorage.getItem("theme");

        if (currentTheme === "light") {
            document.documentElement.classList.remove("light");
            document.documentElement.classList.add("dark");
            localStorage.setItem("theme", "dark");
        } else {
            document.documentElement.classList.remove("dark");
            document.documentElement.classList.add("light");
            localStorage.setItem("theme", "light");
        }

        // If no theme is set in localStorage, default to the opposite theme
        if (!currentTheme) {
            var newTheme = document.documentElement.classList.contains("dark")
                ? "light"
                : "dark";
            document.documentElement.classList.toggle("dark");
            document.documentElement.classList.toggle("light");
            localStorage.setItem("theme", newTheme);
        }
    });
