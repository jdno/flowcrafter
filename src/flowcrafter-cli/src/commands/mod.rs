use anyhow::Result;

pub use self::create::Create;
pub use self::init::Init;

mod create;
mod init;

pub trait Command {
    fn run(&self) -> Result<()>;
}
