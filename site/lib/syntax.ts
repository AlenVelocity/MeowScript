import Prism, { languages } from 'prismjs'

export const meowScriptSyntax = languages.extend('clike', {
  comment: [
    {
      pattern: /\/\//
    },
    {
      pattern: /(^|[^\\:])\/\/.*/,
      lookbehind: false,
      greedy: true
    }
  ],
  string: {
    pattern: /(["`])(?:\\[\s\S]|(?!\1)[^\\])*\1/,
    greedy: true
  },
  keyword: /\b(?:scratch|amew|pawction|purrhaps|meowtually|tail|pawckage|purrfect|clawful|furreal|furrever|hiss|continue)\b/,
  boolean: /\b(?:BUGGER ALL|NAH|YEAH)\b/,
  number: /(?:(?:\b\d+(?:\.\d*)?|\B\.\d+)(?:e[-+]?\d+)?)i?/i,
  operator:
    /[*/%^!=]=?|~|\+[=+]?|-[=-]?|\|[=|]?|&(?:=|&|\^=?)?|>(?:>=?|=)?|<(?:<=?|=|-)?|:=|\.\.\./
})

Prism.languages.meowScript = meowScriptSyntax
