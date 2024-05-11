const invoke = window.__TAURI__ !== undefined ? window.__TAURI__.invoke : (e) => {throw new Error(`Tauri not available`);};

/**
 * Tauri binding for the `get_sites` function.
 * @returns {Promise<>}
 */
export async function getSites() {
    return await invoke("get_sites");
}

/**
 * Tauri binding for the `get_departures` function.
 * @param {Object} site; The site to get the departures from
 * @returns {Promise<>}
 */
export async function getDepartures(site) {
    return await invoke("get_departures", {siteId: site});
}

/**
 * Tauri binding for the `get_currency` function.
 * @returns {Promise<>}
 */
export async function getCurrency() {
    return await invoke("get_currency");
}

/**
 * Tauri binding for the `get_weather` function.
 * @returns {Promise<>}
 */
export async function getWeather() {
    return await invoke("get_weather");
}