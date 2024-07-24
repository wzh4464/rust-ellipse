/**
 * File: /src/util.rs
 * Created Date: Thursday, July 25th 2024
 * Author: Zihan
 * -----
 * Last Modified: Thursday, 25th July 2024 12:37:29 am
 * Modified By: the developer formerly known as Zihan at <wzh4464@gmail.com>
 * -----
 * HISTORY:
 * Date      		By   	Comments
 * ----------		------	---------------------------------------------------------
**/

pub fn save_matrix_to_file(matrix: &Vec<Vec<f64>>, filename: &str) -> Result<(), crate::ElsdcError> {
    use std::fs::File;
    use std::io::Write;

    let mut file = File::create(filename).map_err(crate::ElsdcError::IoError)?;
    for row in matrix {
        for &value in row {
            write!(file, "{:.4} ", value).map_err(crate::ElsdcError::IoError)?;
        }
        writeln!(file).map_err(crate::ElsdcError::IoError)?;
    }
    Ok(())
}
