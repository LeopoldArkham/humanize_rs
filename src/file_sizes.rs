static SCALE_DECIMAL: [&'static str; 9] = ["B", "KB", "MB", "GB", "TB", "PB", "EB", "ZB", "YB"];
static SCALE_BINARY: [&'static str; 9] = ["B", "KiB", "MiB", "GiB", "TiB", "PiB", "EiB", "ZiB", "YiB"];

pub mod file_size_opts {

    #[derive(PartialEq)]
    pub enum Standard {
        Decimal,
        Binary,
    }

    pub struct FileSizeOpts {
        pub divider: Standard,
        pub units: Standard,
        pub decimal_places: usize,
        pub decimal_zeroes: usize
    }

    pub const BINARY: FileSizeOpts = FileSizeOpts {
        divider: Standard::Binary,
        units: Standard::Binary,
        decimal_places: 2,
        decimal_zeroes: 0
    };

    pub const DECIMAL: FileSizeOpts = FileSizeOpts {
        divider: Standard::Decimal,
        units: Standard::Decimal,
        decimal_places: 2,
        decimal_zeroes: 0
    };

    pub const CONVENTIONAL: FileSizeOpts = FileSizeOpts {
        divider: Standard::Binary,
        units: Standard::Decimal,
        decimal_places: 2,
        decimal_zeroes: 0
    };
}

use self::file_size_opts::*;

pub fn file_size(_size: u64, opts: FileSizeOpts) -> String {
    let divider = if opts.divider == Standard::Decimal {
        1000.0
    } else {
        1024.0
    };

    let mut size: f64 = _size as f64;
    let mut scale_idx = 0;

    while size >= divider {
        size /= divider;
    	scale_idx += 1;
    }

    let scale = if opts.units == Standard::Decimal {
    	SCALE_DECIMAL[scale_idx]
    } else {
        SCALE_BINARY[scale_idx]
    };

    let places = if size.fract() == 0.0 {
    	opts.decimal_zeroes
    } else {
    	opts.decimal_places
    };

    format!("{:.*} {}", places, size, scale)
}
