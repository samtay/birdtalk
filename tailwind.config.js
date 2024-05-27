/** @type {import('tailwindcss').Config} */
module.exports = {
  mode: "all",
  content: ["./src/**/*.{rs,html,css}", "./dist/**/*.html"],
  theme: {
    extend: {
      animation: {
        shake: 'shake 0.82s cubic-bezier(.36,.07,.19,.97) both',
        'fly-in': 'fly-in 1s cubic-bezier(0.165, 0.840, 0.440, 1.000) forwards',
        'fade-in': 'fly-in 0.5s cubic-bezier(0.165, 0.840, 0.440, 1.000) forwards',
      },
      keyframes: {
        shake: {
          '10%, 90%': {
            transform: 'translate3d(-1px, 0, 0)'
          },

          '20%, 80%': {
            transform: 'translate3d(2px, 0, 0)'
          },

          '30%, 50%, 70%': {
            transform: 'translate3d(-4px, 0, 0)'
          },

          '40%, 60%': {
            transform: 'translate3d(4px, 0, 0)'
          }
        },
        'fly-in': {
          '0%': {
            transform: 'translateX(1000px)',
            'border-radius': '9999px',
            height: '5rem',
            width: '5rem',
          },
          '65%': {
            transform: 'translateX(0px)',
            'border-radius': '9999px',
            height: '5rem',
            width: '5rem'
          },
          '95%': {
            'border-radius': '0.75rem',
            height: '28rem',
            width: '100%',
          },
          '100%': {
            'border-radius': '0.75rem',
            height: '24rem',
            width: '100%',
          }
        },
        'fade-in': {
          '0%': {
            opacity: '0%'
          },
          '100%': {
            opacity: '100%'
          }
        }
      }
    },
  },
  plugins: [],
};
