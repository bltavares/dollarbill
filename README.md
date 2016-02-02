# dollarbill

Generates a reply based on replies of email threads.

## Dependencies

```
brew install mu
```

## Running

Clone the repo:
```
git clone https://github.com/bltavares/dollarbill
cd dollarbil
```

Generate the dump of the emails from a maildir.
```
bash extract.sh ~/path/to/mail/dir 'What about this subject'
```

Download the binary to generate content and run
```
curl -L https://github.com/bltavares/dollarbill/releases/download/Latests/dollarbill > dollarbill
chmod +x dollarbill
./dollarbill
```

## Building

Requires rust installed:

```
cargo build --release
```

The binary is on `target/release/dollarbill`.

It is possible to control wheter to filter out English phrases with the
environment variables `FILTER_ENGLISH` or `ONLY_ENGLISH` being set.
