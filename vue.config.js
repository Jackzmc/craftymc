const { defineConfig } = require('@vue/cli-service')
process.env.VUE_APP_VERSION = require('./package.json').version
module.exports = defineConfig({
  transpileDependencies: true,
  devServer: {
    allowedHosts: "all",
  }
})
