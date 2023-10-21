import { writable } from "svelte/store";

export const notification = writable<{
	message?: string;

	colorName?: string;
}>({
	message: "",
	colorName: "",
});
