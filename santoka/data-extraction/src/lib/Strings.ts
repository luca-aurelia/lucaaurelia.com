import type { Nullish } from './Nullishes'

export function isBlank(maybeString: string | null | undefined): maybeString is Nullish {
	if (maybeString == null) {
		return true
	}

	return maybeString.trim().length === 0
}

export function prettyPrint(x: unknown): string {
	return JSON.stringify(x, undefined, 2)
}

export function truncate(string: string | Nullish, length: number): string | null {
	if (string == null) {
		return null
	}

	if (string.length > length - 3) {
		return string.slice(0, length - 3) + '...'
	}
	return string
}

export function convertNewlinesToBrTags(string: string): string {
	return string.replaceAll('\n', '<br />')
}
