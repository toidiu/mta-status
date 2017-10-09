module.exports = {
  router: {
    // base: '/mta-status/'
  },
  build: {
    vendor: [
      'axios'
    ]
  },
  modules: [
    //'@nuxtjs/bulma'
    //'@nuxtjs/font-awesome'
  ],
  css: [
    // Load a node module directly (here it's a SASS file)
    'bulma',
    { src: 'font-awesome/scss/font-awesome.scss', lang: 'scss' },
    // CSS file in the project
    '@/assets/css/main.css',
    // SCSS file in the project
    '@/assets/css/main.scss'
  ],
  /*
  ** Headers of the page
  */
  head: {
    title: 'mta-status',
    meta: [
      { charset: 'utf-8' },
      { name: 'viewport', content: 'width=device-width, initial-scale=1' },
      { hid: 'description', name: 'description', content: 'technology' }
    ],
    link: [
      { rel: 'icon', type: 'image/x-icon', href: '/favicon.ico' }
    ]
  },
  /*
  ** Customize the progress bar color
  */
  loading: { color: '#3B8070' },
  /*
  ** Build configuration
  */
  build: {
    /*
    ** Run ESLint on save
    */
    extend (config, ctx) {
      if (ctx.dev && ctx.isClient) {
        config.module.rules.push({
          enforce: 'pre',
          test: /\.(js|vue)$/,
          loader: 'eslint-loader',
          exclude: /(node_modules)/
        })
      }
    }
  }
}
