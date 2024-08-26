import { writable } from "svelte/store";
import { putater } from "./putater";

export const computer = writable(new putater());