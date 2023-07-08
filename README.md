# IMBak

A command line tool for backing up IMAP mailboxes written in Rust

## Usage

On running the command you will be prompted to input a username (email) and password. In non-interactive environments you can set the `IMAP_USERNAME` and `IMAP_PASSWORD` environment variables.

### Example 1

Download all emails from the default mailbox (INBOX) to the current working directory using the default batch size (5).

```sh
imbak backup \
  --domain imap.example.com \
  --port 993 \
  --from 0 \
  --count 100
```

### Example 2

All options.

```sh
imbak backup \
  --domain imap.example.com \
  --port 993 \
  --mailbox INBOX \
  --batch-size 5 \
  --output-directory . \
  --from 0 \
  --count 100
```
