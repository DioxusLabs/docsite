var monacoEditor = null;
var currentMonacoModel = null;

export function initMonaco(
  vsPathPrefix,
  elementId,
  initialTheme,
  initialSnippet, 
  onReadyCallback,
) {
  require.config({ paths: { vs: vsPathPrefix } });

  require(["vs/editor/editor.main"], function () {
    monaco.editor.onDidCreateModel((_model) => onReadyCallback());

    // Light Theme
    monaco.editor.defineTheme("dx-vs", {
      base: "vs",
      inherit: true,
      rules: [],
      colors: {
        "editor.background": "#FFFFFF",
        // "editor.background": "#DCDFE5",
        "editorWidget.background": "#FFFFFF",
        // "editorWidget.background": "#EDEFF2",
        "editorWidget.border": "#A5A5A5",
        "input.background": "#E6E6E6",
        "editor.lineHighlightBackground": "#E6E6E6",
        "editor.lineHighlightBorder": "#E6E6E6",
        "list.hoverBackground": "#E6E6E6",
        "dropdown.background": "#EDEFF2",
        "dropdown.border": "#A5A5A5",
      },
    });

    // Dark theme
    monaco.editor.defineTheme("dx-vs-dark", {
      base: "vs-dark",
      inherit: true,
      rules: [
        { token: "keyword.control", foreground: "#C586C0" },
        { token: "string.escape", foreground: "#D7BA7D" },
        { token: "keyword.controlFlow", foreground: "#C586C0" },
        { token: "variable", foreground: "#9CDCFE" },
        { token: "parameter", foreground: "#9CDCFE" },
        { token: "property", foreground: "#9CDCFE" },
        { token: "support.function", foreground: "#DCDCAA" },
        { token: "function", foreground: "#DCDCAA" },
        { token: "member", foreground: "#4FC1FF" },
        { token: "variable.constant", foreground: "#4FC1FF" },
        { token: "macro", foreground: "#569CD6" },
        { token: "typeParameter", foreground: "#4EC9B0" },
        { token: "interface", foreground: "#4EC9B0" },
        { token: "namespace", foreground: "#4EC9B0" },
        { token: "variable.mutable", fontStyle: "underline" },
        { token: "parameter.mutable", fontStyle: "underline" },
      ],
      colors: {
        "editor.background": "#000000",
        "editorWidget.background": "#454E61",
        "editorWidget.border": "#5B667D",
        "input.background": "#21252E",
        "editor.lineHighlightBackground": "#21252E",
        "editor.lineHighlightBorder": "#21252E",
        "list.hoverBackground": "#21252E",
        "dropdown.background": "#454E61",
        "dropdown.border": "#5B667D",
      },
    });

    // Setup rust language providers
    monaco.languages.register({ id: "rust" });
    monaco.languages.setLanguageConfiguration("rust", rustLangConfig);
    monaco.languages.setMonarchTokensProvider("rust", rustLangGrammar);

    monaco.languages.onLanguage("rust", async () => {
      monaco.languages.setLanguageConfiguration("rust", rustLangConfig);
      monaco.languages.setMonarchTokensProvider("rust", rustLangGrammar);
    });

    var model = monaco.editor.createModel(initialSnippet, "rust");
    var editor = monaco.editor.create(document.getElementById(elementId), {
      model: model,
      automaticLayout: true,
      theme: initialTheme, //dx-vs-dark
      minimap: { enabled: false },
      "semanticHighlighting.enabled": true,
    });

    monacoEditor = editor;
    currentMonacoModel = model;
  });
}

export function getCurrentModelValue() {
  if (!isReady) return;
  return currentMonacoModel.getValue();
}

export function setCurrentModelvalue(value) {
  if (!isReady) return;
  currentMonacoModel.setValue(value);
}

export function isReady() {
  return monacoEditor && currentMonacoModel;
}

export function setTheme(theme) {
  if (monacoEditor) {
    monaco.editor.setTheme(theme);
  }
}

export function setModelMarkers(markers) {
  if (!currentMonacoModel) {
    return;
  }

  // We need to convert severity to monaco's severity enum.
  for (let marker of markers) {
    marker.severity = monaco.MarkerSeverity[marker.severity];
  }

  monaco.editor.setModelMarkers(currentMonacoModel, "owner", markers);
}

export function registerPasteAsRSX(convertHtmlToRSX) {
  if (!monacoEditor) {
    setTimeout(() => registerPasteAsRSX(convertHtmlToRSX), 1000);
    return;
  }

  monacoEditor.addAction({
    id: "paste-as-rsx",
    label: "Paste as RSX",
    contextMenuGroupId: "9_cutcopypaste",
    run: function (ed) {
      {
        navigator.clipboard.readText().then((text) => {
          {
            // Attempt to conver to RSX but paste as normal if failure.
            let modText = convertHtmlToRSX(text);
            if (!modText) {
              modText = text;
            }

            ed.executeEdits("", [
              {
                range: ed.getSelection(),
                text: modText,
                forceMoveMarkers: true,
              },
            ]);
          }
        });
      }
    },
  });
}

export function registerModelChangeEvent(callback) {
  if (!monacoEditor) {
    setTimeout(() => registerModelChangeEvent(callback), 1000);
    return;
  }

  currentMonacoModel.onDidChangeContent(() => {
    let content = getCurrentModelValue();
    callback(content);
  });
}

// Rust language definitions (from rust-playground)
const rustLangConfig = {
  comments: {
    lineComment: "//",
    blockComment: ["/*", "*/"],
  },
  brackets: [
    ["{", "}"],
    ["[", "]"],
    ["(", ")"],
  ],
  autoClosingPairs: [
    { open: "[", close: "]" },
    { open: "{", close: "}" },
    { open: "(", close: ")" },
    { open: '"', close: '"', notIn: ["string"] },
  ],
  surroundingPairs: [
    { open: "{", close: "}" },
    { open: "[", close: "]" },
    { open: "(", close: ")" },
    { open: '"', close: '"' },
    { open: "'", close: "'" },
  ],
  folding: {
    markers: {
      start: new RegExp("^\\s*#pragma\\s+region\\b"),
      end: new RegExp("^\\s*#pragma\\s+endregion\\b"),
    },
  },
};

const rustLangGrammar = {
  // Set defaultToken to invalid to see what you do not tokenize yet
  //defaultToken: 'invalid',

  keywords: [
    "as",
    "break",
    "const",
    "crate",
    "enum",
    "extern",
    "false",
    "fn",
    "impl",
    "in",
    "let",
    "mod",
    "move",
    "async",
    "mut",
    "pub",
    "ref",
    "return",
    "self",
    "Self",
    "static",
    "struct",
    "super",
    "trait",
    "true",
    "type",
    "unsafe",
    "use",
    "where",
    "macro_rules",
  ],

  controlFlowKeywords: [
    "continue",
    "else",
    "for",
    "if",
    "while",
    "loop",
    "match",
  ],

  typeKeywords: [
    "Self",
    "m32",
    "m64",
    "m128",
    "f80",
    "f16",
    "f128",
    "int",
    "uint",
    "float",
    "char",
    "bool",
    "u8",
    "u16",
    "u32",
    "u64",
    "f32",
    "f64",
    "i8",
    "i16",
    "i32",
    "i64",
    "str",
    "Option",
    "Either",
    "c_float",
    "c_double",
    "c_void",
    "FILE",
    "fpos_t",
    "DIR",
    "dirent",
    "c_char",
    "c_schar",
    "c_uchar",
    "c_short",
    "c_ushort",
    "c_int",
    "c_uint",
    "c_long",
    "c_ulong",
    "size_t",
    "ptrdiff_t",
    "clock_t",
    "time_t",
    "c_longlong",
    "c_ulonglong",
    "intptr_t",
    "uintptr_t",
    "off_t",
    "dev_t",
    "ino_t",
    "pid_t",
    "mode_t",
    "ssize_t",
  ],

  operators: [
    "=",
    ">",
    "<",
    "!",
    "~",
    "?",
    ":",
    "==",
    "<=",
    ">=",
    "!=",
    "&&",
    "||",
    "++",
    "--",
    "+",
    "-",
    "*",
    "/",
    "&",
    "|",
    "^",
    "%",
    "<<",
    ">>",
    ">>>",
    "+=",
    "-=",
    "*=",
    "/=",
    "&=",
    "|=",
    "^=",
    "%=",
    "<<=",
    ">>=",
    ">>>=",
  ],

  // we include these common regular expressions
  symbols: /[=><!~?:&|+\-*\/\^%]+/,

  // for strings
  escapes:
    /\\(?:[abfnrtv\\"']|x[0-9A-Fa-f]{1,4}|u[0-9A-Fa-f]{4}|U[0-9A-Fa-f]{8})/,

  // The main tokenizer for our languages
  tokenizer: {
    root: [
      [/r"/, { token: "string.quote", next: "@rawstring0" }],
      [/r(#+)"/, { token: "string.quote", next: "@rawstring1.$1" }],
      // identifiers and keywords
      [
        /[a-z_$][\w$]*/,
        {
          cases: {
            "@typeKeywords": "type.identifier",
            "@keywords": {
              cases: {
                fn: { token: "keyword", next: "@func_decl" },
                "@default": "keyword",
              },
            },
            "@controlFlowKeywords": "keyword.control",
            "@default": "variable",
          },
        },
      ],
      [/[A-Z][\w\$]*/, "type.identifier"], // to show class names nicely

      // whitespace
      { include: "@whitespace" },

      // delimiters and operators
      [/[{}()\[\]]/, "@brackets"],
      [/[<>](?!@symbols)/, "@brackets"],
      [
        /@symbols/,
        {
          cases: {
            "@operators": "operator",
            "@default": "",
          },
        },
      ],

      // @ annotations.
      // As an example, we emit a debugging log message on these tokens.
      // Note: message are supressed during the first load -- change some lines to see them.
      [
        /@\s*[a-zA-Z_\$][\w\$]*/,
        { token: "annotation", log: "annotation token: $0" },
      ],

      // numbers
      [/\d*\.\d+([eE][\-+]?\d+)?/, "number.float"],
      [/0[xX][0-9a-fA-F]+/, "number.hex"],
      [/\d+/, "number"],

      // delimiter: after number because of .\d floats
      [/[;,.]/, "delimiter"],

      // strings
      [/"([^"\\]|\\.)*$/, "string.invalid"], // non-teminated string
      [/"/, { token: "string.quote", bracket: "@open", next: "@string" }],

      // characters
      [/'[^\\']'/, "string"],
      [/(')(@escapes)(')/, ["string", "string.escape", "string"]],
      [/'/, "string.invalid"],
    ],

    comment: [
      [/[^\/*]+/, "comment"],
      [/\/\*/, "comment", "@push"], // nested comment
      ["\\*/", "comment", "@pop"],
      [/[\/*]/, "comment"],
    ],

    rawstring0: [
      [/[^"]+/, "string"],
      [/"/, { token: "string.quote", next: "@pop" }],
    ],
    rawstring1: [
      [
        /"(#+)/,
        {
          cases: {
            "$1==$S2": { token: "string.quote", next: "@pop" },
            "@default": { token: "string" },
          },
        },
      ],
      [/./, "string"],
    ],
    string: [
      [/[^\\"]+/, "string"],
      [/@escapes/, "string.escape"],
      [/\\./, "string.escape.invalid"],
      [/"/, { token: "string.quote", bracket: "@close", next: "@pop" }],
    ],

    whitespace: [
      [/[ \t\r\n]+/, "white"],
      [/\/\*/, "comment", "@comment"],
      [/\/\/.*$/, "comment"],
    ],

    func_decl: [[/[a-zA-Z_$][\w$]*/, "support.function", "@pop"]],
  },
};
