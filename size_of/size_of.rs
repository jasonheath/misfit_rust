fn main() {
    println!(
        "std::mem::size_of<ErrCode> = {}",
        std::mem::size_of::<ErrCode>()
    );
    println!(
        "std::mem::size_of<JustAnEmptyEnum> = {}",
        std::mem::size_of::<JustAnEmptyEnum>()
    );
    println!(
        "std::mem::size_of<HealthCheckResult> = {}",
        std::mem::size_of::<HealthCheckResult>()
    );

    println!(
        "std::mem::size_of<&'static str> = {}",
        std::mem::size_of::<&'static str>()
    );
    println!("std::mem::size_of<&str> = {}", std::mem::size_of::<&str>());
    println!(
        "std::mem::size_of<String> = {}",
        std::mem::size_of::<String>()
    );
    //println!( "std::mem::size_of<str> = {}", std::mem::size_of::<str>() ); // size uknown at compile time
}

pub enum JustAnEmptyEnum {}

pub enum ErrCode {
    Unknown = 0,
    GroupNotFound = 1,
    UserNotFound = 2,
    ExecWait = 3,
    NoPid = 4,
    /// Returned when the Launcher receives a protocol message that it
    /// does not know how to handle. This could happen if a newer
    /// Supervisor was trying to communicate with an older Launcher that
    /// didn't understand a newer message, for instance.
    UnknownMessage = 5,
    InvalidVersionNumber = 6,
}

pub enum HealthCheckResult {
    Ok = 0,
    Warning = 1,
    Critical = 2,
    Unknown = 3,
}
