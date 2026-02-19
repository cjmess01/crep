pub enum SearchMode {
    LineMajor,
    BlockMajor,
}

pub struct Config {
    pub mode: SearchMode,
    pub quite: bool,
    pub recurse: bool,
    pub recurse_depth: u16,
}
// -r = recursive searching
// -c = just print count instead of matches
// -n = use line major as opposed to boyer moore
// --recurse-depth = how far in the directory to go down
