/** @type {import('tailwindcss').Config} */
export default {
  content: [
    './pages/**/*.{vue,js,ts}',
    './components/**/*.{vue,js,ts}',
    './layouts/**/*.{vue,js,ts}',
    './plugins/**/*.{js,ts}',
    './nuxt.config.{js,ts}'
  ],
  theme: {
    extend: {
      colors: {
        background: '#EFFFFB',
        primary: '#4F98CA',
        secondary: '#50D890',
        accent: '#272727',
      },
    },
  },
  plugins: [],
}

