#[macro_export]
macro_rules! my_ready {
    ($e:expr) => {
        match $e {
            std::task::Poll::Ready(v) => return std::task::Poll::Ready(v),
            std::task::Poll::Pending => std::task::Poll::Pending,
        }
    };
}

#[macro_export]
macro_rules! my_try {
    ($e:expr) => {
        match $e {
            Ok(v) => v,
            Err(e) => return Err(e),
        }
    };
}

#[macro_export]
macro_rules! my_vec {
    ($($e:expr),*) => {
        {
            let mut v = Vec::new();
            $(v.push($e);)*
            v
        }
    };
}
