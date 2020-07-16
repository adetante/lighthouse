# Validator Discovery

The `lighthouse vc` command starts a *validator client* instance which connects
to a beacon node performs the duties of a staked validator.

This document provides information on how the validator client discovers the
validators it will act for and how it should obtain their cryptographic
signatures.

Users that create validators using the `lighthouse account` tool in the
standard directories and do not start their `lighthouse vc` with the `--strict`
flag should not need to understand the contents of this document. However,
users with more complex needs may find this document crucial.

## Introducing the `validator_definitions.yml` file

The `validator_definitions.yml` file is located in the `validator_dir`, which
defaults to `~/.lighthouse/validators`. It is a
[YAML](https://en.wikipedia.org/wiki/YAML) encoded file defining exactly which
validators the validator client will (and won't) act for.

### Example

Here's an example file with two validators:

```yaml
---
- enabled: true
  voting_public_key: "0x87a580d31d7bc69069b55f5a01995a610dd391a26dc9e36e81057a17211983a79266800ab8531f21f1083d7d84085007"
  type: local_keystore
  voting_keystore_path: /home/paul/.lighthouse/validators/0x87a580d31d7bc69069b55f5a01995a610dd391a26dc9e36e81057a17211983a79266800ab8531f21f1083d7d84085007/voting-keystore.json
  voting_keystore_password_path: /home/paul/.lighthouse/secrets/0x87a580d31d7bc69069b55f5a01995a610dd391a26dc9e36e81057a17211983a79266800ab8531f21f1083d7d84085007
- enabled: false
  voting_public_key: "0xa5566f9ec3c6e1fdf362634ebec9ef7aceb0e460e5079714808388e5d48f4ae1e12897fed1bea951c17fa389d511e477"
  type: local_keystore
  voting_keystore_path: /home/paul/.lighthouse/validators/0xa5566f9ec3c6e1fdf362634ebec9ef7aceb0e460e5079714808388e5d48f4ae1e12897fed1bea951c17fa389d511e477/voting-keystore.json
  voting_keystore_password_path: /home/paul/.lighthouse/secrets/0xa5566f9ec3c6e1fdf362634ebec9ef7aceb0e460e5079714808388e5d48f4ae1e12897fed1bea951c17fa389d511e477
```
In this example we can see two validators:

- A validator identified by the `0x87a5...` public key which is enabled.
- Another validator identified by the `0x0xa556...` public key which is **not** enabled.

### Fields

Each permitted field of the file is listed below for reference:

- `enabled`: A `true`/`false` indicating if the validator client should consider this
	validator "enabled".
- `voting_public_key`: A validator public key.
- `type`: How the validator signs messages (currently restricted to `local_keystore`).
- `voting_keystore_path`: The path to a EIP-2335 keystore.
- `voting_keystore_password_path`: The path to password for the EIP-2335 keystore.

## Populating the `validator_definitions.yml` file

When validator client starts and the `validator_definitions.yml` file doesn't
exist, a new file will be created. If the `--strict` flag is provided, the new
file will be empty and the validator client will not start any validators. If
the `--strict` flag is **not** provided, an *automatic validator discovery*
routine will start (more on that later). To recap:

- `lighthouse vc`: validators are automatically discovered.
- `lighthouse vc --strict`: validators are **not** automatically discovered.

### Automatic validator discovery

When the `--strict` flag is **not** provided, the validator will search the
`validator_dir` for validators and add any new validators to the
`validator_definitions.yml` with `enabled: true`.

The routine for this search begins in the `validator_dir`, where it obtains a
list of all files in that directory and all sub-directories (i.e., recursive
directory-tree search). For each file named `voting-keystore.json` it creates a
new validator definition by the following process:

1. Set `enabled` to `true`.
1. Set `voting_public_key` to the `pubkey` value from the `voting-keystore.json`.
1. Set `type` to `local_keystore`.
1. Set `voting_keystore_path` to the full path of the discovered keystore.
1. Set `voting_keystore_password_path` to be a file in the `secrets_dir` with a
name identical to the `voting_public_key` value.

### Manual configuration

The automatic validator discovery process works out-of-the-box with validators
that are created using the `lighthouse account validator new` command. The
details of this process are only interesting to those who are using keystores
generated with another tool or have a non-standard requirements.

If you are one of these users, manually edit the `validator_definitions.yml`
file to suit your requirements. If the file is poorly formatted, the validator
client will refuse to start. However, if one or more of the validators have
incorrect paths the validator client will still start and attempt to run
whichever validators it can. As such, care should be taken to observe the logs
when starting the validator client to ensure it has been able to initialize
*all* validators.

## How the `validator_definitions.yml` file is processed

If it validator client were to start using the [example
`validator_definitions.yml` file](#example) would print the follow log,
acknowledging there there are two validators and one is disabled:

```
INFO Initialized validators                  enabled: 1, disabled: 1
```

The validator client will simply ignore the disabled validator. However, for
the active validator, the validator client will:

1. Load an EIP-2335 keystore from the `voting_keystore_path`.
1. Read the contents of the file at `voting_keystore_password_path` and use it
to decrypt the keystore (see prior step) and obtain a BLS keypair.
1. Verify that the decrypted BLS keypair matches the `voting_public_key`.
1.  Create a `voting-keystore.json.lock` file adjacent to the
`voting_keystore_path`, indicating that the voting keystore is in-use and
should not be opened by another process.
1. Proceed to act for that validator, creating blocks and attestations if/when required.

If there is an error during any of these steps (e.g., a file is missing or
corrupt) the validator client will log an error and continue to attempt to
process other validators.

When the validator client exits (or the validator is deactivated) it will
remove the `voting-keystore.json.lock` to indicate that the keystore is free for use again.