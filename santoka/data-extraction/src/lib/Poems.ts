import * as Strings from './Strings'

export type PartiallyCompletePoem = Omit<Partial<Poem>, 'editing'> & { id: number; editing: true }

export interface Poem {
	id: number
	englishText: string
	japaneseText?: string
	publicationId: number
	translatorId: number
	notes?: string
	editing: false
}

export function createCurrentPoem(
	id: number,
	translatorId: number,
	publicationId: number
): PartiallyCompletePoem {
	return {
		id,
		editing: true,
		translatorId: translatorId,
		publicationId: publicationId
	}
}

export function complete(partialPoem: PartiallyCompletePoem): Poem | null {
	// Japanese text and notes are optional.
	const { id, englishText, publicationId, translatorId } = partialPoem
	const isComplete =
		id != null && !Strings.isBlank(englishText) && publicationId != null && translatorId != null

	if (isComplete) {
		const completePoem = {
			...partialPoem,
			editing: false
		} as Poem

		return completePoem
	}

	return null
}
