/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./index.html", "./src/**/*.rs"],
  theme: {
    extend: {
      colors: {
        ambrosia: {
          ink: "#14110e",
          charcoal: "#231c17",
          gold: "#f0d09a",
          crimson: "#8e2a24",
          cream: "#f7f1e6"
        }
      },
      fontFamily: {
        editorial: ["Fraunces", "Georgia", "serif"]
      },
      boxShadow: {
        editorial: "0 24px 60px rgba(0, 0, 0, 0.45)"
      }
    }
  },
  plugins: []
};
