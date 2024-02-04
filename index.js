const imageCompress = require("./dist/node/index.win32-x64-msvc.node");
const path = require("path");

init();

async function init() {
  const image_dir = path.join(process.cwd(), "image/jpg/eye.jpg");

  console.log(new imageCompress.ImageCompression());
  const res = await imageCompress.get_image_info(image_dir);
}
