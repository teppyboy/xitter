// Fake that we have a touchscreen (why)
console.log("Setting up fake touch screen");
const oMatchMedia = window.matchMedia;
function fMatchMedia(query) {
    if (query.includes("pointer") && query.includes("coarse")) {
        return {
            matches: true,
            media: query,
            onchange: null,
            addEventListener: () => {},
            removeEventListener: () => {},
        }
    }
    return oMatchMedia(query);
}
window.matchMedia = fMatchMedia;
