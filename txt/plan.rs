let (temp, name) = IpcOneShotSever::new();
Command::new().arg(name)
temp.accept() -> IpcSender
save IpcSender
|
|
v
new channel()
sender.send(..args, IpcSender<Response>)

receive IpcReceiver<Response>

