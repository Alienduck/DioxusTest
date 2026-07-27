/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./src/**/*.{html,js,jsx,rs,tsx,ts}"],
  theme: {
    extend: {
      fontFamily: {
        sans: ['Inter', 'system-ui', '-apple-system', 'sans-serif'],
        display: ['SF Pro Display', 'Inter', 'system-ui', '-apple-system', 'sans-serif'],
      },
      colors: {
        primary: '#0F0F23',
        secondary: '#1A1A3E',
        accent: '#6366f1',
      },
    },
  },
  plugins: [],
}