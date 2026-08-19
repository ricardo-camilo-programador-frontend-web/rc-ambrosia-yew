/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./index.html", "./src/**/*.rs"],
  theme: {
    extend: {
      colors: {
        ambrosia: {
          ink: "#0c0b09",
          charcoal: "#1a1a1a",
          gold: "#e0b86b",
          crimson: "#9e3a3a",
          cream: "#fff8e7"
        }
      },
      fontFamily: {
        editorial: ["Cormorant Garamond", "Georgia", "serif"]
      },
      boxShadow: {
        editorial: "0 24px 60px rgba(0, 0, 0, 0.45)"
      }
    }
  },
  plugins: []
};
