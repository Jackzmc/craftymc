module.exports = {
  root: true,
  env: {
    node: true
  },
  extends: [
    "plugin:vue/essential",
    "@vue/typescript"
  ],
  parserOptions: {
    parser: "@typescript-eslint/parser"
  },
  rules: {
    'no-console': process.env.NODE_ENV === 'production' ? 'warn' : 'off',
    'no-debugger': process.env.NODE_ENV === 'production' ? 'warn' : 'off',
    "vue/multi-word-component-names": "off",
    semi: "off",
    "vue/no-template-key": "off",
    "vue/no-v-for-template-key": "off",
    quotes: 'off',
    "space-before-function-paren": "off",
    "keyword-spacing": "off",
    "no-unused-vars": "off",
    "@typescript-eslint/no-unused-vars": "error",
    "vue/no-v-model-argument": "off"
  }
}
