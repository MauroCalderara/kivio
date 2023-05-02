# kivio

A Key-Index-Value IO layer

## License

This software is currently not "open source" in the [technical
sense](https://opensource.org/osd/) of the term because there are usage
restrictions even though the source code is publicly available. In particular
this implementation may not be used for commercial purposes as defined in the
[license file](LICENSE.md) and explained in more detail on the license creator's
website, [PolyForm](https://polyformproject.org/).

## Status

- [ ] Shared parts (`kivio-common` crate)
  - [X] `kivio_common::Handle` and `kivio_common::Segment` traits
  - [X] Basic types for vectorized IO (`kivio_common::io_vec`)
  - [X] Virtual memory based implementation of the `Handle` and `Segment` traits (`kivio_common::vmem`)
  - [ ] File descriptor based implementations of the `Handle` and `Segment` traits (`kivio_common::{fd, mmapped_fd}`)
  - [ ] `kivio_common::{Allocator, Store, Backend}` traits
- [ ] Synchronous implementation (`kivio-sync` crate)
- [ ] [Tokio](https://tokio.rs)-based implementation (`kivio-tokio` crate)
- [ ] Zero-copy implementation (`kivio-zcr` crate)

