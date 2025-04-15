Output assets/demo.gif

Set Theme "GruvboxDark"
Set Padding 20
Set Margin 10
Set MarginFill "#458588"
Set BorderRadius 30
Set Height 900
Set TypingSpeed 0.03

Set Shell fish

Hide
Type "cd $(mktemp -d -t git-squad.XXXX)"
Enter
Type "mv ~/.config/git-squad/buddies.yaml ~/.config/git-squad/buddies.yaml.bup"
Enter
Type "git-squad alone"
Enter
Type "git init"
Enter
Type `git config user.name "Captain Hook"`
Enter
Type "git config user.email captain.hook@example.com"
Enter
Type "git config commit.gpgsign false"
Enter
Type "git-squad completions fish > git-squad.fish"
Enter
Type ". ./git-squad.fish"
Enter
Type "set fish_autosuggestion_enabled 0"
Enter
Type "clear"
Enter
Show

Type "git-squad --help"
Enter
Sleep 4s
Type "git-squad create pp"
Enter
Sleep 500ms
Type "Peter Pan"
Enter
Sleep 500ms
Type "peter.pan@example.com"
Enter
Sleep 500ms
Type "git-squad create wd"
Enter
Sleep 500ms
Type "Wendy Darling"
Enter
Sleep 500ms
Type "wendy.darling@example.com"
Enter
Sleep 500ms
Type "git-squad"
Enter
Sleep 500ms
Type "git-squad with "
Tab 
Sleep 1000ms
Down
Enter
Tab 
Sleep 500ms
Down
Down
Enter
Enter
Sleep 2s
Type "git-squad active"
Enter
Sleep 2.5s
Type `echo "Dream Team!" > file`
Enter
Sleep 500ms
Type "git add file"
Enter
Sleep 500ms
Type "git commit"
Enter
Escape
Type "O"
Type "work on file with peter and wendy"
Escape
Sleep 500ms
Type ":wq"
Enter
Sleep 1s
Type "git show"
Enter
Sleep 5s

Hide
Type "mv ~/.config/git-squad/buddies.yaml.bup ~/.config/git-squad/buddies.yaml"
Enter
Type "git-squad alone"
Enter
