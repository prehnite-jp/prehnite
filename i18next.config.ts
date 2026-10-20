export default {
  locales: [
    "en-US",
    "ja-JP"
  ],
  extract: {
    input: "src/**/*.{js,jsx,ts,tsx}",
    output: "public\\locales\\{{language}}\\{{namespace}}.json"
  }
}