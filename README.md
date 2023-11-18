Welcome to the rusty chat app.

What it does:

Sends messages, file and images between clients via a server\
Is a big ol' group chat, but without the corporations stalking you\

How to run it:

1) Clone the repo; cd into it\
2) In a terminal window run `cargo run --bin server`\
3) In another terminal window run `cargo run --bin client`\
4) Repeat step 3)\
5) Congrats, you have a running chatting app between two clients

Features:

You can choose host and port as you start up both server and client, specify after -c flag\
If you don't, it will do localhost:11111 - ONLY TESTED LOCALLY\
Clients get a generated nicknames unless specified otherwise by the -u flag
You can add as many clients as you want

In the chatting mode you can send plain text by just typing\
You can send png images by typing .image and a valid path\
You can send files by typing .file and a valid path\
Both files and images will save in the folder media/current username\
You can end the convo by typing .quit

Have fun!

todo:
write doc comments\
look for one more crates to add

cd Documents/hekovani/rust/rusty_dreams
