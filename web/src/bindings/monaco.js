var monacoEditor = null;
var currentMonacoModel = null;

export function initMonaco(elementId, initialSnippet) {
    require.config({ paths: { vs: './monaco-editor-0.52/vs' } });

    require(['vs/editor/editor.main'], function () {

        // const media = window.matchMedia("(prefers-color-scheme: dark");
        // let currentTheme = "vs";
        // if (media.matches) {{
        //     currentTheme = "vs-dark";
        // }}

        // Create theme
        monaco.editor.defineTheme("dx-vs-dark", {
            base: "vs-dark",
            inherit: true,
            rules: [],
            colors: {
                "editor.background": "#2B303B",
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

        var model = monaco.editor.createModel(initialSnippet, 'rust');

        var editor = monaco.editor.create(document.getElementById(elementId), {
            model: model,
            automaticLayout: true,
            theme: "dx-vs-dark",
        });

        monacoEditor = editor;
        currentMonacoModel = model;


        // Add theme logic
        // media.addEventListener("change", () => {{
        //     if (media.matches) {{
        //         monaco.editor.setTheme("vs-dark");
        //     }} else {{
        //         monaco.editor.setTheme("vs");
        //     }}
        // }});
    });
}

export function registerPasteAsRSX(convertHtmlToRSX) {
    if (!monacoEditor) {
        setTimeout(() => registerPasteAsRSX(convertHtmlToRSX), 1000);
        return;
    }

    monacoEditor.addAction({
        id: "paste-as-rsx",
        label: "Paste as RSX",
        contextMenuGroupId: '9_cutcopypaste',
        run: function (ed) {
            {
                navigator.clipboard.readText().then(text => {
                    {
                        // Attempt to conver to RSX but paste as normal if failure.
                        let modText = convertHtmlToRSX(text);
                        if (!modText) {
                            modText = text;
                        }

                        ed.executeEdits('', [{
                            range: ed.getSelection(),
                            text: modText,
                            forceMoveMarkers: true,
                        }]);
                    }
                });
            }
        },
    });
}

export function getCurrentModelValue() {
    return currentMonacoModel.getValue();
}
