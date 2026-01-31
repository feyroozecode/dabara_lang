import init, { DabaraRuntime, version } from '../pkg/dabara.js';

let runtime;

// ── Dabara Syntax Highlighter ──────────────────────────────────

const KEYWORDS = new Set([
    'fara', 'ƙare', 'kare', 'rubuta', 'idan', 'amma', 'ammaina',
    'maimaita', 'ga', 'cikin', 'katse', 'ci_gaba', 'aiki', 'mayar',
]);

const DECL_KEYWORDS = new Set(['var', 'naɗa', 'nada']);

const BOOLEANS = new Set(['gaskiya', 'karya']);

const BUILTINS = new Set(['karɓa']);

// Tokenize source into spans with CSS classes
function highlightCode(source) {
    let result = '';
    let i = 0;
    const len = source.length;

    while (i < len) {
        const ch = source[i];

        // Comment
        if (ch === '#') {
            let end = i;
            while (end < len && source[end] !== '\n') end++;
            result += span('comment', esc(source.slice(i, end)));
            i = end;
            continue;
        }

        // String
        if (ch === '"') {
            let end = i + 1;
            while (end < len && source[end] !== '"') end++;
            if (end < len) end++; // include closing quote
            result += span('string', esc(source.slice(i, end)));
            i = end;
            continue;
        }

        // Number
        if (isDigit(ch) || (ch === '.' && i + 1 < len && isDigit(source[i + 1]))) {
            let end = i;
            while (end < len && (isDigit(source[end]) || source[end] === '.')) end++;
            result += span('number', esc(source.slice(i, end)));
            i = end;
            continue;
        }

        // Identifier / keyword
        if (isIdentStart(ch)) {
            let end = i;
            while (end < len && isIdentChar(source[end])) end++;
            const word = source.slice(i, end);

            if (KEYWORDS.has(word)) {
                result += span('keyword', word);
            } else if (DECL_KEYWORDS.has(word)) {
                result += span('variable', word);
            } else if (BOOLEANS.has(word)) {
                result += span('boolean', word);
            } else if (BUILTINS.has(word)) {
                result += span('builtin', word);
            } else {
                // Check if followed by ( → function call
                let peek = end;
                while (peek < len && source[peek] === ' ') peek++;
                if (peek < len && source[peek] === '(') {
                    result += span('function', word);
                } else {
                    result += esc(word);
                }
            }
            i = end;
            continue;
        }

        // Dot followed by identifier → method call
        if (ch === '.') {
            let end = i + 1;
            if (end < len && isIdentStart(source[end])) {
                let mEnd = end;
                while (mEnd < len && isIdentChar(source[mEnd])) mEnd++;
                result += span('operator', '.');
                result += span('method', source.slice(end, mEnd));
                i = mEnd;
                continue;
            }
        }

        // Operators
        if ('+-*/'.includes(ch)) {
            result += span('operator', esc(ch));
            i++;
            continue;
        }

        // Comparison operators (==, !=, <=, >=, <, >)
        if (ch === '=' && i + 1 < len && source[i + 1] === '=') {
            result += span('operator', '==');
            i += 2;
            continue;
        }
        if (ch === '!' && i + 1 < len && source[i + 1] === '=') {
            result += span('operator', '!=');
            i += 2;
            continue;
        }
        if (ch === '<' && i + 1 < len && source[i + 1] === '=') {
            result += span('operator', '&lt;=');
            i += 2;
            continue;
        }
        if (ch === '>' && i + 1 < len && source[i + 1] === '=') {
            result += span('operator', '&gt;=');
            i += 2;
            continue;
        }
        if (ch === '<') {
            result += span('operator', '&lt;');
            i++;
            continue;
        }
        if (ch === '>') {
            result += span('operator', '&gt;');
            i++;
            continue;
        }

        // Assignment =
        if (ch === '=') {
            result += span('operator', '=');
            i++;
            continue;
        }

        // Brackets
        if ('(){}[]'.includes(ch)) {
            result += span('bracket', esc(ch));
            i++;
            continue;
        }

        // Default: whitespace, newlines, commas, etc.
        result += esc(ch);
        i++;
    }

    // Append a trailing newline so the pre height matches textarea
    result += '\n';
    return result;
}

function span(cls, content) {
    return `<span class="syn-${cls}">${content}</span>`;
}

function esc(text) {
    return text.replace(/&/g, '&amp;').replace(/</g, '&lt;').replace(/>/g, '&gt;');
}

function isDigit(c) {
    return c >= '0' && c <= '9';
}

function isIdentStart(c) {
    return /[a-zA-Z_ɓɗƙƴʔ]/.test(c);
}

function isIdentChar(c) {
    return /[a-zA-Z0-9_ɓɗƙƴʔ]/.test(c);
}

// ── Example programs ───────────────────────────────────────────

const EXAMPLES = {
    hello: `fara
  rubuta "Sannu, Duniya!"
ƙare`,

    variables: `fara
  var sunan = "Musa"
  var shekara = 25
  rubuta "Suna: " + sunan
  rubuta "Shekara: " + shekara
  rubuta sunan + " yana da shekara " + shekara
ƙare`,

    conditions: `fara
  var maki = 75

  idan maki >= 70 {
    rubuta "Ka yi nasara! Maki: " + maki
  } ammaina maki >= 50 {
    rubuta "Ka wuce kawai. Maki: " + maki
  } amma {
    rubuta "Ka kasa. Maki: " + maki
  }
ƙare`,

    loops: `fara
  # Buga lambobi 1 zuwa 5
  var i = 1
  maimaita (i <= 5) {
    rubuta "Lamba: " + i
    var i = i + 1
  }

  # For loop tare da jeri
  var sunaye = ["Musa", "Issa", "Fatima"]
  ga suna cikin sunaye {
    rubuta "Sannu, " + suna + "!"
  }
ƙare`,

    functions: `fara
  aiki sallama(suna) {
    mayar "Sannu, " + suna + "!"
  }

  aiki jimlar(a, b) {
    mayar a + b
  }

  rubuta sallama("Ahmad")
  rubuta sallama("Fatima")

  var sakamakon = jimlar(10, 20)
  rubuta "Jimla: " + sakamakon
ƙare`,

    lists: `fara
  var lambobi = [10, 20, 30, 40, 50]
  rubuta "Jeri: " + lambobi
  rubuta "Tsawo: " + lambobi.tsawo()

  # Zagaya jerin
  ga lamba cikin lambobi {
    rubuta "Lamba: " + lamba
  }

  # Haruffan jimla
  var kalma = "Dabara"
  rubuta "Tsawon kalma: " + kalma.tsawo()
  rubuta "Babba: " + kalma.babba()
  rubuta "Karami: " + kalma.karami()
ƙare`,

    fibonacci: `fara
  aiki fibonacci(n) {
    idan n <= 1 {
      mayar n
    }
    mayar fibonacci(n - 1) + fibonacci(n - 2)
  }

  var i = 0
  maimaita (i < 10) {
    rubuta "fib(" + i + ") = " + fibonacci(i)
    var i = i + 1
  }
ƙare`,
};

// ── DOM elements ───────────────────────────────────────────────

const codeEditor = document.getElementById('code-editor');
const highlightLayer = document.getElementById('highlight-layer');
const outputEl = document.getElementById('output');
const runBtn = document.getElementById('run-btn');
const resetBtn = document.getElementById('reset-btn');
const clearOutputBtn = document.getElementById('clear-output');
const examplesSelect = document.getElementById('examples-select');
const errorBar = document.getElementById('error-bar');
const lineInfo = document.getElementById('line-info');
const versionBadge = document.getElementById('version-badge');

// ── Highlighting sync ──────────────────────────────────────────

function updateHighlight() {
    highlightLayer.innerHTML = highlightCode(codeEditor.value);
}

function syncScroll() {
    highlightLayer.scrollTop = codeEditor.scrollTop;
    highlightLayer.scrollLeft = codeEditor.scrollLeft;
}

// ── Core functions ─────────────────────────────────────────────

function showError(message) {
    errorBar.textContent = message;
    errorBar.classList.remove('hidden');
    outputEl.classList.add('has-error');
}

function hideError() {
    errorBar.classList.add('hidden');
    outputEl.classList.remove('has-error');
}

function runCode() {
    hideError();
    const code = codeEditor.value;

    if (!code.trim()) {
        outputEl.textContent = '';
        return;
    }

    try {
        const result = runtime.run_code(code);
        outputEl.textContent = result;
    } catch (e) {
        const errorMsg = typeof e === 'string' ? e : e.message || String(e);
        outputEl.textContent = errorMsg;
        showError(errorMsg);
    }
}

function resetRuntime() {
    runtime.reset();
    outputEl.textContent = '';
    hideError();
}

function loadExample(name) {
    if (EXAMPLES[name]) {
        codeEditor.value = EXAMPLES[name];
        updateHighlight();
        resetRuntime();
    }
}

function updateLineInfo() {
    const pos = codeEditor.selectionStart;
    const text = codeEditor.value.substring(0, pos);
    const line = text.split('\n').length;
    lineInfo.textContent = `Layi: ${line}`;
}

// Tab key support in editor
function handleTab(e) {
    if (e.key === 'Tab') {
        e.preventDefault();
        const start = codeEditor.selectionStart;
        const end = codeEditor.selectionEnd;
        codeEditor.value = codeEditor.value.substring(0, start) + '  ' + codeEditor.value.substring(end);
        codeEditor.selectionStart = codeEditor.selectionEnd = start + 2;
        updateHighlight();
    }
}

// ── Init ───────────────────────────────────────────────────────

async function main() {
    try {
        await init();
        runtime = new DabaraRuntime();

        // Show version
        try {
            versionBadge.textContent = 'v' + version();
        } catch (_) {
            versionBadge.textContent = 'v0.2.0';
        }

        // Initial highlight
        updateHighlight();

        // Event listeners
        runBtn.addEventListener('click', runCode);
        resetBtn.addEventListener('click', resetRuntime);
        clearOutputBtn.addEventListener('click', () => {
            outputEl.textContent = '';
            hideError();
        });

        examplesSelect.addEventListener('change', (e) => {
            if (e.target.value) {
                loadExample(e.target.value);
                e.target.value = '';
            }
        });

        codeEditor.addEventListener('input', () => {
            updateHighlight();
            updateLineInfo();
        });

        codeEditor.addEventListener('scroll', syncScroll);

        codeEditor.addEventListener('keydown', (e) => {
            handleTab(e);
            // Ctrl+Enter / Cmd+Enter to run
            if ((e.ctrlKey || e.metaKey) && e.key === 'Enter') {
                e.preventDefault();
                runCode();
            }
        });

        codeEditor.addEventListener('click', updateLineInfo);
        codeEditor.addEventListener('keyup', updateLineInfo);

        outputEl.textContent = 'Dabara yana shirye! Danna "Gudanar" ko Ctrl+Enter.\n';
    } catch (e) {
        outputEl.textContent = 'Kuskure wajen loda WASM: ' + e;
        console.error('WASM init failed:', e);
    }
}

main();
