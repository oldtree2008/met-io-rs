#![allow(non_snake_case)]
use crate::MetError;

use std::fs::File;
use std::io::Cursor;
use std::io::Read;
use std::result::Result;

use binread::prelude::*;
use binread::NullString;
use std::io::SeekFrom;

#[derive(Debug, BinRead)]
#[br(little)]
pub struct Product {
    fName: NullString,
    #[br(seek_before = SeekFrom::Start(12))]
    byteSequence: u16,
    #[br(is_little=(byteSequence==0))]
    firstClassHeadLength: i16,
    #[br(is_little=(byteSequence==0))]
    secondClassHeadLength: i16,
    #[br(is_little=(byteSequence==0))]
    padDataLength: i16,
    #[br(is_little=(byteSequence==0))]
    recordLength: i16,
    #[br(is_little=(byteSequence==0))]
    headRecordNumber: i16,
    #[br(is_little=(byteSequence==0))]
    dataRecordNumber: i16,
    #[br(is_little=(byteSequence==0))]
    productCategory: i16,
    #[br(is_little=(byteSequence==0))]
    compressMethod: i16,
    formatString: NullString,
    #[br(is_little=(byteSequence==0),seek_before = SeekFrom::Start(38))]
    qualityFlag: i16,

    //for product1
    #[br(is_little=(byteSequence==0),if(productCategory==1))]
    pub header1: Option<Header1>,
    #[br(is_little=(byteSequence==0),if(productCategory==1),calc=header1.as_ref().unwrap().widthOfImage  as u64)]
    data1_width: Option<u64>,
    #[br(is_little=(byteSequence==0),if(productCategory==1),calc=header1.as_ref().unwrap().heightOfImage  as u64)]
    data1_height: Option<u64>,
    #[br(is_little=(byteSequence==0),if(productCategory==1),calc=data1_height.unwrap() * data1_width.unwrap())]
    data1_len: Option<u64>,
    #[br(is_little=(byteSequence==0),if(productCategory==1),calc=(headRecordNumber*recordLength) as u64)]
    data1_start: Option<u64>,
    #[br(is_little=(byteSequence==0),if(productCategory==1),seek_before=SeekFrom::Start(data1_start.unwrap()),count=data1_len.unwrap())]
    pub data1: Option<Vec<u8>>,

    //HeadExt
    #[br(calc=(firstClassHeadLength +secondClassHeadLength + padDataLength) as u64)]
    index: u64,
    #[br(seek_before=SeekFrom::Start(index),pad_size_to=64)]
    fileName: NullString,
    #[br(pad_size_to = 8)]
    formateVersion: NullString,
    #[br(pad_size_to = 8)]
    manufacturer: NullString,
    #[br(pad_size_to = 16)]
    satelliteName: NullString,
    #[br(pad_size_to = 8)]
    instrumentName: NullString,
    #[br(pad_size_to = 8)]
    channelCount: NullString,
    #[br(pad_size_to = 8)]
    programVersion: NullString,
    #[br(pad_size_to = 8)]
    copyright: NullString,
    #[br(pad_size_to = 8)]
    sizeFilled: NullString,
}

#[derive(Debug, BinRead)]
#[br(little)]
pub struct Header1 {
    satelliteName: NullString,
    #[br(seek_before = SeekFrom::Start(48))]
    year: u16,
    month: u16,
    day: u16,
    hour: u16,
    minute: u16,
    channel: i16,
    flagOfProjection: i16,
    pub widthOfImage: i16,
    pub heightOfImage: i16,
    scanLineNumberOfImageTopLeft: i16,
    pixelNumberOfImageTopLeft: i16,
    sampleRatio: i16,
    latitudeOfNorth: i16,
    latitudeOfSouth: i16,
    longitudeOfWest: i16,
    longitudeOfEast: i16,
    centerLatitudeOfProjection: i16,
    centerLongitudeOfProjection: i16,
    standardLatitude1: i16,
    standardLatitude2: i16,
    horizontalResolution: i16,
    verticalResolution: i16,
    overlapFlagGeoGrid: i16,
    overlapValueGeoGrid: i16,
    dataLengthOfColorTable: i16,
    dataLengthOfCalibration: i16,
    dataLengthOfGeolocation: i16,
    reserved: i16,
    #[br(if (dataLengthOfColorTable!=0),pad_size_to=256,count=256)]
    colorR: Option<Vec<u8>>,
    #[br(if (dataLengthOfColorTable!=0),pad_size_to=256,count=256)]
    colorG: Option<Vec<u8>>,
    #[br(if (dataLengthOfColorTable!=0),pad_size_to=256,count=256)]
    colorB: Option<Vec<u8>>,
    #[br(if (dataLengthOfCalibration!=0),pad_size_to=256,count=dataLengthOfCalibration/2)]
    calibration: Option<Vec<u16>>,
}

#[derive(Debug)]
pub struct AwxReader(pub Product);

impl AwxReader {
    pub fn read(fname: &str) -> Result<AwxReader, MetError> {
        let mut f = File::open(fname)?;
        let mut d = Vec::new();
        f.read_to_end(&mut d)?;
        let mut reader = Cursor::new(&d);
        dbg!(d.len());
        let product: Product = reader.read_le().unwrap();
        // dbg!(product.productCategory);
        // dbg!(product.manufacturer);
        // dbg!(&product.data1.unwrap()[900..1000]);
        // dbg!(product.header1.unwrap().channel);
        Ok(AwxReader(product))
    }
}

// #[test]
// #[ignore]
// fn test_metreader() {
//     let ref d = vec![1u8, 2, 3, 4];
//     let r = SatReader::read::<SatReader>(&d);
//     assert!(r.is_err());
// }

// #[test]
// fn test_read_satfile() {
//     let r = SatReader::read_file::<SatReader>(
//         r##"D:\BaiduNetdiskDownload\ANI_IR1_R04_20200509_0900_FY2G.AWX"##,
//         // r##"H:\data\yuntu\ERNA19J1.AWX"##,
//     );
//     assert!(r.is_err())
// }
