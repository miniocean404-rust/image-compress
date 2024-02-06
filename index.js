const { getPaths } = require("./dist/node/index");
const path = require("path");

init();

async function init() {
  const image_dir = path.join(process.cwd(), "image/jpg/eye.jpg");

  console.log(getPaths("*.{png,webp,gif,jpg,jpeg}", "image"));
  // const res = await imageCompress.get_image_info(image_dir);
}
