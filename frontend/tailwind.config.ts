/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    "./pages/**/*.{js,ts,jsx,tsx}",
    "./components/**/*.{js,ts,jsx,tsx}",
    "./context/**/*.{js,ts,jsx,tsx}",
  ],
  theme: {
    extend: {
      fontFamily: {
        "josefin-sans": ['"Josefin Sans"', "sans-serif"],
        montserrat: ['"Montserrat"', "sans-serif"],
        bungee: ["Bungee", "sans-serif"],
      },
      colors: {
        primary: "#b3b3ff",
        secondary: "#9999ff",
        tertiary: "#8080ff",
        quaternary: "#6666ff",
        quinary: "#4d4dff",
        "btn-left-bg": "#6c8df8",
        "btn-left-text": "#444",
        "btn-right-text": "#eee",
      },
      textShadow: {
        custom:
          "3px 3px 0 var(--color-secondary), 6px 6px 0 var(--color-tertiary), 9px 9px var(--color-quaternary), 12px 12px 0 var(--color-quinary)",
      },
      screens: {
        "max-sm": { max: "480px" },
      },
      keyframes: {
        moveToLeft: {
          "0%": { transform: "translateX(120px)", opacity: "0" },
          "100%": { transform: "translateX(0)", opacity: "1" },
        },
        moveToRight: {
          "0%": { transform: "translateX(-120px)", opacity: "0" },
          "100%": { transform: "translateX(0)", opacity: "1" },
        },
        animateBtn: {
          "0%": { opacity: "0" },
          "100%": { opacity: "1" },
          shadows: "shadows 1.2s ease-in infinite",
          move: "move 1.2s ease-in infinite",
        },

        shadows: {
          "0%": { textShadow: "none" },
          "10%": { textShadow: "3px 3px 0 var(--color-secondary)" },
          "20%": {
            textShadow:
              "3px 3px 0 var(--color-secondary), 6px 6px 0 var(--color-tertiary)",
          },
          "30%": {
            textShadow:
              "3px 3px 0 var(--color-secondary), 6px 6px 0 var(--color-tertiary), 9px 9px var(--color-quaternary)",
          },
          "40%": {
            textShadow:
              "3px 3px 0 var(--color-secondary), 6px 6px 0 var(--color-tertiary), 9px 9px var(--color-quaternary), 12px 12px 0 var(--color-quinary)",
          },
          "50%": {
            textShadow:
              "3px 3px 0 var(--color-secondary), 6px 6px 0 var(--color-tertiary), 9px 9px var(--color-quaternary), 12px 12px 0 var(--color-quinary)",
          },
          "60%": {
            textShadow:
              "3px 3px 0 var(--color-secondary), 6px 6px 0 var(--color-tertiary), 9px 9px var(--color-quaternary), 12px 12px 0 var(--color-quinary)",
          },
          "70%": {
            textShadow:
              "3px 3px 0 var(--color-secondary), 6px 6px 0 var(--color-tertiary), 9px 9px var(--color-quaternary)",
          },
          "80%": {
            textShadow:
              "3px 3px 0 var(--color-secondary), 6px 6px 0 var(--color-tertiary)",
          },
          "90%": { textShadow: "3px 3px 0 var(--color-secondary)" },
          "100%": { textShadow: "none" },
        },
        move: {
          "0%": { transform: "translate(0px, 0px)" },
          "40%": { transform: "translate(-12px, -12px)" },
          "50%": { transform: "translate(-12px, -12px)" },
          "60%": { transform: "translate(-12px, -12px)" },
          "100%": { transform: "translate(0px, 0px)" },
        },
      },
      animation: {
        moveToLeft: "moveToLeft 2s ease",
        moveToRight: "moveToRight 2s ease",
        animateBtn: "animateBtn 1s ease",
        shadows: "shadows 1.2s ease-in infinite",
        move: "move 1.2s ease-in infinite",
        "animate-btn-left": "animateBtn 1s 1.5s backwards",
        "animate-btn-right": "animateBtn 1s 2.3s backwards",
      },
      scale: {
        120: "1.2",
      },
    },
  },
  plugins: [
    function ({ addUtilities }) {
      addUtilities(
        {
          ".text-shadow": {
            textShadow:
              "3px 3px 0 var(--color-secondary), 6px 6px 0 var(--color-tertiary), 9px 9px var(--color-quaternary), 12px 12px 0 var(--color-quinary)",
          },
        },
        ["responsive", "hover"]
      );
    },
  ],
};
