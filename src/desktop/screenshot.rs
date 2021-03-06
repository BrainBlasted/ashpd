//! # Examples
//!
//! Taking a screenshot
//!
//! ```no_run
//! use ashpd::desktop::screenshot::{Screenshot, ScreenshotOptions, ScreenshotProxy};
//! use ashpd::{RequestProxy, Response, WindowIdentifier};
//! use zbus::fdo::Result;
//!
//! fn main() -> Result<()> {
//!     let connection = zbus::Connection::new_session()?;
//!     let proxy = ScreenshotProxy::new(&connection)?;
//!     let request_handle = proxy.screenshot(
//!         WindowIdentifier::default(),
//!         ScreenshotOptions::default()
//!             .interactive(true)
//!     )?;
//!
//!     let request = RequestProxy::new(&connection, &request_handle)?;
//!     request.on_response(|response: Response<Screenshot>| {
//!         println!("{}", response.unwrap().uri);
//!     })?;
//!     Ok(())
//! }
//!```
//!
//! Picking a color
//!```no_run
//! use ashpd::desktop::screenshot::{Color, PickColorOptions, ScreenshotProxy};
//! use ashpd::{RequestProxy, Response, WindowIdentifier};
//! use zbus::fdo::Result;
//!
//! fn main() -> Result<()> {
//!     let connection = zbus::Connection::new_session()?;
//!     let proxy = ScreenshotProxy::new(&connection)?;
//!
//!     let request_handle = proxy.pick_color(
//!             WindowIdentifier::default(),
//!             PickColorOptions::default()
//!     )?;
//!
//!     let request = RequestProxy::new(&connection, &request_handle)?;
//!     request.on_response(|response: Response<Color>| {
//!         if let Ok(color) = response {
//!             println!("({}, {}, {})", color.red(), color.green(), color.blue());
//!         }
//!     })?;
//!
//!     Ok(())
//! }
//! ```
use crate::{HandleToken, WindowIdentifier};
use zbus::{dbus_proxy, fdo::Result};
use zvariant::OwnedObjectPath;
use zvariant_derive::{DeserializeDict, SerializeDict, TypeDict};

#[derive(SerializeDict, DeserializeDict, TypeDict, Debug, Default)]
/// Specified options on a screenshot request.
pub struct ScreenshotOptions {
    /// A string that will be used as the last element of the handle.
    pub handle_token: Option<HandleToken>,
    /// Whether the dialog should be modal.
    pub modal: Option<bool>,
    /// Hint whether the dialog should offer customization before taking a screenshot.
    pub interactive: Option<bool>,
}

impl ScreenshotOptions {
    /// Sets the handle token.
    pub fn handle_token(mut self, handle_token: HandleToken) -> Self {
        self.handle_token = Some(handle_token);
        self
    }

    /// Sets whether the dialog should be a modal.
    pub fn modal(mut self, modal: bool) -> Self {
        self.modal = Some(modal);
        self
    }

    /// Sets whether the dialog should offer customization before a screenshot or not.
    pub fn interactive(mut self, interactive: bool) -> Self {
        self.interactive = Some(interactive);
        self
    }
}

#[derive(DeserializeDict, SerializeDict, TypeDict, Debug)]
/// A response to a screenshot request.
pub struct Screenshot {
    /// The screenshot uri.
    pub uri: String,
}

#[derive(SerializeDict, DeserializeDict, TypeDict, Debug, Default)]
/// Specified options on a pick color request.
pub struct PickColorOptions {
    /// A string that will be used as the last element of the handle.
    pub handle_token: Option<HandleToken>,
}

impl PickColorOptions {
    /// Sets the handle token.
    pub fn handle_token(mut self, handle_token: HandleToken) -> Self {
        self.handle_token = Some(handle_token);
        self
    }
}

#[derive(SerializeDict, DeserializeDict, TypeDict, Debug)]
/// A response to a pick color request.
pub struct Color {
    color: ([f64; 3]),
}

impl Color {
    /// Red.
    pub fn red(&self) -> f64 {
        self.color[0]
    }

    /// Green.
    pub fn green(&self) -> f64 {
        self.color[1]
    }

    /// Blue.
    pub fn blue(&self) -> f64 {
        self.color[2]
    }
}

#[cfg(feature = "feature_gtk")]
impl Into<gdk::RGBA> for Color {
    fn into(self) -> gdk::RGBA {
        gdk::RGBA {
            red: self.red(),
            green: self.green(),
            blue: self.blue(),
            alpha: 1_f64,
        }
    }
}

#[dbus_proxy(
    interface = "org.freedesktop.portal.Screenshot",
    default_service = "org.freedesktop.portal.Desktop",
    default_path = "/org/freedesktop/portal/desktop"
)]
/// The interface lets sandboxed applications request a screenshot.
trait Screenshot {
    /// Obtains the color of a single pixel.
    ///
    /// Returns a [`RequestProxy`] object path..
    ///
    /// # Arguments
    ///
    /// * `parent_window` - Identifier for the application window
    /// * `options` - A [`PickColorOptions`]
    ///
    /// [`PickColorOptions`]: ./struct.PickColorOptions.html
    /// [`RequestProxy`]: ../request/struct.RequestProxy.html
    fn pick_color(
        &self,
        parent_window: WindowIdentifier,
        options: PickColorOptions,
    ) -> Result<OwnedObjectPath>;

    /// Takes a screenshot
    ///
    /// Returns a [`RequestProxy`] object path.
    ///
    /// # Arguments
    ///
    /// * `parent_window` - Identifier for the application window
    /// * `options` - A [`ScreenshotOptions`]
    ///
    /// [`ScreenshotOptions`]: ./struct.ScreenshotOptions.html
    /// [`RequestProxy`]: ../request/struct.RequestProxy.html
    fn screenshot(
        &self,
        parent_window: WindowIdentifier,
        options: ScreenshotOptions,
    ) -> Result<OwnedObjectPath>;

    /// version property
    #[dbus_proxy(property, name = "version")]
    fn version(&self) -> Result<u32>;
}
