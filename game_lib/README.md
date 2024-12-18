
This is the center of the tool. In here all models, actions and rules are defined. All other projects need this one to function. Think of it as the core domain.

## cards

Inside the `cards` module the `types` of cards are defined and all the `properties` a card can have. Also the `serialization` is defined.
### properties

The cards can have properties, e.g. a duration. Each property is defined in its own file with its own tests (hopefully). The properties may use type from the `world` module inside the `game_lib`. E.g. `Resources` or a `FixModifier`

## serialization

In here tools for serialization are stored. At the moment this is a `StrVisitor` and a `NumberVisitor` to help `serde` serialize and deserialize custom types.

## types

Each card type has its own file encompassing a  `struct` to hold the data and an implementation of needed methods for this type of card. The `card_model.rs` file combines the different card types into a `Card` enum and defines a `CardTrait` and implements it for `Card` and all the types present in the enum.

I'm not particular happy with this enum solution and the `card_model.rs` file. It feels clunky to use. And there may be better ways to do this. This is also one of the oldest parts of the code. I think it would be beneficial to think a bit about how to do this better and then refactor. At same time one should be aware, changing this will affect a large portion of the code base.

## file

In here file system operations are defined. E.g reading and writing single cards (`cards.rs`). Also helper method like counting card types are present in here (`general.rs`).
The `repository.rs` is a special case. In here the `DeckLoader` is defined, which implements the `DeckRepository` trait defined in the `world` module. The `DeckRepository` is the central point when it come to load cards from the file system.

## world

In here all the central parts are defined.  This modules should be tested thoroughly.

Every `Game` and `Board` are immutable objects. Every interaction with them will return a new one with the updates state.

The `action` module contains all the actions a user can perform. However, the actions are not directly exposed to the consumer. All interactions with the game must use the `Game`  object defined in `game.rs`. The `Game` will orchestrate actions and also hold the result of the last user action in the `action_status` field.

The possible action results are typed via the `GameActionResult` enum to make pattern matching possible.

Some of the files have comments for details.  