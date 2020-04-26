## Gus 2020-04-25

First, I think there are three components to this Ulti project.
(We should have some central place where we keep notes on the whole project...
  but at the same time we should try to keep this simple)
1. This core library,
   implementing single-round game logic
1. An HTTP server (could also be Rust),
   which adds "everything needed for multiplayer"
1. A client (probably HTML + Javascript?),
   which is the webpage that the user actually interacts with


Things that I think we implement here:
- All the logic needed for a single round.
  So, no scorekeeping or notion of player ordering, etc.
  We may want to roll in rounds/scorekeeping/player order.
  I'm not sure!

Things I that I think should go in the server:
- Parsing HTTP requests into Ulti commands
- Handling connected players
- Keeping score
- Logs and `data viz`, deep `data analytics`, leveraging of `big data` to `dominate the market`

Things I think should go in the client:
- All interactivity
- All visuals (ie card images)
