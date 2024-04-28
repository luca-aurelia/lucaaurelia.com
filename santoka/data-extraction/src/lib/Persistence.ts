import type { SaveableState } from './States'
import { browser } from '$app/environment'

export function saveState(state: SaveableState) {
	setItem(localStorageKey, state)
}

export function loadState(): SaveableState | null {
	return getItem(localStorageKey)
	// return null
}

function getItem<Parsed>(key: string): Parsed | null {
	if (!browser) {
		return null
	}

	const unparsed = window.localStorage.getItem(key)
	if (unparsed == null) {
		return null
	}

	return JSON.parse(unparsed)
}

function setItem(key: string, value: unknown) {
	if (!browser) {
		return
	}

	const stringified = JSON.stringify(value)
	window.localStorage.setItem(key, stringified)
}

const localStorageKey = 'savedState'
