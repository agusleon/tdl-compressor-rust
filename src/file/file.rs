use std::fs::File;
use std::io::Read;

use crate::utils::error::CompressionError;



/// The function read_arguments reads the arguments passed from command and calls the valid_arguments function
/// If arguments are valid, read_arguments removes the first argument which is the program name and returns
/// a vector of Strings containing the different torrent paths. If the arguments are not valid, it returns an error.
// pub fn read_arguments(args: Vec<String>) -> Result<Vec<String>, TorrentError> {
//     match valid_arguments(&args) {
//         Ok(()) => {
//             let mut args_mut = args;
//             let _program = args_mut.remove(0);

//             Ok(args_mut)
//         }
//         Err(e) => Err(e),
//     }
// }

/// The function valid_arguments evaluates if the number of arguments received, matches the valid number of arguments to keep running the diff algorithm.
/// If the number is less than the valid one, it wraps the error and returns it with a message, if the number is equal, it wraps the vector into an Ok value.
///   The valid number of arguments in this crate is 2 (two).
// fn valid_arguments(args: &[String]) -> Result<(), TorrentError> {
//     let args_min = 2;

//     match args.len().cmp(&args_min) {
//         Ordering::Equal => Ok(()),
//         Ordering::Greater => Ok(()),
//         Ordering::Less => Err(TorrentError::NotEnoughArguments),
//     }
// }

/// The function read_file opens and reads the torrent file as a vector of bytes and returns it if there is no error.
/// Is there is one error, the corresponding type will be returned according to the open and read_to_end methods from std::fs::File and
/// std::io::Read
pub fn read_file(path: &str) -> Result<Vec<u8>, CompressionError> {
    let mut file = File::open(path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    Ok(buffer)
}