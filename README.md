[![Rust](https://github.com/maschmi/seccardgamecli/actions/workflows/rust.yml/badge.svg?branch=main)](https://github.com/maschmi/seccardgamecli/actions/workflows/rust.yml)

# SecCardGame CLI and GUI

At some time in point this will be a simple tool to create cards and decks 
for the [securityDeckGame](https://github.com/maschmi/securityDeckGame).

This CLI supports:

* card creation
* checkout the card repository
* create a deck while specifying how many cards to use in total and how many cards from which type
* a simple GUI to show the drawn cards - you will still need a dice and keep track of resources ;)

## How to play

Important, this game runs locally on your computer. If you want to play
together, share your screen!

Run `seccardgame` with the `init` command. Then run it `game play`. This will ask you about the deck composition
and then opens the UI.

On the left side panel you can enter the resources you will gain every turn and also start a
new turn with the `Draw card` button. Also, your available resources are displayed there. To pay
costs for fixing enter the amount in the entry field and hit `Pay`. If you need to roll a dice
to get fixing costs, enter min and max and press `Roll`.

There is little validation, be gentle. 

## How to add cards

Run `seccardgame` with the `init` command. Then run it `cards create` and follow the on-screen directions. By default,
this creates a card in the game directory, which is a git repository. If you want to contribute your card
please create pull request to this [repo](https://github.com/Security-Card-Game/securityDeckGame).

## What to expect next?

* rework of the UI
* some more help in playing the game
* who knows :)

## Known issues

### Windows

* editing of a card opens the editor but changes are not propagated - so editing is broken

## Why in rust?

Because I want to learn it ;)

## Why not Apple Silicon binary`?

Because it was not working out of the box. Feel free to fix it - otherwise if time allows
it will be done.
