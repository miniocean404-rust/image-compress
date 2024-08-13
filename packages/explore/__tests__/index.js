const { getOsFileManagerPath, initCustomTraceSubscriber } = require("../index")
const path = require("path")

init()

async function init() {
  initCustomTraceSubscriber("./logs")
  setTimeout(() => {
    const info = getOsFileManagerPath()
    console.log(info)
  }, 1000)
}
