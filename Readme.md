# Reader feed tool

Reader is a proof of concept program designed to revolutionize how you consume
your daily feed from your trusted sources. Say goodbye to distractions and
hello to a streamlined, custom-tailored summary that fits your preferences
perfectly!

## Why Reader?

I wrote Reader because I want to skip the  the chaos of information overload.
With Reader, you can enjoy your daily dose of news and updates without any
unnecessary distractions.

## What Makes Reader Special?

Unlike other feed readers, Reader puts YOU in control. Customize your feed
exactly the way you want it, ensuring that you only see the content that's
most relevant and important to you. No more wasting time on irrelevant
articles or clickbait headlines – with Reader, every moment spent reading is a
moment well spent!

## How Does It Work?

It's simple! Just run the program once a day using GitHub Actions, and let
Reader do the push a summary of the feed in a new markdown file.

## Tooling

### How to open the summary in a easy way?

Fedora
```
alias open-reader="xdg-open https://github.com/eloycoto/reader/blob/main/output/output_$(date '+%Y-%m-%d').md"
```

Apple
```
alias open-reader="open https://github.com/eloycoto/reader/blob/main/output/output_$(date '+%Y-%m-%d').md"
```

### How can I add a source

Reader run uses a config file, where you can define the url for the feed, the
kind, and the category to order the feeds, a config file looks like this:

```
  {
    "kind": "atom",
    "url": "https://blog.rust-lang.org/feed.xml",
    "category": "rust"
  },
  {
    "kind": "feed",
    "url": "https://blogs.gnome.org/uraeus/feed/",
    "category": "linux"
  },
```

Multiple configs can be used, using the `--config` option in the `run` command.
