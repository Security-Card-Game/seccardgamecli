# Changes

## upcoming

* [BUG] does not reduce attack duration directly when it is drawn

## 0.7.1

* [FEATURE] Change label of X on Oopise Cards to "Close"

## 0.7.0

* [FEATURE] adds reputation. Must be increased and decreased manually.

## 0.6.0

* [FEATURE] adds incident impact a coded field for incident cards. This breaks the old card format. Please re-init.

## 0.5.7

* [FEATURE] adds blank evaluation cards

## 0.5.6

* [UPDATE] Updates dependencies, most notably egui
* [REFACTOR] Implements a command pattern for game action commands in the UI, see Readme.md
* [REFACTOR] Moves around some code to give it more structure and also in parts for encapsulation reasons

## 0.5.5

* [BUG] Cards that were not closeable had a close button
* [TYPO] misspelled label for multiplier
* [BUG] resource gain was not always updated correctly
* [FEATURE] grace period for attacks is now configurable via prompt
* [FEATURE] adds defaults to deck creation prompts

## 0.5.4

* [BUG] when using a lucky card and a general multiplier the multiplier was applied twice to the lucky card
* [GAME] major rewrite of the game_lib to make it more testable and hopefully easier to maintain

## 0.5.3
* shows game resources at end of game
* reorganizes control panel code

## 0.5.2

* version updates in github actions
* adds aarch64 artifact for maxOS
  * this uses openssl vendored

## 0.5.1

* [GUI, GAME] adds a fix multiplier which will be used on all oopsies, effects and lucky cards if possible,
this allows to increase the roundly resource gain to more sensible numbers
* [CLI] Limits the targets for incidents and oopsies to pre-defined ones

## 0.5.0

* creates a deck in memory when `game play` is used
* adds card repo to game_lib (file based)
* moves deck creation to game_lib
* [GUI] counts down duration of attacks and closes them if over
* [GUI] automatically rolls a dice when an oopsie is closed 
  * reports the results as message
  * does not close the card when resources are exceeded
* [GUI] automatically applys "next fix" modifiers
* [GUI] allows to select/deselect cards to be used in next fix attemtp

## 0.4.2

* Renames Incidents to Attack cards
* Renames Action to Effect
* Introduces fix cost modifying effects for lucky and event card
* Major refactoring of game model types
* [MIGRATION] Migrates cards from v1 and v2 to v3, also moves incident cards to attacks directory
    * use the `migration version3` to migrate the game directory
    * make sure your cards are at least version1 (version0 to version1 may not be working anymore, sorry)

## 0.4.1

* [GUI] adds duration to incidents/attacks
* [CLI] adds duration to incidents/attacks
* [MIGRATION] Migrates card to v2, where incidents have a duration (and are basically attacks)
    * use the `migration version2` to migrate the game directory
    * make sure your cards are version1

## 0.4.0

* major UI overhaul
  * adds side panel for controls
  * adds resource management
  * adds dice

## 0.3.0

* adds UI to show drawn cards
* displays help if subcommands are needed

## 0.2.1

* removes card to stdout in migrations

## 0.2.0

* adds deck creation functionality
* adds very simple playing function
  * iterates through cards and print them to screen
* [MIGRATION] Migrates card to v1, with the card type tagged
  * use the `migration version1` to migrate the game directory

## 0.1.1-beta

* [BUG] now only counts json files as cards

## 0.1.0-beta

* introduces flag to supply config file
* adds init command
  * clones cards repository
  * creates config file
* respects game path from config file in card creation
* adds simple stats to supply card count
* adds edit card possibility before saving
* [BUG] fixes bug in counting cards

## 0.1.0-alpha

* Adds basic card creator
