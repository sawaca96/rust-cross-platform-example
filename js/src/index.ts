import init, * as wasm from '../pkg/calculator.js';

async function run() {
    await init();

    const addResult = document.getElementById('add-result') as HTMLElement;
    const subtractResult = document.getElementById('subtract-result') as HTMLElement;
    const multiplyResult = document.getElementById('multiply-result') as HTMLElement;
    const divideResult = document.getElementById('divide-result') as HTMLElement;
    const divideError = document.getElementById('divide-error') as HTMLElement;

    addResult.textContent = `1 + 2 = ${wasm.add(1, 2)}`;
    subtractResult.textContent = `5 - 2 = ${wasm.subtract(5, 2)}`;
    multiplyResult.textContent = `3 * 4 = ${wasm.multiply(3, 4)}`;
    try {
        divideResult.textContent = `6 / 2 = ${wasm.divide(6, 2)}`;
        divideError.textContent = '';
    } catch (e) {
        divideResult.textContent = '';
        divideError.textContent = `Error: ${(e as Error).message}`;
    }
}

run();
