<script lang="ts">
    import { computer } from "../state";    
    let command = "";

    let messages = [
        "putater v0.0.1\nrun help to get started\nmade with <3 by niko",
    ];

    computer.subscribe(value => {
        if(value) {
            messages = [...messages, `${value.logger.message}`];
        }
    });

    function handleKey(event: KeyboardEvent) {
        if (event.key === "Enter") {
            messages = [...messages, `> ${command}`];
            executeCommand(command);
            command = "";
        }
    }   

    function executeCommand(command: string) {  
        switch (command.split(" ")[0]) {  
            case "load":
                const args: string[] = command.split(" ").slice(1);
                const filePath = `/code/${args.join("/")}.pa`;
                messages = [...messages, `Loading program ${args.join("/")}`];  
                fetch(filePath)
                    .then(response => response.text())
                    .then(file => {
                        const contents = new Uint16Array(file.split("\n").map(line => parseInt(line, 2)));
                        messages = [...messages, `Program loaded: ${file} instructions!`];
                        $computer.loadProgram(contents);
                    })
                    .catch(error => {
                        messages = [...messages,`Error loading program ${filePath}: ${error}`];
                    }); 
                break;
            case "run":
                //@ts-ignore this works anyway lol          
                messages = [...messages, "Running program: ", $computer.cpu.programData];
                $computer.runProgram();
                break;
            case "debug":
                $computer.cpu.debug = true;
                messages = [...messages, `Debugging mode: ${$computer.cpu.debug}`];
                break;
            case "clear":
                messages = [
                    "putater v0.0.1",
                    "run help to get started!"
                ];
                break;
            case "help":
                messages = [...messages,
                    `help - shows this message
load - loads a program from a directory
run - runs program
debug - enable debug mode / load debug log
clear - clears the terminal`
                ];
                break;
            default:
                messages = [...messages, "Invalid command"];
        }
    }
</script>

<div class="terminal">
    {#key messages}
        {#each messages as message}
            {#if (message[0] === ">")}
                <pre class="input">{message}</pre>
            {:else}
                <pre>{message}</pre>
            {/if}
        {/each} 
    {/key}      
    <input type="text" on:keydown={handleKey} bind:value={command} placeholder="Type here..." />
</div>

<style>
    input {
        width:100%;
        position: sticky;
        padding:10px;
        border: 1px solid;
        background-color: black;
        color:white;
        font-family: monospace;
        box-sizing: border-box;
        bottom:0;
    }   
    pre {
        padding-left: 10px;
        padding-right: 0px;
    }
    .input {
        width: 100%;
        padding: 10px;
        padding-right: 0px;
        border-top: 1px solid;
        border-bottom: 1px solid;
    }
</style>
