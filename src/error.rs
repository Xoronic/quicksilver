#[cfg(all(not(target_arch = "wasm32"), feature = "gamepads"))]
use gilrs;
#[cfg(not(target_arch = "wasm32"))]
use glutin;
use image;
#[cfg(all(not(target_arch = "wasm32"), feature = "rodio"))]
use rodio;
use graphics::{AtlasError, ImageError};
#[cfg(feature = "rusttype")]
use rusttype::Error as FontError;
#[cfg(feature = "saving")]
use saving::SaveError;
#[cfg(feature = "sounds")]
use sound::SoundError;
use std::{fmt, error::Error, io::Error as IOError};

#[derive(Debug)]
/// An error generated by some Quicksilver subsystem
pub enum QuicksilverError {
    /// An error from an image atlas
    AtlasError(AtlasError),
    /// Creating or manipulating the OpenGL Context failed
    ContextError(String),
    /// An error from loading an image
    ImageError(ImageError),
    /// An error from loading a file
    IOError(IOError),
    /// An error when creating a gilrs context
    #[cfg(all(not(target_arch = "wasm32"), feature = "gamepads"))]
    GilrsError(gilrs::Error),
    /// An error from loading a sound
    #[cfg(feature = "sounds")]
    SoundError(SoundError),
    /// A serialize or deserialize error
    #[cfg(feature = "saving")]
    SaveError(SaveError),
    /// There was an error loading a font file
    #[cfg(feature = "rusttype")]
    FontError(FontError),
}

impl fmt::Display for QuicksilverError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl Error for QuicksilverError {
    fn description(&self) -> &str {
        match self {
            QuicksilverError::AtlasError(err) => err.description(),
            QuicksilverError::ContextError(string) => string.as_str(),
            QuicksilverError::ImageError(err) => err.description(),
            QuicksilverError::IOError(err) => err.description(),
            #[cfg(all(not(target_arch = "wasm32"), feature = "gamepads"))]
            QuicksilverError::GilrsError(err) => err.description(),
            #[cfg(feature = "sounds")]
            QuicksilverError::SoundError(err) => err.description(),
            #[cfg(feature = "saving")]
            QuicksilverError::SaveError(err) => err.description(),
            #[cfg(feature = "rusttype")]
            QuicksilverError::FontError(err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&dyn Error> {
        match self {
            QuicksilverError::AtlasError(err) => Some(err),
            QuicksilverError::ContextError(_) => None,
            QuicksilverError::ImageError(err) => Some(err),
            QuicksilverError::IOError(err) => Some(err),
            #[cfg(all(not(target_arch = "wasm32"), feature = "gamepads"))]
            QuicksilverError::GilrsError(err) => Some(err),
            #[cfg(feature = "sounds")]
            QuicksilverError::SoundError(err) => Some(err),
            #[cfg(feature = "saving")]
            QuicksilverError::SaveError(err) => Some(err),
            #[cfg(feature = "rusttype")]
            QuicksilverError::FontError(err) => Some(err),
        }
    }
}

#[doc(hidden)]
impl From<ImageError> for QuicksilverError {
    fn from(err: ImageError) -> QuicksilverError {
        QuicksilverError::ImageError(err)
    }
}

#[doc(hidden)]
#[cfg(feature = "sounds")]
impl From<SoundError> for QuicksilverError {
    fn from(err: SoundError) -> QuicksilverError {
        QuicksilverError::SoundError(err)
    }
}

#[doc(hidden)]
impl From<AtlasError> for QuicksilverError {
    fn from(err: AtlasError) -> QuicksilverError {
        QuicksilverError::AtlasError(err)
    }
}

impl From<IOError> for QuicksilverError {
    fn from(err: IOError) -> QuicksilverError {
        QuicksilverError::IOError(err)
    }
}

#[cfg(feature = "saving")]
impl From<SaveError> for QuicksilverError {
    fn from(err: SaveError) -> QuicksilverError {
        QuicksilverError::SaveError(err)
    }
}

#[doc(hidden)]
impl From<image::ImageError> for QuicksilverError {
    fn from(img: image::ImageError) -> QuicksilverError {
        let image_error: ImageError = img.into();
        image_error.into()
    }
}

#[doc(hidden)]
#[cfg(all(feature = "sounds", not(target_arch = "wasm32")))]
impl From<rodio::decoder::DecoderError> for QuicksilverError {
    fn from(snd: rodio::decoder::DecoderError) -> QuicksilverError {
        let sound_error: SoundError = snd.into();
        sound_error.into()
    }
}

#[doc(hidden)]
#[cfg(feature = "rusttype")]
impl From<FontError> for QuicksilverError {
    fn from(fnt: FontError) -> QuicksilverError {
        QuicksilverError::FontError(fnt)
    }
}

#[cfg(not(target_arch = "wasm32"))]
const ROBUST_ERROR: &str = r#"Internal Quicksilver error: robustness not supported
Please file a bug report at https://github.com/ryanisaacg/quicksilver that includes:
- A minimum reproducing code snippet
- The error message above
"#;

#[doc(hidden)]
#[cfg(not(target_arch = "wasm32"))]
impl From<glutin::CreationError> for QuicksilverError {
    fn from(err: glutin::CreationError) -> QuicksilverError {
        QuicksilverError::ContextError(match err {
            glutin::CreationError::OsError(string) => string,
            glutin::CreationError::NotSupported(err) => err.to_owned(),
            glutin::CreationError::NoBackendAvailable(error) => error.to_string(),
            glutin::CreationError::RobustnessNotSupported => ROBUST_ERROR.to_owned(),
            glutin::CreationError::OpenGlVersionNotSupported => {
                "OpenGL version not supported".to_owned()
            }
            glutin::CreationError::NoAvailablePixelFormat => "No available pixel format".to_owned(),
            glutin::CreationError::PlatformSpecific(string) => string,
            glutin::CreationError::Window(error) => match error {
                glutin::WindowCreationError::OsError(string) => string,
                glutin::WindowCreationError::NotSupported => {
                    "Window creation failed: not supported".to_owned()
                }
            },
        })
    }
}

#[doc(hidden)]
#[cfg(not(target_arch = "wasm32"))]
impl From<glutin::ContextError> for QuicksilverError {
    fn from(err: glutin::ContextError) -> QuicksilverError {
        match err {
            glutin::ContextError::IoError(err) => QuicksilverError::IOError(err),
            glutin::ContextError::OsError(err) => QuicksilverError::ContextError(err),
            glutin::ContextError::ContextLost => {
                QuicksilverError::ContextError("Context lost".to_owned())
            }
        }
    }
}

#[doc(hidden)]
#[cfg(all(not(target_arch = "wasm32"), feature = "gamepads"))]
impl From<gilrs::Error> for QuicksilverError {
    fn from(err: gilrs::Error) -> QuicksilverError {
        QuicksilverError::GilrsError(err)
    }
}
