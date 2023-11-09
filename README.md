what a doozie

remarks:

1) As discussed before - it only works, when I thread::sleep

2) Why missing connections are only pushed into clients_to_remove in server after second unsuccesful attempt? The same goes for sending to server from client after server has disconnected

3) std::process::exit() is probably not the cleanliest way to stop the program, desiging the program so it stops if necessary would require rethink the entire structure of the program, because a show-stopping signal would have to be able to be sent from all relevant places (threads) to the main threads or something.
