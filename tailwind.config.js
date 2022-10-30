/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./src/**/*.{html,cshtml,razor}"],
  theme: {
    container: {
      center: true
    },
    extend: {},
  },
  plugins: [require("@tailwindcss/typography"), require("daisyui")],
}
