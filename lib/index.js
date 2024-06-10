var addon = require('../native');

setInterval(() => addon.deleteExpiredItems(), 3600 * 1000);

module.exports = addon;
