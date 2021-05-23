# manny
A better, more sleek community based man page effort.

## How

Manny is an enfort to help get more info on apps you love via the CLI. There is a seperate repo, [MannyDB](https://github.com/Milo123459/mannydb) which is where your app information is stored. If you want to add your app to manny, or update it, you submit a PR there. On the repo you will find contribution guidelines. Make sure to follow them!

After adding, you can type `manny update`, which will update the DB locally. It is stored in the home dir: $HOME/.manny/db 
This DB folder is a cloned version of the DB repo. You need to have git installed. When you run it for the first time, each time you run a command, it checks if the DB is present. If not, it'll clone it. This should only take a few seconds. Once cloned, every file ending with `.md` will then be an app you can later get info on.

Manny's CLI / core (the repo you are on) is written in Rust. This allows the CLI to be ridiculously fast. 

If you have any other questions, please create an issue! I'll try and help out ASAP. (Please don't make issues on the MannyDB repo, there isn't a point)
## Generic

A generic app means that the name of the app you added is already used, ie, an app with the name of `action` wouldn't work as there is an action command. To bypass this, simply type `manny manifest action` rather than `manny action`.