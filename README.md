# Oubliette

Inspired by @killercup's [Static FileZ][static-filez] this is a library intended
for producing compressed and archived versions of static file trees, with
different tradeoffs than most general purpose archive formats.

See [_Static FileZ: What and Why_][what-and-why] for some of the background
constraints. The major deviations from that design are around flexibility;
_Oubliette_ is intended to support multiple compression algorithms, different
storage backends (including `async` ones such as doing `Range` requests to S3)
and be able to configure the builder to tradeoff between total archive size and
extraneous data needing to be discarded to read a specific file (if you can get
the archive small enough, then instead of doing a remote request to access data
it may be cheaper to do extra work while reading from it locally).

[static-filez]: https://github.com/killercup/static-filez)
[what-and-why]: https://github.com/killercup/static-filez#what-and-why
