export default {
  mode: "all",
  content: ["./src/**/*.{rs,html,css}"],
  theme: {
    extend: {},
  },
  plugins: [require("@tailwindcss/forms")],
};
