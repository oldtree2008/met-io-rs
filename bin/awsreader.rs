use met_io_rs::AwsReader;
pub fn main() {
    println!("aws reader");

    let fname = "/mnt/e/aws/Z_SURF_C_BEHT_20200910140022_O_AWS_FTM_PQC.BIN1";
    let reader = AwsReader::new(fname);
}
