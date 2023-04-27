### TCP echo server.

A TCP Echo Server is a simple echo server written in rust that works using the TCP protocol. 
The server listens to connection requests from a client and writes back the data on on the connection.

## How to use it.
Start the TCP server in a shell with:

```rust
rustc tcp-echo-server.rs && ./tcp-echo-server
```

start the client with in a new shell:
```rust
rustc tcp-client.rs && ./tcp-client
```

You can then enter text on the client and it will be echoed back...🙀



                            $$$$      $$           
                          4$$$$     ^$$$$$.
                          $$$$        $$$$$.
                         .$$$$         $$$$$
                      z$$$$$$$$       $$$$$$b
                     $$$$$$""          *$$$$$$.
                     $$$$$                $$$$$r
            \        $$$*     dc    ..    '$$$$b
            4       $$$F      $F    $%     $$$$$       4
            'r     4$$$L  .e$$$$$$$$$$bc    $$$$r      $
             $.    '$$$$z$$$$$$$$$$$$$$$$$..$$$$$     z$
             $$$c   $$$$$$$$$$$$$$$$$$$$$$$$$$$$F  .d$$*
               "$$$. $$$$$$$$$$$$$$$$$$$$$$$$$$P z$$$"
                  "$$b$$$$$$$$$$$$$$$$$$$$$$$$$d$$*
                     "$$$$$$$$$$$🦀$$$$$$$$$$$$$P"
         ^         .d$$$$$$$$$$$$$$$$$$$$$$$$"
          "e   .e$$$$*"$$$$$$$$$$$$$$$$$$$$$$$$$$e..  e"
           *$$$$P"     ^$$$$$$$$$$$$$$$$$$$$P ""**$$$$
            *"          $$$$$$$$$$$$$$$$$$$P
                      .$$"*$$$$$$$$$$$$P**$$e
                     z$P   J$$$$$$$$$$$e.  "$$c     .
                    d$" .$$$$$$*""""*$$$$$F  "$$. .P
             ^%e.  $$   4$$$"          ^$$     "$$"
                "*$*     "$b           $$"       ^
                           $r          $
                            ".        $    Gilo94'
                             ^