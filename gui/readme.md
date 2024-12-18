# GUI

The gui is implemented using [egui](https://github.com/emilk/egui). In September 2024 a switch to [ICED](https://iced.rs)
was evaluated and not performed. 

## Modules

* [main](./src/readme.md)
* [actions](./src/actions/readme.md)
* [card_window](./src/card_window/readme.md)
* [components](./src/components/readme.md)
* [side_panel](./src/side_panel/readme.md)

## The Command concept

### Why

In the first implementations of the UI each UI action was handled in place. This lead to duplicated code, super special
evaluations in some cases and thanks to its entanglement with UI components a lot of barely testable code. While researching
ICED and its ELM inspired mode, an idea began to shape how to make it better.

### What

Each user action triggers a command. The command is handled at a central place inside the `actions` module. Sound simple,
right? As this is rust it had some interesting side effects.

### How

In a first implementation the UI component called the command handler directly. However, this had some issues when
putting a callback into the card windows. Rust complained a lot about borrowing and lifetimes. In the end another
approach was implemented.

To understand this approach, lets look first at egui and the drawing of the UI. With egui, every time the UI need to be redrawn the `update` method if its App trait is called. Therefore, a button
press triggers a redraw and then the `update` method. Taken together with the fact, that **every** action triggers
a redraw, we either have to handle exactly **one** command or **none** at redraw. This is the reason each update cycle
first calls the `self.process_command()` method, before drawing the UI.

Inside the `self.process_command()` function the command is read from `self.command`, `self` is updated and the `self.command`
is set to `None`. Also, the game action result is evaluated and the correct message to display is set.

To create a callback which sets the command in `self` there are at least two ways. We could work with a `Rc<RefCell<Option<Command>>>` which let us
clone the pointer and safely borrow the underlying value via RefCell. This works (as the commits in this repo can show you). However, there is an
easier way to do this, without the magics of Rc and RefCell. We also can create a closure as mutable and send it borrowed as mutable to the card window
we want to create:

```rust
// stuff
let mut set_command = | cmd: Command | self.command = Some(cmd);
create_window(
    &mut set_command,
    ... // other paremters
);
// stuff
```

For a more detailed explanation look at this [article](https://blog.maschmi.net/EguiComamnds/) - written after implementing
these changes.


## Why not ICED

While ICED looks really promising it had two drawbacks when looking into it in September 2024. First of all,
not much documentation. Secondly, much more sever: No easy support for an application with windows in windows (e.g. the cards)
and also no simple drag & drop support. After spending around a workday I've decided not to move to ICED. //maschmi