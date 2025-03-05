export default {
  mode: "all",
  content: ["./src/**/*.{rs,html,css}", "./dist/**/*.html"],
  theme: {
    extend: {},
  },
  plugins: [require("@tailwindcss/forms")],
};
