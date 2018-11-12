# Telegram Groups Bot

[![Build Status](https://img.shields.io/travis/coredump-ch/telegram-groups-bot/master.svg)](https://travis-ci.org/coredump-ch/telegram-groups-bot)

**NOTE: This bot is currently unmaintained, since I'm not using it myself
anymore. If someone would be interested in taking over the development, let
me know in #25.**

This bot can be used to manage multiple Telegram topic group chats. In other
chat tools, topic groups are known as "channels", but we tried to avoid that
terminology because of the Telegram Channels that are something like public
groups.

Note that you need a Redis instance on your server.

Docs: https://coredump-ch.github.io/rust-docs/telegram-groups-bot/telegram_groups_bot/


## API

The bot provides the following commands:

- `/help` Show help
- `/list` Show list of available groups
- `/add <name> <url>` Register a new topic group


## Development

Export env variables that control the logging and specify the bot token:

    export RUST_LOG=telegram_groups_bot=debug
    export TELEGRAM_BOT_TOKEN=<your-token>
    export TELEGRAM_BOT_OWNER=<@your-telegram-username>

Other env vars:

- `REDIS_HOST`
- `REDIS_PORT`
- `REDIS_DB`

Then run the code:

    cargo run


## License

MIT License
