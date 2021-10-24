mod request;
pub use request::PowRequest1;
pub use request::PowRequest2;

mod response;
pub use response::PowResponse1;
pub use response::PowResponse2;
pub use response::PowResponseErr;

mod puzzle;
pub use puzzle::Md5PuzzleTask;
pub use puzzle::Md5PuzzleSolution;