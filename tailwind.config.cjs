/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    './src/***.{html,js,svelte,ts}',
  ],
  theme: {
    extend: {},
  },
  plugins: [require('daisyui')],
  daisyui: {
    themes: [
      {
        zelfi: {
          ...require('daisyui/src/theming/themes')['dark'],
          'primary': 'oklch(76% 0.17 185)',
          'primary-content': 'oklch(15% 0.05 185)',
          'accent': 'oklch(80% 0.19 185)',
          'accent-content': 'oklch(15% 0.05 185)',
          'secondary': 'oklch(65% 0.12 200)',
          'secondary-content': 'oklch(13% 0.024 200)',
        },
      },
    ],
    darkTheme: 'zelfi',
    base: true,
    styled: true,
    utils: true,
  },
};
