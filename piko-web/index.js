let vm = null;
let wasmModule = null;

async function initWasm() {
    try {
        const module = await import('./wasm-build/pkg/piko_web.js');
        await module.default();
        wasmModule = module;
        vm = new module.PikoVM();
    } catch (error) {
        document.getElementById('output-box').textContent = 'Failed to load WASM module';
    }
}

function runCode() {
    if (!vm) {
        document.getElementById('output-box').textContent = 'WASM not initialized';
        return;
    }
    
    const code = document.getElementById('code-editor').value;
    const outputBox = document.getElementById('output-box');
    
    try {
        vm.execute(code);
        const output = vm.get_output();
        outputBox.textContent = output || 'No output';
    } catch (error) {
        outputBox.textContent = 'Error: ' + error;
    }
}

function switchTab(tabId) {
    document.querySelectorAll('.tab').forEach(tab => tab.classList.remove('active'));
    document.querySelectorAll('.view').forEach(view => view.classList.remove('active'));
    
    document.getElementById(tabId).classList.add('active');
    document.getElementById(tabId.replace('-tab', '-view')).classList.add('active');
}

function loadExample(name) {    
    const example = wasmModule.get_example(name);
    if (example) {
        document.getElementById('code-editor').value = example;
        switchTab('code-tab');
    }
}

document.addEventListener('DOMContentLoaded', function() {
    document.getElementById('run-btn').addEventListener('click', runCode);
    document.getElementById('code-tab').addEventListener('click', () => switchTab('code-tab'));
    document.getElementById('examples-tab').addEventListener('click', () => switchTab('examples-tab'));
    document.getElementById('help-tab').addEventListener('click', () => switchTab('help-tab'));
    
    document.querySelectorAll('.example-item').forEach(item => {
        item.addEventListener('click', () => {
            const exampleName = item.dataset.example;
            loadExample(exampleName);
        });
    });
    
    initWasm();
});