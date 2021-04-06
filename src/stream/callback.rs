use std::future::Future;
use std::path::PathBuf;
use std::pin::Pin;

use futures::FutureExt;
use tokio::sync::mpsc;
use tokio::sync::mpsc::{Receiver, Sender};

// maybe:
//  pub type OnProgress = Box<dyn Fn(&dyn Any, &[u8], u32)>;
//  pub type OnComplete = Box<dyn Fn(&dyn Any, Option<PathBuf>)>;
/// Arguments given either to a on_progress callback or on_progress receiver
#[cfg(any(feature = "callback", doc))]
#[doc(cfg(feature = "callback"))]
#[derive(Clone, derivative::Derivative)]
#[derivative(Debug)]
pub struct CallbackArguments {
    pub current_chunk: usize,
}

// TODO: Add Debug
/// Type to process on_progress
#[cfg(any(feature = "callback", doc))]
#[doc(cfg(feature = "callback"))]
pub enum OnProgressType {
    /// Box containing a closure to execute on progress
    Closure(Box<dyn Fn(CallbackArguments)>),
    /// Box containing a async closure to execute on progress
    AsyncClosure(Box<dyn Fn(CallbackArguments) -> Pin<Box<dyn Future<Output = ()>>>>),
    /// Channel to send a message to on progress,
    /// bool indicates whether or not to cancel on a closed channel
    Channel(Sender<CallbackArguments>, bool),
    /// Box containing a closure to execute on progress
    /// Will get executed for every MB downloaded
    SlowClosure(Box<dyn Fn(CallbackArguments)>),
    /// Box containing a async closure to execute on progress
    /// Will get executed for every MB downloaded
    SlowAsyncClosure(Box<dyn Fn(CallbackArguments) -> Pin<Box<dyn Future<Output = ()>>>>),
    /// Channel to send a message to on progress,
    /// bool indicates whether or not to cancel on a closed channel
    /// Will get executed for every MB downloaded
    SlowChannel(Sender<CallbackArguments>, bool),
    None,
}

#[cfg(any(feature = "callback", doc))]
#[doc(cfg(feature = "callback"))]
impl Default for OnProgressType {
    fn default() -> Self {
        OnProgressType::None
    }
}

// TODO: Add Debug
/// Type to process on_progress
#[cfg(any(feature = "callback", doc))]
#[doc(cfg(feature = "callback"))]
pub enum OnCompleteType {
    /// Box containing a closure to execute on complete
    Closure(Box<dyn Fn(Option<PathBuf>)>),
    // fixme: Find a way to store async closures
    /// Box containing a async closure to execute on complete
    AsyncClosure(Box<dyn Fn(Option<PathBuf>) -> Pin<Box<dyn Future<Output = ()>>>>),
    None,
}

#[cfg(any(feature = "callback", doc))]
#[doc(cfg(feature = "callback"))]
impl Default for OnCompleteType {
    fn default() -> Self {
        OnCompleteType::None
    }
}

// TODO: Add Debug
/// Methods and streams to process either on_progress or on_complete
#[cfg(any(feature = "callback", doc))]
#[doc(cfg(feature = "callback"))]
pub struct Callback {
    pub on_progress: OnProgressType,
    pub on_complete: OnCompleteType,
    pub(crate) internal_sender: Sender<usize>,
    pub(crate) internal_receiver: Option<Receiver<usize>>,
}

#[cfg(any(feature = "callback", doc))]
#[doc(cfg(feature = "callback"))]
impl Callback {
    /// Create a new callback struct without actual callbacks
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel(100);
        Callback {
            on_progress: OnProgressType::None,
            on_complete: OnCompleteType::None,
            internal_sender: tx,
            internal_receiver: Some(rx)
        }
    }

    /// Attach a closure to be executed on progress
    ///
    /// ### Warning:
    /// This closure gets executed quite often, once every ~10kB progress.
    /// If it's too slow, some on_progress events will be dropped.
    /// If you are looking fore something that will be executed more seldom, look for
    /// [Callback::connect_on_progress_closure_slow](crate::stream::Callback::connect_on_progress_closure_slow)
    #[inline]
    pub fn connect_on_progress_closure(mut self, closure: impl Fn(CallbackArguments) + 'static) -> Self {
        self.on_progress = OnProgressType::Closure(Box::new(closure));
        self
    }

    /// Attach a closure to be executed on progress. This closure will be executed
    /// more seldom, around once for every MB downloaded.
    #[inline]
    pub fn connect_on_progress_closure_slow(mut self, closure: impl Fn(CallbackArguments) + 'static) -> Self {
        self.on_progress = OnProgressType::SlowClosure(Box::new(closure));
        self
    }

    /// Attach a async closure to be executed on progress
    ///
    /// ### Warning:
    /// This closure gets executed quite often, once every ~10kB progress.
    /// If it's too slow, some on_progress events will be dropped.
    /// If you are looking fore something that will be executed more seldom, look for
    /// [Callback::connect_on_progress_closure_async_slow](crate::stream::Callback::connect_on_progress_closure_async_slow)
    #[inline]
    pub fn connect_on_progress_closure_async<Fut: Future<Output = ()> + Send + 'static, F: Fn(CallbackArguments) -> Fut + 'static>(mut self, closure: F) -> Self {
        self.on_progress = OnProgressType::AsyncClosure(box move |arg| closure(arg).boxed());
        self
    }

    /// Attach a async closure to be executed on progress. This closure will be executed
    /// more seldom, around once for every MB downloaded.
    #[inline]
    pub fn connect_on_progress_closure_async_slow<Fut: Future<Output = ()> + Send + 'static, F: Fn(CallbackArguments) -> Fut + 'static + Sync + Send>(mut self, closure: F) -> Self {
        self.on_progress = OnProgressType::SlowAsyncClosure(box move |arg| closure(arg).boxed());
        self
    }

    /// Attach a bounded sender that receives messages on progress
    /// cancel_or_close indicates whether or not to cancel the download, if the receiver is closed
    ///
    /// ### Warning:
    /// This sender gets messages quite often, once every ~10kB progress.
    /// If it's too slow, some on_progress events will be dropped.
    #[inline]
    pub fn connect_on_progress_sender(
        mut self,
        sender: Sender<CallbackArguments>,
        cancel_on_close: bool
    ) -> Self {
        self.on_progress = OnProgressType::Channel(sender, cancel_on_close);
        self
    }

    /// Attach a bounded sender that receives messages on progress
    /// cancel_or_close indicates whether or not to cancel the download, if the receiver is closed
    ///
    /// This closure will be executed more seldom, around once for every MB downloaded.
    #[inline]
    pub fn connect_on_progress_sender_slow(
        mut self,
        sender: Sender<CallbackArguments>,
        cancel_on_close: bool
    ) -> Self {
        self.on_progress = OnProgressType::SlowChannel(sender, cancel_on_close);
        self
    }

    /// Attach a closure to be executed on complete
    #[inline]
    pub fn connect_on_complete_closure(mut self, closure: impl Fn(Option<PathBuf>) + 'static) -> Self {
        self.on_complete = OnCompleteType::Closure(Box::new(closure));
        self
    }

    /// Attach a async closure to be executed on complete
    #[inline]
    pub fn connect_on_complete_closure_async<Fut: Future<Output = ()> + Send + 'static, F: Fn(Option<PathBuf>) -> Fut + 'static>(mut self, closure: F) -> Self {
        self.on_complete = OnCompleteType::AsyncClosure(box move |arg| closure(arg).boxed());
        self
    }
}
