#[cfg(not(any(
    feature = "runtime-actix",
    feature = "runtime-async-std",
    feature = "runtime-tokio",
)))]
compile_error!(
    "one of 'runtime-actix', 'runtime-async-std' or 'runtime-tokio' features must be enabled"
);

#[cfg(any(
    all(feature = "runtime-actix", feature = "runtime-async-std"),
    all(feature = "runtime-actix", feature = "runtime-tokio"),
    all(feature = "runtime-async-std", feature = "runtime-tokio"),
))]
compile_error!(
    "only one of 'runtime-actix', 'runtime-async-std' or 'runtime-tokio' features can be enabled"
);

pub use native_tls;

//
// Actix *OR* Tokio
//

#[cfg(all(
    not(feature = "runtime-async-std"),
    any(feature = "runtime-tokio", feature = "runtime-actix"),
))]
pub use tokio::{
    self, fs, io::AsyncRead, io::AsyncReadExt, io::AsyncWrite, io::AsyncWriteExt, net::TcpStream,
    task::spawn, task::yield_now, time::delay_for as sleep, time::timeout,
};

#[cfg(all(
    unix,
    not(feature = "runtime-async-std"),
    any(feature = "runtime-tokio", feature = "runtime-actix"),
))]
pub use tokio::net::UnixStream;

//
// tokio
//

#[cfg(all(
    feature = "runtime-tokio",
    not(any(feature = "runtime-actix", feature = "runtime-async-std",))
))]
#[macro_export]
macro_rules! blocking {
    ($($expr:tt)*) => {
        $crate::tokio::task::block_in_place(move || { $($expr)* })
    };
}

#[cfg(all(feature = "tokio-native-tls", not(feature = "async-native-tls")))]
pub use tokio_native_tls::{TlsConnector, TlsStream};

#[cfg(all(feature = "tokio-native-tls", not(feature = "async-native-tls")))]
pub use native_tls::Error as TlsError;

//
// actix
//

#[cfg(feature = "runtime-actix")]
pub use {actix_rt, actix_threadpool};

#[cfg(all(
    feature = "runtime-actix",
    not(any(feature = "runtime-tokio", feature = "runtime-async-std",))
))]
#[macro_export]
macro_rules! blocking {
    ($($expr:tt)*) => {
        $crate::actix_threadpool::run(move || { $($expr)* }).await.map_err(|err| match err {
            $crate::actix_threadpool::BlockingError::Error(e) => e,
            $crate::actix_threadpool::BlockingError::Canceled => panic!("{}", err)
        })
    };
}

//
// async-std
//

#[cfg(all(
    feature = "runtime-async-std",
    not(any(feature = "runtime-actix", feature = "runtime-tokio",))
))]
pub use async_std::{
    self, fs, future::timeout, io::prelude::ReadExt as AsyncReadExt,
    io::prelude::WriteExt as AsyncWriteExt, io::Read as AsyncRead, io::Write as AsyncWrite,
    net::TcpStream, task::sleep, task::spawn, task::yield_now,
};

#[cfg(all(
    feature = "runtime-async-std",
    not(any(feature = "runtime-actix", feature = "runtime-tokio",))
))]
#[macro_export]
macro_rules! blocking {
    ($($expr:tt)*) => {
        $crate::async_std::task::spawn_blocking(move || { $($expr)* }).await
    };
}

#[cfg(all(
    unix,
    feature = "runtime-async-std",
    not(any(feature = "runtime-actix", feature = "runtime-tokio",))
))]
pub use async_std::os::unix::net::UnixStream;

#[cfg(all(feature = "async-native-tls", not(feature = "tokio-native-tls")))]
pub use async_native_tls::{Error as TlsError, TlsConnector, TlsStream};
