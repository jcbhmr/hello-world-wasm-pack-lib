const currentMessageUndefined = Symbol('currentMessageUndefined')
/** @type {symbol | string | any} */
let currentMessage = currentMessageUndefined
/** @param {string} message */
export function hook(message) {
  const error = new WebAssembly.RuntimeError(message)

  currentMessage = getWASMPackMessage(error)
  queueMicrotask(() => currentMessage = currentMessageUndefined)

  // If 'consoleError' is set, that means we've already overwritten it. Don't do
  // so again.
  if (!consoleError) {
    consoleError = console.error;
    console.error = newConsoleError
    // Skip the reset if 'console.error()' has changed since we changed it. We
    // don't want to undo someone else's overwriting but we still want to try to
    // reset the 'console.error()' back to normal so that logging locations are
    // as good as possible.
    queueMicrotask(() => {
      if (console.error === newConsoleError) {
        console.error = consoleError
        consoleError = undefined
      }
    })
  }

  throw error;
}

/** @type {typeof console.error | undefined} */
let consoleError;
function newConsoleError(...args) {
  if (args.length === 2 && args[0] === "wasm-bindgen: imported JS function that was not marked as `catch` threw an error:" && args[1] === currentMessage) {
    return
  } else {
    return consoleError.apply(this, args)
  }
}

function getWASMPackMessage(e) {
  try {
      return e instanceof Error ? `${e.message}\n\nStack:\n${e.stack}` : e.toString();
  } catch {
      return "<failed to stringify thrown value>";
  }
}
