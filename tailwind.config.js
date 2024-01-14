/** @type {import('tailwindcss').Config} */
export default {
  content: ['./src/**/*.{html,js,svelte,ts}'],
  theme: {
    extend: {
      animation: {
        'spin-slow': 'spin 3s linear infinite',
      },
      height: {
        'fullscreen': 'calc(100vh - 6.75rem)',
        'aiPage': 'calc(100vh - 6.75rem - 78px)'
      },
      minHeight: {
        'fullscreen': 'calc(100vh - 6.75rem)'
      },
      dropShadow: {
        'navBtn': '0 4px 3px rgba(60, 116, 126, 0.65)',
        'aiBtn': '0 2px 3px rgba(192, 223, 225, 0.45)',
        'greenBtn': '0 2px 3px rgba(74, 222, 128, 0.45)',
        'redBtn': '0 2px 3px rgba(248, 113, 113, 0.45)'
      },
      colors: {
        'cotton': '#EFE6DD',
        'neptune': {
          '50': '#f2f9f9',
          '100': '#ddeff0',
          '200': '#c0dfe1',
          '300': '#7ebdc2', // Main color
          '400': '#60a8b0',
          '500': '#458c95',
          '600': '#3c747e',
          '700': '#365f68',
          '800': '#325158',
          '900': '#2e444b',
          '950': '#1a2c32',
        },
        'midnight': {
          '50': '#f6f6f6',
          '100': '#e7e7e7',
          '200': '#d1d1d1',
          '300': '#b0b0b0',
          '400': '#888888',
          '500': '#6d6d6d',
          '600': '#5d5d5d',
          '700': '#4f4f4f',
          '800': '#454545',
          '900': '#3d3d3d',
          '950': '#0c0c0c', // Main color
        },      
      }
    },
  },
  plugins: [],
}

