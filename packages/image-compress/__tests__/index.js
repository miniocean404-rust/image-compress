const { compress } = require("../index")
const path = require("path")
const fs = require("fs")

init()

async function init() {
  compress_png()
}

function compress_jpeg() {
  const image = path.join(process.cwd(), "../../assets/image/jpeg/eye.jpg")

  const res = compress(image, {
    /** 质量, 推荐 60-80. 范围：`1..=100` */
    quality: 60,
    /** 设置图像的渐进模式 */
    progressive: true,
    /** 设置为 false 可以毫无理由地使文件变大 */
    optimizeCoding: true,
    /** 非 0 （1..=100） 它将使用MozJPEG的平滑。 */
    smoothing: 0,
    /** 设置正在写入的 JPEG 的颜色空间，不同于输入的颜色空间 */
    colorSpace: "JCS_YCbCr",
    /** 指定在网格量化期间是否应考虑多次扫描。 */
    trellisMultipass: false,
    /** 设置色度子采样，保留为“None”以使用自动子采样 */
    chromaSubsample: null,
    /** 是否使用特定的量化表。替代质量（quality）设置。 */
    luma: false,
    /** 是否使用特定的量化表的颜色。替代质量（quality）设置。 */
    chroma: false,
    qtable: null,
  })

  fs.writeFileSync("./test.jpg", res.compressedImage)
}

function compress_png() {
  const image = path.join(process.cwd(), "../../assets/image/png/time-icon.png")
  const res = compress(image)
  fs.writeFileSync("./test.jpg", res.compressedImage)
}
