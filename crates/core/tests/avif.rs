// use std::fs::File;
// mod utils;

// use utils::mock::*;

// // ! 暂未集成
// #[test]
// fn encode_mem_avif() {}

// #[test]
// fn decode() {
//     let file_content = File::open("tests/files/avif/f1t.avif").unwrap();

//     let decoder = AvifDecoder::try_new(file_content).unwrap();

//     let img = Image::from_decoder(decoder).unwrap();

//     assert_eq!(img.dimensions(), (48, 80));
//     assert_eq!(img.colorspace(), ColorSpace::RGBA);
// }

// #[test]
// fn encode_colorspaces_u8() {
//     let mut results = vec![];

//     let encoder = AvifEncoder::new();

//     for colorspace in encoder.supported_colorspaces() {
//         let builder = std::thread::Builder::new().name(format!("{:?}", colorspace));

//         let handler = builder
//             .spawn(move || {
//                 let image = create_test_image_u8(200, 200, *colorspace);

//                 let mut encoder = AvifEncoder::new();

//                 let buf = Cursor::new(vec![]);

//                 let result = encoder.encode(&image, buf);

//                 if result.is_err() {
//                     dbg!(&result);
//                 }

//                 assert!(result.is_ok());
//             })
//             .unwrap();

//         results.push(handler.join())
//     }

//     results.into_iter().collect::<Result<Vec<()>, _>>().unwrap();
// }

// #[test]
// fn encode_colorspaces_u16() {
//     let mut results = vec![];

//     let encoder = AvifEncoder::new();

//     for colorspace in encoder.supported_colorspaces() {
//         let builder = std::thread::Builder::new().name(format!("{:?}", colorspace));

//         let handler = builder
//             .spawn(move || {
//                 let image = create_test_image_u16(200, 200, *colorspace);

//                 let mut encoder = AvifEncoder::new();

//                 let buf = Cursor::new(vec![]);

//                 let result = encoder.encode(&image, buf);

//                 if result.is_err() {
//                     dbg!(&result);
//                 }

//                 assert!(result.is_ok());
//             })
//             .unwrap();

//         results.push(handler.join())
//     }

//     results.into_iter().collect::<Result<Vec<()>, _>>().unwrap();
// }

// #[test]
// fn encode_colorspaces_f32() {
//     let mut results = vec![];

//     let encoder = AvifEncoder::new();

//     for colorspace in encoder.supported_colorspaces() {
//         let builder = std::thread::Builder::new().name(format!("{:?}", colorspace));

//         let handler = builder
//             .spawn(move || {
//                 let image = create_test_image_f32(200, 200, *colorspace);

//                 let mut encoder = AvifEncoder::new();

//                 let buf = Cursor::new(vec![]);

//                 let result = encoder.encode(&image, buf);

//                 if result.is_err() {
//                     dbg!(&result);
//                 }

//                 assert!(result.is_ok());
//             })
//             .unwrap();

//         results.push(handler.join())
//     }

//     results.into_iter().collect::<Result<Vec<()>, _>>().unwrap();
// }

// #[test]
// fn encode_u8() {
//     let image = create_test_image_u8(200, 200, ColorSpace::RGB);
//     let mut encoder = AvifEncoder::new();

//     let buf = Cursor::new(vec![]);

//     let result = encoder.encode(&image, buf);
//     dbg!(&result);

//     assert!(result.is_ok());
// }

// #[test]
// fn encode_u16() {
//     let image = create_test_image_u16(200, 200, ColorSpace::RGB);
//     let mut encoder = AvifEncoder::new();

//     let buf = Cursor::new(vec![]);

//     let result = encoder.encode(&image, buf);
//     dbg!(&result);

//     assert!(result.is_ok());
// }

// #[test]
// fn encode_f32() {
//     let image = create_test_image_f32(200, 200, ColorSpace::RGB);
//     let mut encoder = AvifEncoder::new();

//     let buf = Cursor::new(vec![]);

//     let result = encoder.encode(&image, buf);
//     dbg!(&result);

//     assert!(result.is_ok());
// }

// #[test]
// fn encode_animated() {
//     let image = create_test_image_animated(200, 200, ColorSpace::RGB);
//     let mut encoder = AvifEncoder::new();

//     let buf = Cursor::new(vec![]);

//     let result = encoder.encode(&image, buf);
//     dbg!(&result);

//     assert!(result.is_ok());
// }
