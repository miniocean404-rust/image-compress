const { getPaths, getImageInfo, getOSDir } = require("./dist/node/index");
const path = require("path");

init();

async function init() {
  //   const image_dir = path.join(process.cwd(), "assets/image/jpg/eye.jpg");
  //   const paths = getPaths("*.{png,webp,gif,jpg,jpeg}", image_dir);
  //   paths.forEach(async (path) => {
  //     let res = await getImageInfo(path, 80);
  //     console.log(res);
  //   });

  setTimeout(() => {
    const info = getOSDir();
    console.log(info);
  }, 1000);
}
