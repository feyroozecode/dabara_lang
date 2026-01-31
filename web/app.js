import init, { DabaraRuntime, version } from '../pkg/dabara.js';

let runtime;

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

// DOM elements
const codeEditor = document.getElementById('code-editor');
const outputEl = document.getElementById('output');
const runBtn = document.getElementById('run-btn');
const resetBtn = document.getElementById('reset-btn');
const clearOutputBtn = document.getElementById('clear-output');
const examplesSelect = document.getElementById('examples-select');
const errorBar = document.getElementById('error-bar');
const lineInfo = document.getElementById('line-info');
const versionBadge = document.getElementById('version-badge');

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
    }
}

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

        codeEditor.addEventListener('keydown', (e) => {
            handleTab(e);
            // Ctrl+Enter / Cmd+Enter to run
            if ((e.ctrlKey || e.metaKey) && e.key === 'Enter') {
                e.preventDefault();
                runCode();
            }
        });

        codeEditor.addEventListener('input', updateLineInfo);
        codeEditor.addEventListener('click', updateLineInfo);
        codeEditor.addEventListener('keyup', updateLineInfo);

        outputEl.textContent = 'Dabara yana shirye! Danna "Gudanar" ko Ctrl+Enter.\n';
    } catch (e) {
        outputEl.textContent = 'Kuskure wajen loda WASM: ' + e;
        console.error('WASM init failed:', e);
    }
}

main();
