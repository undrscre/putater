<script lang="ts">
    import { computer } from "../state";    
    let command = "";

    let messages: string[] = [];

    function handleKey(event: KeyboardEvent) {
        if (event.key === "Enter") {
            messages = [...messages, `> ${command}`];
            executeCommand(command);
            command = "";
        }
    }   

    function executeCommand(command: string) {  
        const args: string[] = command.split(" ").slice(1);
        switch (command.split(" ")[0]) {  
            case "load":
                const filePath = `/code/${args.join("/")}.pa`;
                messages = [...messages, `Loading program ${args.join("/")}`];  
                fetch(filePath)
                    .then(response => response.text())
                    .then(file => {
                        const contents = new Uint16Array(file.split("\n").map(line => parseInt(line, 2)));
                        messages = [...messages, `Program loaded: \n${file}`];
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
                messages = [...messages, "Program execution complete"];
                break;
            case "debug":
                switch (args[0]) {
                    case "on":
                        $computer.cpu.debug = true;
                        messages = [...messages, "Debug mode: on"]
                        break;
                    case "off":
                        $computer.cpu.debug = false;
                        messages = [...messages, "Debug mode: off"]
                        break;
                    case "log":
                        messages = [...messages, `${$computer.logger.message}`];
                        break;
                }
                break;
            case "clear":
                messages = [""];
                break;
            case "help":
                messages = [...messages,
                    `help - shows this message
load - loads a program from a directory
run - runs program
debug on/off/log - enable debug mode / load debug log
clear - clears the terminal`
                ];
                break;
            default:
                messages = [...messages, "Invalid command"];
        }
    }
</script>

<div class="terminal">
    <div class="messages">
        <pre><img src="/assets/logo.png" alt="putater" width="200" />
putater v0.0.1
run help to get started
made with &lt;3 by niko</pre>
        {#key messages}
            {#each messages as message}
                {#if (message[0] === ">")}
                    <pre class="input">{message.slice(2)}</pre>
                {:else}
                    <pre>{message}</pre>
                {/if}
            {/each} 
        {/key}      
    </div>

    <input type="text" on:keydown={handleKey} bind:value={command} placeholder="Type here..." />
</div>

<style>
    .terminal {
        padding:30px;
        padding-bottom: 60px;
    }
    input {
        width:100%;
        position:fixed;
        padding:10px;
        border-top: 1px solid gray;
        border:none;
        background-color: rgb(41, 41, 41);
        color:white;
        font-family: monospace;
        box-sizing: border-box;
        bottom:0;
        left:0;
        &::placeholder::before {
            content: "> ";
        }
    }

    .input {
        width: 100%;
        padding: 10px;
        padding-right: 0px;
        border-top: 1px solid gray;
        border-bottom: 1px solid gray;
        &::before {
            content: "> ";
            color:#ffdf7e;
            font-weight: bold;
        }
    }
</style>
