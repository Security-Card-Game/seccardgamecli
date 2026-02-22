[![Rust](https://github.com/maschmi/seccardgamecli/actions/workflows/rust.yml/badge.svg?branch=main)](https://github.com/maschmi/seccardgamecli/actions/workflows/rust.yml)

# SecCardGame CLI and GUI

This Tool supports:

* card creation
* checkout the card repository
* create a deck while specifying how many cards to use in total and how many cards from which type (via CLI and UI)
* a UI to play the game, enforcing some rules but not all - after all we experiment with the mechanics

See [Project](PROJECT.md) form some details on structure and history.

## How to play

Important, this game runs locally on your computer. If you want to play
together, share your screen!

Run `seccardgame` with the `init` command by executing `seccardgame init`. This will create a config file and download the cards from the other repository. Then start a game with `seccradgame game ui`. This opens the UI in a start screen where you can edit the deck composition, define parameters to start games with or select from pre-defined scenarios. You can find binaries under releases. 

Be aware, this is a CLI and need to be run in a terminal. It will create a config file and clone the cards into a folder.

We alreday have an [open issue](https://github.com/Security-Card-Game/seccardgamecli/issues/61) to remove the init step which may open the venue to have a binary which opens the UI direclty when double clicked. Oh, and if you happen to run into overflow errors when creating a game, pick less cards. We have also an [open issue](https://github.com/Security-Card-Game/seccardgamecli/issues/60) to fix this.

On the left side panel you can enter the resources you will gain every turn and also start a
new turn with the `Draw card` button. Also, your available resources are displayed there. To pay
costs for fixing enter the amount in the entry field and hit `Pay`. If you need to roll a dice
to get fixing costs, enter min and max and press `Roll`.

There is little validation, be gentle. 

# Resources

* [Why the game is created and how it all began](https://blog.maschmi.net/seccardgame/)
* [How to play the game in its very early stage and how to contribute cards](https://blog.maschmi.net/seccardgame-play/)
* [A simple UI and CLI for playing and creating card](https://github.com/Security-Card-Game/seccardgamecli)
* [A repo full of cards](https://github.com/Security-Card-Game/securityDeckGame)

## How to add cards

Run `seccardgame` with the `init` command. Then run it `cards create` and follow the on-screen directions. By default,
this creates a card in the game directory, which is a git repository. If you want to contribute your card
please create pull request to this [repo](https://github.com/Security-Card-Game/securityDeckGame).

## What to expect next?

* see issues

## Known issues

### Windows

* editing of a card opens the editor but changes are not propagated - so editing is broken

## Why in rust?

Because I want to learn it ;)
