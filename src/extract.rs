pub struct Data {}

impl Data {
    pub fn netcdf(
        name: &str,
        variable: &str,
        _lon: i32,
        _lat: i32,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let file = netcdf::open(name)?;

        let var = &file.variable(variable).expect("Could not find variable.");

        let res = match var.value::<i32, _>([0, 0, 0]) {
            Ok(data_i32) => data_i32,
            Err(e) => panic!("{}", e),
        };

        Ok(res)
    }
}
