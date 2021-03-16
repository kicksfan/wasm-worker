/**
 * Extract the exported handler from wasm
 */
const { js_handler } = wasm_bindgen

/**
 * Content returned in the event of a rejected promise from WASM
 * 
 * !! WASM code should attempt to robustly handler all paths, this should not be used in normal operation !!
 */
const server_fault_html = `<!DOCTYPE html>
<body>
The requested resource was not found
</body>`

/**
 * Wraps the server_fault_html in a Response structure with a 404 status code
 * 
 * @param {*} error 
 * @returns Response
 */
function server_fault_response(error) {
  console.error("server_fault_response: ", error)
  return new Response(server_fault_html, {
    status: 404,
    headers: {
      statusText: "not found",
      "content-type": "text/html;charset=UTF-8",
    }
  })
}

/**
 * handleRequest wraps the wasm compilation and dispatch in a compatible
 * way to enable the event.respondWith to successfully complete
 * ! respondWith does not respect inline async functions
 * @param {FetchEvent} event
 * @returns {Response}
 */
 async function handleRequest(event) {
  // Await compile completion of the wasm to ensure the funtion is ready
  await wasm_bindgen(wasm)
  try {
    // Await completion of the wasm call to return the Response object
    return await js_handler(event)
  } catch(error) { // Handles the case where the target WASM returns a rejected promise
    return server_fault_response(error)
  }
}

/**
 * Cloudflare follows the Web Worker pattern of dispatching events
 * into a registered handler.
 * 
 * While the js implementation allows for multiple registered listeners
 * which execute sequentially, we expect the wasm side to handle any
 * middleware patterns
 */
addEventListener('fetch', event => {
  try {
    // WASM handler returns a promise to enable it to run async operations
    const resultPromise = handleRequest(event)
    // respondWith accepts a Promise which enables async operations
    event.respondWith(resultPromise)
  } catch(error) {
    console.error("Error from respondWith: ", error)
  }
})