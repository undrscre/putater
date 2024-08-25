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