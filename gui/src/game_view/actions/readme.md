# Game Actions

Each user interaction triggers a game action. Each game action has a result
To disentangle the call to the game model and updating the UI state `Command` are used.

## Concept
The GUI state (SecCardGameApp) contains a shareable, mutable reference containing a command
to be executed every update cycle. Each component (e.g. buttons) can update this field when it is
interacted with. As soon as a component is triggered the update cycle is triggered as well.
Inside the update cycle `process_command` is called. This command delegates the handling of it
to various methods which in turn mutate the GUI state (e.g. remove a card from the board). Once
this is done, the command is set to none and the update cycle is completed.


## Encapsulation

Inside this `actions` module the Commands and the command_handler are defined.
Also a submodule is defined. This `command_handler` submodule is imported
into the `command_handler.rs` file to be used in the implementation. This allows to
keep all the single handling functions hidden from the App. Outside this `actions` module
only the `process_command` function is visible.
