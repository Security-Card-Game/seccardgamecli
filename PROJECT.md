To be upfront: This was my first project with Rust. Therefore the architecture and the style is a reflection of my journey. There are a lot of rough edges and less tests then I want at the current stage.  I will try to explain my reasoning behind some decisions.

## History

The project started as a simple CLI tool to help to create and edit cards in a machine readable
format, in this case `json`. But as it is sometimes it became clear there must be more. In this case more meant a GUI and also kind of a game core module to model the game rules. At the same time the GUI and the rules should remain as flexible as possible as we are using this project mostly to toy around with mechanics and ideas. And so it evolved from a mere CLI tool to a multi-project source. And it has still a lot of room to evolve.

## Project Structure

### root

in the root folder you will find the root projects `Cargo.toml` and `Cargo.lock`files. As the project evolved the need to split responsibilities into different project became apparent. Therefore, the former `seccardgame`which hold the CLI source code was moved into its own directory and the `game_lib` and `gui` projects were created.

## game_lib

This is the core of this tool. In here the `cards` module holds the different cards and card properties , the interaction with the file system is implemented in the `file`module. The `world` module contains all the interactions and rules needed to create and play a game.

See the game lib [readme](./game_lib/README.md) for details.

## gui

As the name suggests, this project encompasses the complete GUI. It needs the `game_lib` to be built as the `game_lib` is basically the core domain.

See the gui [readme](./gui/readme.md) for details.

## seccardgame

The oldest part of the project and the single entry point to interact with this tool. It holds code to create and modify cards, prints statistics on cards, migrate cards to a new version of the model and also the entry point to start the GUI.