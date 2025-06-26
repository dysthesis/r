# r - a command-line RSS feed fetcher

Given a list of newline-separated RSS or Atom feed URLs, `r` fetches the full-text content for them and returns the articles as a JSON array, making it easily scriptable with `jq`.

## To do

- [ ] Fix contents getting mismatched with the wrong title, probably because of some weird async issue.
- [ ] Consider extracting the web page to Markdown fetching logic to a separate binary.
