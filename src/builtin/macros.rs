macro_rules! arg {
    ($args:ident, $scope:ident, $super_selector:ident, $idx:literal, $name:literal) => {
        match $args.get_positional($idx, $scope, $super_selector) {
            Some(v) => v?.eval()?,
            None => match $args.get_named($name.to_owned(), $scope, $super_selector) {
                Some(v) => v?.eval()?,
                None => return Err(concat!("Missing argument $", $name, ".").into()),
            },
        };
    };
    ($args:ident, $scope:ident, $super_selector:ident, $idx:literal, $name:literal=$default:expr) => {
        match $args.get_positional($idx, $scope, $super_selector) {
            Some(v) => v?.eval()?,
            None => match $args.get_named($name.to_owned(), $scope, $super_selector) {
                Some(v) => v?.eval()?,
                None => $default,
            },
        };
    };
}

macro_rules! named_arg {
    ($args:ident, $scope:ident, $super_selector:ident, $name:literal) => {
        match $args.get_named($name.to_owned(), $scope, $super_selector) {
            Some(v) => v?.eval()?,
            None => return Err(concat!("Missing argument $", $name, ".").into()),
        };
    };
    ($args:ident, $scope:ident, $super_selector:ident, $name:literal=$default:expr) => {
        match $args.get_named($name.to_owned(), $scope, $super_selector) {
            Some(v) => v?.eval()?,
            None => $default,
        };
    };
}

macro_rules! max_args {
    ($args:ident, $count:literal) => {
        if $args.len() > $count {
            let mut err = String::with_capacity(50);
            err.push_str(&format!("Only {} argument", $count));
            if $count != 1 {
                err.push('s');
            }
            err.push_str(&format!(" allowed, but {} ", $args.len()));
            if $args.len() == 1 {
                err.push_str("was passed.")
            } else {
                err.push_str("were passed.")
            }
            return Err(err.into());
        }
    };
}

macro_rules! bound {
    ($name:literal, $arg:ident, $unit:ident, $low:literal, $high:literal) => {
        if $arg > Number::from($high) || $arg < Number::from($low) {
            return Err(format!(
                "${}: Expected {}{} to be within {}{} and {}{}.",
                $name, $arg, $unit, $low, $unit, $high, $unit,
            )
            .into());
        } else {
            $arg
        }
    };
    // HACK: we accept `$low` as an ident here in order to work around
    // a bug in the nightly compiler.
    // https://github.com/rust-lang/rust/issues/70050
    ($name:literal, $arg:ident, $unit:ident, $low:ident, $high:literal) => {
        if $arg > Number::from($high) || $arg < Number::from($low) {
            return Err(format!(
                "${}: Expected {}{} to be within {}{} and {}{}.",
                $name, $arg, $unit, $low, $unit, $high, $unit,
            )
            .into());
        } else {
            $arg
        }
    };
    ($name:literal, $arg:ident, $unit:path, $low:literal, $high:literal) => {
        if $arg > Number::from($high) || $arg < Number::from($low) {
            return Err(format!(
                "${}: Expected {}{} to be within {}{} and {}{}.",
                $name, $arg, $unit, $low, $unit, $high, $unit,
            )
            .into());
        } else {
            $arg
        }
    };
    // HACK: we accept `$low` as an ident here in order to work around
    // a bug in the nightly compiler.
    // https://github.com/rust-lang/rust/issues/70050
    ($name:literal, $arg:ident, $unit:path, $low:ident, $high:literal) => {
        if $arg > Number::from($high) || $arg < Number::from($low) {
            return Err(format!(
                "${}: Expected {}{} to be within {}{} and {}{}.",
                $name, $arg, $unit, $low, $unit, $high, $unit,
            )
            .into());
        } else {
            $arg
        }
    };
}
