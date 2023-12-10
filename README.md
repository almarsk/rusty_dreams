Welcome to the rusty chat app.

What it does:

Sends messages, file and images between clients via a server\
It's really a big ol' group chat

Prerequistits:
1) Make sure you have postgres installed on your system

How to run it:

0) setting up the database:\
    start a .env file with a valid url to a postgres database\
    in postgres shell start a table called rusty_app_user\
1) Clone the repo; cd into it
2) In a terminal window run `cargo run --bin server`
3) In another terminal window run `cargo run --bin client`
4) Repeat step 3)
5) Congrats, you have a running chatting app between two clients

Features:

You can choose host and port as you start up both server and client, specify by --host and --port flags\
If you don't, it defaults to localhost:11111 - ONLY TESTED LOCALLY\
Clients get a generated nicknames unless specified otherwise by the -u flag\
You can add as many clients as you want

In the chatting mode you can send plain text by just typing\
You can send png images by typing .image and a valid path\
You can send files by typing .file and a valid path\
Both files and images will save in the folder media/current username of the addresee\
You can end the convo by typing .quit

Have fun!
