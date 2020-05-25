#![allow(non_snake_case)]
use super::MetError;
use super::MetReader;
use byteorder::{BigEndian, LittleEndian, ReadBytesExt};
use std::io::Cursor;
use std::result::Result;
use std::string::String;

#[derive(Debug)]
struct SatReader;

impl MetReader for SatReader {
    fn read<SatReader>(d: &[u8]) -> Result<SatReader, MetError> {
        dbg!(d.len());

        let f_name = &d[0..12];
        let f_name = String::from_utf8_lossy(&f_name);
        dbg!(f_name);

        let mut byteSequence = Cursor::new(&d[12..14]);
        let byteSequence = byteSequence.read_i16::<LittleEndian>()?;
        dbg!(byteSequence);

        let mut firstClassHeadLength = Cursor::new(&d[14..16]);
        let firstClassHeadLength = if byteSequence == 0 {
            firstClassHeadLength.read_i16::<LittleEndian>()?
        } else {
            firstClassHeadLength.read_i16::<BigEndian>()?
        };
        dbg!(firstClassHeadLength);

        let mut secondClassHeadLength = Cursor::new(&d[16..18]);
        let secondClassHeadLength = if byteSequence == 0 {
            secondClassHeadLength.read_i16::<LittleEndian>()?
        } else {
            secondClassHeadLength.read_i16::<BigEndian>()?
        };
        dbg!(secondClassHeadLength);

        let mut padDataLength = Cursor::new(&d[18..20]);
        let padDataLength = if byteSequence == 0 {
            padDataLength.read_i16::<LittleEndian>()?
        } else {
            padDataLength.read_i16::<BigEndian>()?
        };
        dbg!(padDataLength);

        let mut recordLength = Cursor::new(&d[20..22]);
        let recordLength = if byteSequence == 0 {
            recordLength.read_i16::<LittleEndian>()?
        } else {
            recordLength.read_i16::<BigEndian>()?
        };
        dbg!(recordLength);

        let mut headRecordNumber = Cursor::new(&d[22..24]);
        let headRecordNumber = if byteSequence == 0 {
            headRecordNumber.read_i16::<LittleEndian>()?
        } else {
            headRecordNumber.read_i16::<BigEndian>()?
        };

        dbg!(headRecordNumber);

        let mut dataRecordNumber = Cursor::new(&d[24..26]);
        let dataRecordNumber = if byteSequence == 0 {
            dataRecordNumber.read_i16::<LittleEndian>()?
        } else {
            dataRecordNumber.read_i16::<BigEndian>()?
        };
        dbg!(dataRecordNumber);

        let mut productCategory = Cursor::new(&d[26..28]);
        let productCategory = if byteSequence == 0 {
            productCategory.read_i16::<LittleEndian>()?
        } else {
            productCategory.read_i16::<BigEndian>()?
        };
        dbg!(productCategory);

        let mut compressMethod = Cursor::new(&d[28..30]);
        let compressMethod = if byteSequence == 0 {
            compressMethod.read_i16::<LittleEndian>()?
        } else {
            compressMethod.read_i16::<BigEndian>()?
        };
        dbg!(compressMethod);

        let formatString = &d[30..38];
        let formatString = String::from_utf8_lossy(&formatString);
        dbg!(formatString);

        let mut qualityFlag = Cursor::new(&d[38..40]);
        let qualityFlag = if byteSequence == 0 {
            qualityFlag.read_i16::<LittleEndian>()?
        } else {
            qualityFlag.read_i16::<BigEndian>()?
        };
        dbg!(qualityFlag);

        let satelliteName = &d[40..48];
        let satelliteName = String::from_utf8_lossy(&satelliteName);
        dbg!(satelliteName);

        let mut year = Cursor::new(&d[48..50]);
        let year = if byteSequence == 0 {
            year.read_i16::<LittleEndian>()?
        } else {
            year.read_i16::<BigEndian>()?
        };
        dbg!(year);

        let mut month = Cursor::new(&d[50..52]);
        let month = if byteSequence == 0 {
            month.read_i16::<LittleEndian>()?
        } else {
            month.read_i16::<BigEndian>()?
        };
        dbg!(month);

        let mut day = Cursor::new(&d[52..54]);
        let day = if byteSequence == 0 {
            day.read_i16::<LittleEndian>()?
        } else {
            day.read_i16::<BigEndian>()?
        };
        dbg!(day);

        let mut hour = Cursor::new(&d[54..56]);
        let hour = if byteSequence == 0 {
            hour.read_i16::<LittleEndian>()?
        } else {
            hour.read_i16::<BigEndian>()?
        };
        dbg!(hour);

        let mut minute = Cursor::new(&d[56..58]);
        let minute = if byteSequence == 0 {
            minute.read_i16::<LittleEndian>()?
        } else {
            minute.read_i16::<BigEndian>()?
        };
        dbg!(minute);

        let mut channel = Cursor::new(&d[58..60]);
        let channel = if byteSequence == 0 {
            channel.read_i16::<LittleEndian>()?
        } else {
            channel.read_i16::<BigEndian>()?
        };
        dbg!(channel);

        let mut flagOfProjection = Cursor::new(&d[60..62]);
        let flagOfProjection = if byteSequence == 0 {
            flagOfProjection.read_i16::<LittleEndian>()?
        } else {
            flagOfProjection.read_i16::<BigEndian>()?
        };
        dbg!(flagOfProjection);

        let mut widthOfImage = Cursor::new(&d[62..64]);
        let widthOfImage = if byteSequence == 0 {
            widthOfImage.read_i16::<LittleEndian>()?
        } else {
            widthOfImage.read_i16::<BigEndian>()?
        };
        dbg!(widthOfImage);

        let mut heightOfImage = Cursor::new(&d[64..66]);
        let heightOfImage = if byteSequence == 0 {
            heightOfImage.read_i16::<LittleEndian>()?
        } else {
            heightOfImage.read_i16::<BigEndian>()?
        };
        dbg!(heightOfImage);

        let mut scanLineNumberOfImageTopLeft = Cursor::new(&d[66..68]);
        let scanLineNumberOfImageTopLeft = if byteSequence == 0 {
            scanLineNumberOfImageTopLeft.read_i16::<LittleEndian>()?
        } else {
            scanLineNumberOfImageTopLeft.read_i16::<BigEndian>()?
        };
        dbg!(scanLineNumberOfImageTopLeft);

        let mut pixelNumberOfImageTopLeft = Cursor::new(&d[68..70]);
        let pixelNumberOfImageTopLeft = if byteSequence == 0 {
            pixelNumberOfImageTopLeft.read_i16::<LittleEndian>()?
        } else {
            pixelNumberOfImageTopLeft.read_i16::<BigEndian>()?
        };
        dbg!(pixelNumberOfImageTopLeft);

        let mut sampleRatio = Cursor::new(&d[70..72]);
        let sampleRatio = if byteSequence == 0 {
            sampleRatio.read_i16::<LittleEndian>()?
        } else {
            sampleRatio.read_i16::<BigEndian>()?
        };
        dbg!(sampleRatio);

        let mut latitudeOfNorth = Cursor::new(&d[72..74]);
        let latitudeOfNorth = if byteSequence == 0 {
            latitudeOfNorth.read_i16::<LittleEndian>()?
        } else {
            latitudeOfNorth.read_i16::<BigEndian>()?
        };
        dbg!(latitudeOfNorth);

        let latitudeOfNorth = latitudeOfNorth as f32 / 100.0;
        dbg!(latitudeOfNorth);

        let mut latitudeOfSouth = Cursor::new(&d[74..76]);
        let latitudeOfSouth = if byteSequence == 0 {
            latitudeOfSouth.read_i16::<LittleEndian>()?
        } else {
            latitudeOfSouth.read_i16::<BigEndian>()?
        };
        dbg!(latitudeOfSouth);

        let latitudeOfSouth = latitudeOfSouth as f32 / 100.0;
        dbg!(latitudeOfSouth);

        let mut longitudeOfWest = Cursor::new(&d[76..78]);
        let longitudeOfWest = if byteSequence == 0 {
            longitudeOfWest.read_i16::<LittleEndian>()?
        } else {
            longitudeOfWest.read_i16::<BigEndian>()?
        };
        dbg!(longitudeOfWest);

        let longitudeOfWest = longitudeOfWest as f32 / 100.0;
        dbg!(longitudeOfWest);

        let mut longitudeOfEast = Cursor::new(&d[78..80]);
        let longitudeOfEast = if byteSequence == 0 {
            longitudeOfEast.read_i16::<LittleEndian>()?
        } else {
            longitudeOfEast.read_i16::<BigEndian>()?
        };
        dbg!(longitudeOfEast);

        let longitudeOfEast = longitudeOfEast as f32 / 100.0;
        dbg!(longitudeOfEast);

        let mut centerLatitudeOfProjection = Cursor::new(&d[80..82]);
        let centerLatitudeOfProjection = if byteSequence == 0 {
            centerLatitudeOfProjection.read_i16::<LittleEndian>()?
        } else {
            centerLatitudeOfProjection.read_i16::<BigEndian>()?
        };
        dbg!(centerLatitudeOfProjection);

        let mut centerLongitudeOfProjection = Cursor::new(&d[82..84]);
        let centerLongitudeOfProjection = if byteSequence == 0 {
            centerLongitudeOfProjection.read_i16::<LittleEndian>()?
        } else {
            centerLongitudeOfProjection.read_i16::<BigEndian>()?
        };
        dbg!(centerLongitudeOfProjection);

        let mut standardLatitude1 = Cursor::new(&d[84..86]);
        let standardLatitude1 = if byteSequence == 0 {
            standardLatitude1.read_i16::<LittleEndian>()?
        } else {
            standardLatitude1.read_i16::<BigEndian>()?
        };
        dbg!(standardLatitude1);

        let mut standardLatitude2 = Cursor::new(&d[86..88]);
        let standardLatitude2 = if byteSequence == 0 {
            standardLatitude2.read_i16::<LittleEndian>()?
        } else {
            standardLatitude2.read_i16::<BigEndian>()?
        };
        dbg!(standardLatitude2);

        let mut horizontalResolution = Cursor::new(&d[88..90]);
        let horizontalResolution = if byteSequence == 0 {
            horizontalResolution.read_i16::<LittleEndian>()?
        } else {
            horizontalResolution.read_i16::<BigEndian>()?
        };
        dbg!(horizontalResolution);

        let mut verticalResolution = Cursor::new(&d[90..92]);
        let verticalResolution = if byteSequence == 0 {
            verticalResolution.read_i16::<LittleEndian>()?
        } else {
            verticalResolution.read_i16::<BigEndian>()?
        };
        dbg!(verticalResolution);

        let mut overlapFlagGeoGrid = Cursor::new(&d[92..94]);
        let overlapFlagGeoGrid = if byteSequence == 0 {
            overlapFlagGeoGrid.read_i16::<LittleEndian>()?
        } else {
            overlapFlagGeoGrid.read_i16::<BigEndian>()?
        };
        dbg!(overlapFlagGeoGrid);

        let mut overlapValueGeoGrid = Cursor::new(&d[94..96]);
        let overlapValueGeoGrid = if byteSequence == 0 {
            overlapValueGeoGrid.read_i16::<LittleEndian>()?
        } else {
            overlapValueGeoGrid.read_i16::<BigEndian>()?
        };
        dbg!(overlapValueGeoGrid);

        let mut dataLengthOfColorTable = Cursor::new(&d[96..98]);
        let dataLengthOfColorTable = if byteSequence == 0 {
            dataLengthOfColorTable.read_i16::<LittleEndian>()?
        } else {
            dataLengthOfColorTable.read_i16::<BigEndian>()?
        };
        dbg!(dataLengthOfColorTable);

        let mut dataLengthOfCalibration = Cursor::new(&d[98..100]);
        let dataLengthOfCalibration = if byteSequence == 0 {
            dataLengthOfCalibration.read_i16::<LittleEndian>()?
        } else {
            dataLengthOfCalibration.read_i16::<BigEndian>()?
        };
        dbg!(dataLengthOfCalibration);

        let mut dataLengthOfGeolocation = Cursor::new(&d[100..102]);
        let dataLengthOfGeolocation = if byteSequence == 0 {
            dataLengthOfGeolocation.read_i16::<LittleEndian>()?
        } else {
            dataLengthOfGeolocation.read_i16::<BigEndian>()?
        };
        dbg!(dataLengthOfGeolocation);

        let mut reserved = Cursor::new(&d[102..104]);
        let reserved = if byteSequence == 0 {
            reserved.read_i16::<LittleEndian>()?
        } else {
            reserved.read_i16::<BigEndian>()?
        };
        dbg!(reserved);

        let mut ind = 104;
        let head_rest_len = recordLength * headRecordNumber - ind;
        dbg!(head_rest_len);

        ind += head_rest_len;
        dbg!(ind);

        let data_len = dataRecordNumber as i32 * recordLength as i32;
        dbg!(data_len);

        let value = &d[ind as usize..(ind as i32 + data_len) as usize];
        dbg!(value.len());

        Err(MetError::ReadSatError("demo".to_string()))
    }
}

#[test]
#[ignore]
fn test_metreader() {
    let ref d = vec![1u8, 2, 3, 4];
    let r = SatReader::read::<SatReader>(&d);
    assert!(r.is_err());
}

#[test]
fn test_read_satfile() {
    let r = SatReader::read_file::<SatReader>(
        r##"D:\BaiduNetdiskDownload\ANI_IR1_R04_20200509_0900_FY2G.AWX"##,
    );
    assert!(r.is_err())
}
