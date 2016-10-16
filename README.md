This is a streaming drain, similiar to `slog-stream` but designed to use
`fmt::Write` instead of `io::Write`, therefore be `no_std` compatible.

It's a work in progress, ATM. Please contact `slog` authors if you'd like to
use it.
