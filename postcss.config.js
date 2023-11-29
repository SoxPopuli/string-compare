//import autoprefixer from 'autoprefixer'
//import sass from '@csstools/postcss-sass'

/** @type {import('postcss-load-config').Config} */
const config = {
  plugins: [
    require('autoprefixer'),
    require('@csstools/postcss-sass'),
  ]
}

module.exports = config
