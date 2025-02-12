### 1.2.0

- Add support for Elasticsearch authentication (`logs.elastic.auth` config section)
- **FIX:** Objects belonging to group `0` of LRU-SP cache were never fully dropped from the state because of a bug in `StateCacheService.dropById`

### 1.1.0 (Ephesus release)

- `dev:batchUpload` command was removed as it relied on no longer available `sudo` extrinsic
- Any other references to `sudo` were removed (not affecting any functionality besides the removed `dev:batchUpload` command)
- Updated `@joystream/types` and `@joystream/metadata-protobuf` dependencies
- Minor fix for invalid elasticsearch log metadata format in `verbose` mode (https://github.com/Joystream/joystream/issues/3877)

### 1.0.1

- `@joystream/types` dependency version bump for consistency with mainnet release versioning

### 1.0.0 (Mainnet release)

- Adds connected query-node's state information to the `/status` endpoint.
- Bumps the major version of the node since it's now Joystream mainnet compatible.

### 0.2.0 (Carthage release)

- Support for mime-type provided as part of `SubtitleMetadata` (video subtitles). It is now treated as fallback `mime-type` as long as it's a valid `text/*` type and the `file-type` package fails to detect any type from the file signature (magic number),
- **Security:** Sensitive information (like private keys) is now hidden in the node's logs,
- Requesting an asset which has not been accepted by any storage provider yet (`isAccepted == false`) now results in `404: Data object has not been uploaded yet` (previously `500: Failed to download object {id} from any availablable storage provider`),
- Fixed links in CLI docs,
- Autogenerated API client library (`@joystream/distributor-node-client`) is now part of the distributor node codebase.

### 0.1.2

- Fix cache `cache-control: max-age` header for objects served from the filesystem ([`send`](https://www.npmjs.com/package/send) library requires `max-age` to be provided as miliseconds).

### 0.1.1

- Replace `read-chunk` dependency w/ custom `readFileChunk` implementation in order to fix the issue w/ data object `mime-type` detection (https://github.com/Joystream/joystream/pull/3723),
- Add `version` property to `/status` endpoint result.
