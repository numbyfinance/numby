// Light switcher
const lightSwitches: NodeListOf<HTMLInputElement> =
  document.querySelectorAll(".light-switch");

if (lightSwitches.length > 0) {
  lightSwitches.forEach((lightSwitch: HTMLInputElement, i: number) => {
    if (localStorage.getItem("dark-mode") === "true") {
      lightSwitch.checked = true;
    }

    lightSwitch.addEventListener("change", () => {
      const { checked } = lightSwitch;

      lightSwitches.forEach((el: HTMLInputElement, n: number) => {
        if (n !== i) {
          el.checked = checked;
        }
      });

      document.documentElement.classList.add("**:transition-none!");

      if (lightSwitch.checked) {
        document.documentElement.classList.add("dark");
        document.querySelector("html")!.style.colorScheme = "dark";
        localStorage.setItem("dark-mode", "true");
        document.dispatchEvent(
          new CustomEvent("darkMode", { detail: { mode: "on" } }),
        );
      } else {
        document.documentElement.classList.remove("dark");
        document.querySelector("html")!.style.colorScheme = "light";
        localStorage.setItem("dark-mode", "false");
        document.dispatchEvent(
          new CustomEvent("darkMode", { detail: { mode: "off" } }),
        );
      }

      setTimeout(() => {
        document.documentElement.classList.remove("**:transition-none!");
      }, 1);
    });
  });
}
