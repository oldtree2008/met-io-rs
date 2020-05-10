#![allow(non_snake_case)]
use super::MetError;
use super::MetReader;
use byteorder::{LittleEndian, ReadBytesExt};
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
        let firstClassHeadLength = firstClassHeadLength.read_i16::<LittleEndian>()?;
        dbg!(firstClassHeadLength);

        let mut secondClassHeadLength = Cursor::new(&d[16..18]);
        let secondClassHeadLength = secondClassHeadLength.read_i16::<LittleEndian>()?;
        dbg!(secondClassHeadLength);

        let mut padDataLength = Cursor::new(&d[18..20]);
        let padDataLength = padDataLength.read_i16::<LittleEndian>()?;
        dbg!(padDataLength);

        let mut recordLength = Cursor::new(&d[20..22]);
        let recordLength = recordLength.read_i16::<LittleEndian>()?;
        dbg!(recordLength);

        let mut headRecordNumber = Cursor::new(&d[22..24]);
        let headRecordNumber = headRecordNumber.read_i16::<LittleEndian>()?;
        dbg!(headRecordNumber);

        let mut dataRecordNumber = Cursor::new(&d[24..26]);
        let dataRecordNumber = dataRecordNumber.read_i16::<LittleEndian>()?;
        dbg!(dataRecordNumber);

        let mut productCategory = Cursor::new(&d[26..28]);
        let productCategory = productCategory.read_i16::<LittleEndian>()?;
        dbg!(productCategory);

        let mut compressMethod = Cursor::new(&d[28..30]);
        let compressMethod = compressMethod.read_i16::<LittleEndian>()?;
        dbg!(compressMethod);

        let formatString = &d[30..38];
        let formatString = String::from_utf8_lossy(&formatString);
        dbg!(formatString);

        let mut qualityFlag = Cursor::new(&d[38..40]);
        let qualityFlag = qualityFlag.read_i16::<LittleEndian>()?;
        dbg!(qualityFlag);

        let satelliteName = &d[40..48];
        let satelliteName = String::from_utf8_lossy(&satelliteName);
        dbg!(satelliteName);

        let mut year = Cursor::new(&d[48..50]);
        let year = year.read_i16::<LittleEndian>()?;
        dbg!(year);

        let mut month = Cursor::new(&d[50..52]);
        let month = month.read_i16::<LittleEndian>()?;
        dbg!(month);

        let mut day = Cursor::new(&d[52..54]);
        let day = day.read_i16::<LittleEndian>()?;
        dbg!(day);

        let mut hour = Cursor::new(&d[54..56]);
        let hour = hour.read_i16::<LittleEndian>()?;
        dbg!(hour);

        let mut minute = Cursor::new(&d[56..58]);
        let minute = minute.read_i16::<LittleEndian>()?;
        dbg!(minute);

        let mut channel = Cursor::new(&d[58..60]);
        let channel = channel.read_i16::<LittleEndian>()?;
        dbg!(channel);

        let mut flagOfProjection = Cursor::new(&d[60..62]);
        let flagOfProjection = flagOfProjection.read_i16::<LittleEndian>()?;
        dbg!(flagOfProjection);

        let mut widthOfImage = Cursor::new(&d[62..64]);
        let widthOfImage = widthOfImage.read_i16::<LittleEndian>()?;
        dbg!(widthOfImage);

        let mut heightOfImage = Cursor::new(&d[64..66]);
        let heightOfImage = heightOfImage.read_i16::<LittleEndian>()?;
        dbg!(heightOfImage);

        let mut scanLineNumberOfImageTopLeft = Cursor::new(&d[66..68]);
        let scanLineNumberOfImageTopLeft =
            scanLineNumberOfImageTopLeft.read_i16::<LittleEndian>()?;
        dbg!(scanLineNumberOfImageTopLeft);

        let mut pixelNumberOfImageTopLeft = Cursor::new(&d[68..70]);
        let pixelNumberOfImageTopLeft = pixelNumberOfImageTopLeft.read_i16::<LittleEndian>()?;
        dbg!(pixelNumberOfImageTopLeft);

        let mut sampleRatio = Cursor::new(&d[70..72]);
        let sampleRatio = sampleRatio.read_i16::<LittleEndian>()?;
        dbg!(sampleRatio);

        let mut latitudeOfNorth = Cursor::new(&d[72..74]);
        let latitudeOfNorth = latitudeOfNorth.read_i16::<LittleEndian>()?;
        dbg!(latitudeOfNorth);

        let latitudeOfNorth = latitudeOfNorth/90;
        dbg!(latitudeOfNorth);

        let mut latitudeOfSouth = Cursor::new(&d[74..76]);
        let latitudeOfSouth = latitudeOfSouth.read_i16::<LittleEndian>()?;
        dbg!(latitudeOfSouth);

        let latitudeOfSouth = latitudeOfSouth/90;
        dbg!(latitudeOfSouth);

        let mut longitudeOfWest = Cursor::new(&d[76..78]);
        let longitudeOfWest = longitudeOfWest.read_i16::<LittleEndian>()?;
        dbg!(longitudeOfWest);

        let longitudeOfWest = longitudeOfWest/-180;
        dbg!(longitudeOfWest);

        let mut longitudeOfEast = Cursor::new(&d[78..80]);
        let longitudeOfEast = longitudeOfEast.read_i16::<LittleEndian>()?;
        dbg!(longitudeOfEast);

        let longitudeOfEast = longitudeOfEast/180;
        dbg!(longitudeOfEast);

        let mut centerLatitudeOfProjection = Cursor::new(&d[80..82]);
        let centerLatitudeOfProjection = centerLatitudeOfProjection.read_i16::<LittleEndian>()?;
        dbg!(centerLatitudeOfProjection);

        let mut centerLongitudeOfProjection = Cursor::new(&d[82..84]);
        let centerLongitudeOfProjection = centerLongitudeOfProjection.read_i16::<LittleEndian>()?;
        dbg!(centerLongitudeOfProjection);

        let mut standardLatitude1 = Cursor::new(&d[84..86]);
        let standardLatitude1 = standardLatitude1.read_i16::<LittleEndian>()?;
        dbg!(standardLatitude1);

        let mut standardLatitude2 = Cursor::new(&d[86..88]);
        let standardLatitude2 = standardLatitude2.read_i16::<LittleEndian>()?;
        dbg!(standardLatitude2);

        let mut horizontalResolution = Cursor::new(&d[88..90]);
        let horizontalResolution = horizontalResolution.read_i16::<LittleEndian>()?;
        dbg!(horizontalResolution);

        let mut verticalResolution = Cursor::new(&d[90..92]);
        let verticalResolution = verticalResolution.read_i16::<LittleEndian>()?;
        dbg!(verticalResolution);

        let mut overlapFlagGeoGrid = Cursor::new(&d[92..94]);
        let overlapFlagGeoGrid = overlapFlagGeoGrid.read_i16::<LittleEndian>()?;
        dbg!(overlapFlagGeoGrid);

        let mut overlapValueGeoGrid = Cursor::new(&d[94..96]);
        let overlapValueGeoGrid = overlapValueGeoGrid.read_i16::<LittleEndian>()?;
        dbg!(overlapValueGeoGrid);

        let mut dataLengthOfColorTable = Cursor::new(&d[96..98]);
        let dataLengthOfColorTable = dataLengthOfColorTable.read_i16::<LittleEndian>()?;
        dbg!(dataLengthOfColorTable);

        let mut dataLengthOfCalibration = Cursor::new(&d[98..100]);
        let dataLengthOfCalibration = dataLengthOfCalibration.read_i16::<LittleEndian>()?;
        dbg!(dataLengthOfCalibration);

        let mut dataLengthOfGeolocation = Cursor::new(&d[100..102]);
        let dataLengthOfGeolocation = dataLengthOfGeolocation.read_i16::<LittleEndian>()?;
        dbg!(dataLengthOfGeolocation);

        let mut reserved = Cursor::new(&d[102..104]);
        let reserved = reserved.read_i16::<LittleEndian>()?;
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
fn test_metreader() {
    let ref d = vec![1u8, 2, 3, 4];
    let r = SatReader::read::<SatReader>(&d);
    assert!(r.is_err());
}

#[test]
fn test_read_satfile() {
    let r = SatReader::read_file::<SatReader>(
        r##"E:\BaiduNetdiskDownload\ANI_IR1_R04_20200509_0900_FY2G.AWX"##,
    );
    assert!(r.is_err())
}
