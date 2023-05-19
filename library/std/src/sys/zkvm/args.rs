use super::os;
use crate::ffi::OsString;
use crate::fmt;
use crate::str::FromStr;

pub struct Args {
    i: usize,
    count: usize,
}

pub fn args() -> Args {
    let count = os::getenv(&OsString::from("ARGC"))
        .map(|var| usize::from_str(var.to_str().unwrap()).unwrap())
        .unwrap_or(0);

    Args { i: 0, count }
}

impl fmt::Debug for Args {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().finish()
    }
}

impl Iterator for Args {
    type Item = OsString;

    fn next(&mut self) -> Option<OsString> {
        if self.i >= self.count {
            None
        } else {
            let mut varname = OsString::from("ARGV_");
            varname.push(&self.i.to_string());

            self.i += 1;

            Some(os::getenv(&varname).unwrap())
        }
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.count, Some(self.count))
    }
}

impl ExactSizeIterator for Args {
    fn len(&self) -> usize {
        self.count
    }
}

impl DoubleEndedIterator for Args {
    fn next_back(&mut self) -> Option<OsString> {
        panic!("unsupported");
    }
}
