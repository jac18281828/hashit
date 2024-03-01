use clap::Parser;
use sha256::digest as sha256_digest;
use sha3::{Digest, Sha3_256};

#[derive(Parser, Debug)]
#[command(
    name = "hashit",
    version = "0.5.0",
    about = "hash columns of text using sha3 (default) or sha256"
)]
pub struct Args {
    #[arg(long = "sha256", default_value_t = false)]
    pub sha256: bool,
    #[arg(short = 'v', long = "verbose", default_value_t = false)]
    pub verbose: bool,
}

pub fn parse_args() -> Args {
    Args::parse()
}

pub fn hash_line_sha256(input: &str) -> String {
    sha256_digest(input)
}

pub fn hash_line_sha3(input: &str) -> String {
    let mut hasher = Sha3_256::new();
    hasher.update(input);
    let result = hasher.finalize();
    hex::encode(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_line_sha3() {
        assert_eq!(
            hash_line_sha3("libre"),
            "14ebaef78a0bc7554477314b88029b8eb35687814b3c5e75a65c3ba01b0ebcf6"
        );
    }

    #[test]
    fn test_hash_line_sha256() {
        assert_eq!(
            hash_line_sha256("freedom"),
            "13b1f7ec5beaefc781e43a3b344371cd49923a8a05edd71844b92f56f6a08d38"
        );
    }

    #[test]
    fn test_parse_args() {
        let args = parse_args();
        assert_eq!(args.sha256, false);
        assert_eq!(args.verbose, false);
    }

    #[test]
    fn test_parse_args_sha256() {
        let args = Args::parse_from(&["hashit", "--sha256"]);
        assert_eq!(args.sha256, true);
        assert_eq!(args.verbose, false);
    }

    #[test]
    fn test_parse_args_verbose() {
        let args = Args::parse_from(&["hashit", "--verbose"]);
        assert_eq!(args.sha256, false);
        assert_eq!(args.verbose, true);
    }
}
