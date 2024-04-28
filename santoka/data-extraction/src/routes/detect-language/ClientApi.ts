import type { Language } from './Shared'

export * from './Shared'

export async function detectLanguage(text: string): Promise<Language> {
	const response = await fetch('/detect-language', {
		method: 'POST',
		body: JSON.stringify({ text }),
		headers: {
			'content-type': 'application/json'
		}
	})
	const { language } = await response.json()
	return language
}
