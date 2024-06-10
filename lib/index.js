var addon = require('../native');

addon.setItem("abd", "hef", 10);

setTimeout(() => {
    console.log(addon.getItem("abd"));
}, 11 * 1000);

console.log(addon.getItem("abd"));
