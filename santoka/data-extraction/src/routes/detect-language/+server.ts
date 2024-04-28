import { json, error } from '@sveltejs/kit'
import type { Language } from './Shared'
import cld from 'cld'
import * as StatusCodes from '$lib/StatusCodes'

/** @type {import('./$types').RequestHandler} */
export async function POST({ request }) {
	const body = await request.json()
	console.log(`POST /detect-language`)

	const language = await detectLanguage(body.text)

	const response = { language }
	console.log(`Response:`, response)
	return json(response)
}

async function detectLanguage(text: string): Promise<Language> {
	for (const hack of hacks) {
		const applyHack = text.toLowerCase().includes(hack.trigger)

		if (applyHack) {
			return hack.language
		}
	}

	try {
		console.log(`Detecting language: ${text}`)
		const languageDetectorResponse = await cld.detect(text)
		console.log(`Detected language`, text)
		console.log(`Language detector response:`, languageDetectorResponse)

		if (languageDetectorResponse.languages.length > 1) {
			throw error(StatusCodes.serverError, `Detected multiple languages.`)
		}

		const language = languageDetectorResponse.languages[0].name.toLowerCase()
		if (language === 'english') {
			return 'english'
		}

		// If it guesses any other language besides English, assume it's Japanese
		// under the assumption that the Compact Language Detector is better at
		// distinguishing English than any other language.
		return 'japanese'
	} catch (detectionError) {
		// The Compact Language Detector has a hard time with romanized Japanese,
		// so if it doesn't get the language, assume it's Japanese.
		if (
			detectionError instanceof Error &&
			detectionError.message === 'Failed to identify language'
		) {
			return 'japanese'
		}

		console.error(`Error detecting language:`, detectionError)

		throw error(StatusCodes.serverError)
	}
}

interface Hack {
	trigger: string
	language: Language
}

// When we need to manually override Compact Language Detector.
const hacks: Hack[] = [
	{
		trigger: 'today i pick',
		language: 'english'
	}
]
