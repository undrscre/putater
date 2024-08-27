// import { format } from 'util';

export class Stack<T> {
    stack: T[] = [];

    push(item: T): void {
        this.stack.push(item);
    }

    pop(): T | undefined {
        return this.stack.pop();
    }

    isEmpty(): boolean {
        return this.stack.length === 0;
    }

    get items(): T[] {
        return this.stack;
    }
}

export class Logger {
    message: string = "";

    log(...args: any[]): void {
        let message = args.join(" ");   
        console.log(message);
        this.message += message + "\n";
    }
        
    clear(): void {
        this.message = "";
    }
}