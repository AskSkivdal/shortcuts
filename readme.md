# Shortcuts
This is a shortcuts cli tool for shortening commands. It reads text files from the users config
directory. To add shortcuts add a file with just the name and in the file you put the commands.


## Examples

`~/.config/shortcuts/ollama`
```
ollama run llama3.2:1b
ollama run llama3.2:3b

ollama run qwen2.5-coder:3b
ollama run qwen2.5-coder:7b
ollama run qwen2.5-coder:14b
```

`~/.bashrc`
```
alias llms='eval "$(shortcuts ollama)"'
```