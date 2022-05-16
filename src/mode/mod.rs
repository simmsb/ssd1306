//! Display modes.

mod buffered_graphics;
mod terminal;

use core::future::Future;

use crate::{command::AddrMode, rotation::DisplayRotation, size::DisplaySize, Ssd1306};
pub use buffered_graphics::*;
use display_interface::{DisplayError, WriteOnlyDataCommand};
pub use terminal::*;

/// Common functions to all display modes.
pub trait DisplayConfig {
    /// Error.
    type Error;

    type SetRotationFuture<'a>: Future<Output = Result<(), Self::Error>> + 'a
    where
        Self: 'a;

    type InitFuture<'a>: Future<Output = Result<(), Self::Error>> + 'a
    where
        Self: 'a;

    /// Set display rotation.
    fn set_rotation<'a>(&'a mut self, rotation: DisplayRotation) -> Self::SetRotationFuture<'a>;

    /// Initialise and configure the display for the given mode.
    fn init<'a>(&'a mut self) -> Self::InitFuture<'a>;
}

/// A mode with no additional functionality beyond that provided by the base [`Ssd1306`] struct.
#[derive(Debug, Copy, Clone)]
pub struct BasicMode;

impl<DI, SIZE> Ssd1306<DI, SIZE, BasicMode>
where
    DI: WriteOnlyDataCommand,
    SIZE: DisplaySize,
{
    /// Clear the display.
    pub async fn clear(&mut self) -> Result<(), DisplayError> {
        self.set_draw_area((0, 0), self.dimensions()).await?;

        // TODO: If const generics allows this, replace `1024` with computed W x H value for current
        // `SIZE`.
        self.draw(&[0u8; 1024]).await
    }
}

impl<DI, SIZE> DisplayConfig for Ssd1306<DI, SIZE, BasicMode>
where
    DI: WriteOnlyDataCommand,
    SIZE: DisplaySize,
{
    type Error = DisplayError;

    type SetRotationFuture<'a> = impl Future<Output = Result<(), Self::Error>> + 'a
    where
        Self: 'a;

    type InitFuture<'a> = impl Future<Output = Result<(), Self::Error>> + 'a
    where
        Self: 'a;

    /// Set the display rotation.
    fn set_rotation<'a>(&'a mut self, rot: DisplayRotation) -> Self::SetRotationFuture<'a> {
        self.set_rotation(rot)
    }

    /// Initialise in horizontal addressing mode.
    fn init<'a>(&'a mut self) -> Self::InitFuture<'a> {
        self.init_with_addr_mode(AddrMode::Horizontal)
    }
}
