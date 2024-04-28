import * as Persistence from './Persistence'
import type { Poem, PartiallyCompletePoem } from './Poems'
import * as Poems from './Poems'
import * as Strings from './Strings'
import { detectLanguage } from '../routes/detect-language/ClientApi'
import type { Language } from '../routes/detect-language/ClientApi'
import { writable, get } from 'svelte/store'
import type { Readable } from 'svelte/store'
import * as SequentialQueues from './SequentialQueues'
import * as Arrays from '$lib/Arrays'
import * as Numbers from '$lib/Numbers'
import type { Publication } from '$lib/Publications'
import type { Translator } from '$lib/Translators'
import type { Nullish } from '$lib/Nullishes'

export interface SaveableState {
	id: string
	selectedElementIndex: number
	currentPoem: PartiallyCompletePoem
	currentPublicationId: number
	currentTranslatorId: number
	poems: Poem[]
	publications: Publication[]
	translators: Translator[]
}

export interface State extends SaveableState {
	elements: HTMLElement[]
	undoActions: UndoableAction[]
	redoActions: UndoableAction[]
}

interface UndoableAction {
	undo: (...args: unknown[]) => unknown
	redo: (...args: unknown[]) => unknown
}

export type Dispatch = (action: Action | Action[]) => Promise<void>

export interface StateStore extends Readable<State> {
	dispatch: Dispatch
}

export type GetTextFrom =
	| {
			source: 'selectedElement'
	  }
	| { source: 'selectedText' }
	| {
			source: 'alreadyProvided'
			text: string
	  }

export interface Actions {
	composite: {
		type: 'composite'
		subActions: Action[]
	}

	addEnglishTextToCurrentPoem: {
		type: 'addEnglishTextToCurrentPoem'
		getTextFrom: GetTextFrom
	}

	addJapaneseTextToCurrentPoem: {
		type: 'addJapaneseTextToCurrentPoem'
		getTextFrom: GetTextFrom
	}

	addEnglishAndJapaneseTextToCurrentPoem: {
		type: 'addEnglishAndJapaneseTextToCurrentPoem'
		getTextFrom: GetTextFrom
	}

	addNoteToCurrentPoem: {
		type: 'addNoteToCurrentPoem'
		getTextFrom: GetTextFrom
	}

	resetCurrentPoem: {
		type: 'resetCurrentPoem'
	}

	saveCurrentPoem: {
		type: 'saveCurrentPoem'
	}

	deletePoem: {
		type: 'deletePoem'
		id: number
	}

	// deletePublication: {
	// 	type: 'deletePublication'
	// 	id: number
	// }

	// deleteTranslator: {
	// 	type: 'deleteTranslator'
	// 	id: number
	// }

	moveSelectedElementIndex: {
		type: 'moveSelectedElementIndex'
		distanceToMove: number // +1 to go to the next element, -1 to go to the previous element.
	}

	setSelectedElementIndex: {
		type: 'setSelectedElementIndex'
		newIndex: number
	}

	createPublication: {
		type: 'createPublication'
		getTextFrom: GetTextFrom
	}

	createTranslator: {
		type: 'createTranslator'
		getTextFrom: GetTextFrom
	}

	undo: {
		type: 'undo'
	}

	redo: {
		type: 'redo'
	}
}

export type ActionTypes = keyof Actions

type ActionHandlers = {
	[actionType in ActionTypes]: (state: State, action: Actions[actionType]) => Promise<State>
}

export type Action = Actions[ActionTypes]

export async function create(elements: HTMLElement[]): Promise<StateStore> {
	const startingState = await getStartingState(elements)
	const state = writable(startingState)

	const queue = SequentialQueues.create({ timeoutMilliseconds: 10_000 })

	async function dispatch(actionOrActions: Action | Action[]) {
		let queueLabel
		let action: Action
		if (Array.isArray(actionOrActions)) {
			const actions = actionOrActions
			queueLabel = actions.map((action) => action.type).join(' + ')
			action = {
				type: 'composite',
				subActions: actions
			}
		} else {
			action = actionOrActions
			queueLabel = action.type
		}

		console.log(`Dispatching`, action)
		await queue.push(queueLabel, async () => {
			const $state = get(state)
			const newState = await update($state, action)
			state.set(newState)
		})
	}

	return {
		dispatch,
		subscribe: state.subscribe
	}
}

export function findTranslatorById(state: State, id: number | Nullish): Translator | undefined {
	if (id == null) {
		return
	}

	return state.translators.find((translator) => translator.id === id)
}

export function findPublicationById(state: State, id: number | Nullish): Publication | undefined {
	if (id == null) {
		return
	}

	return state.publications.find((publication) => publication.id === id)
}

async function getStartingState(elements: HTMLElement[]): Promise<State> {
	const saved = Persistence.loadState()

	let startingState: State
	if (saved != null) {
		startingState = {
			...saved,
			elements,
			undoActions: [],
			redoActions: []
		}
	} else {
		const now = new Date()

		const year = now.getFullYear()
		const month = now.getMonth() + 1
		const day = now.getDate()
		const hour = now.getHours()
		const minute = now.getMinutes()

		const id = `${year}.${month}.${day}.${hour}.${minute}`

		startingState = {
			id,
			currentPoem: {
				id: 0,
				editing: true,
				translatorId: 0,
				publicationId: 0
			},
			currentPublicationId: 0,
			currentTranslatorId: 0,
			poems: [],
			publications: [
				{
					id: 0,
					description: `"Santoka" by R. H. Blyth, in A History of Haiku. Tokyo: The Hokuseido Press, 1964. Volume 2: From Issa to the Present. pp. 173-88.`
				}
			],
			translators: [
				{
					id: 0,
					name: 'R. H. Blyth'
				}
			],
			selectedElementIndex: 0,
			elements,
			undoActions: [],
			redoActions: []
		}
	}

	for (const element of elements) {
		element.classList.remove('selected')
	}

	const selectedElement = getSelectedElement(startingState)
	selectedElement.classList.add('selected')

	return startingState
}

export async function update<ActionType extends ActionTypes>(
	state: State,
	action: Actions[ActionType]
): Promise<State> {
	console.log(`Updating state: ${action.type}.`)
	const handler = actionHandlers[action.type] as ActionHandlers[ActionType]

	if (handler == null) {
		const stringified = Strings.prettyPrint(action)
		throw new Error(`Unknown action:\n${stringified}`)
	}

	const newState = await handler(state, action)

	Persistence.saveState({
		id: newState.id,
		selectedElementIndex: newState.selectedElementIndex,
		currentPoem: newState.currentPoem,
		currentPublicationId: newState.currentPublicationId,
		currentTranslatorId: newState.currentTranslatorId,
		poems: newState.poems,
		publications: newState.publications,
		translators: newState.translators
	})

	console.log(`New state`, newState)
	console.log('')

	return newState
}

const actionHandlers: ActionHandlers = {
	async composite(state: State, action: Actions['composite']): Promise<State> {
		async function undo() {
			for (let i = 0; i < action.subActions.length; i++) {
				actionHandlers.undo(state, { type: 'undo' })
			}
		}

		async function redo() {
			let previousState = state
			for (const subAction of action.subActions) {
				previousState = await update(previousState, subAction)
			}
		}

		return await makeUndoable(state, {
			undo,
			redo
		})
	},

	async addEnglishAndJapaneseTextToCurrentPoem(
		state: State,
		action: Actions['addEnglishAndJapaneseTextToCurrentPoem']
	): Promise<State> {
		const lines = getSelectedLines(state, action.getTextFrom)

		if (lines.length === 0) {
			return state
		}

		const separated = await separateLanguages(lines)

		const subActions: Action[] = []

		if (separated.englishText != null) {
			subActions.push({
				type: 'addEnglishTextToCurrentPoem',
				getTextFrom: { source: 'alreadyProvided', text: separated.englishText }
			})
		}

		if (separated.japaneseText != null) {
			subActions.push({
				type: 'addJapaneseTextToCurrentPoem',
				getTextFrom: { source: 'alreadyProvided', text: separated.japaneseText }
			})
		}

		return actionHandlers.composite(state, {
			type: 'composite',
			subActions
		})
	},

	async addEnglishTextToCurrentPoem(
		state: State,
		action: Actions['addEnglishTextToCurrentPoem']
	): Promise<State> {
		const { currentPoem } = state

		const text = getSelectedText(state, action.getTextFrom)

		if (Strings.isBlank(text)) {
			return state
		}

		const originalText = currentPoem.englishText

		const cleanedText = removeUnnecessaryText(text)

		const joinedText = [originalText, cleanedText]
			.filter((text) => !Strings.isBlank(text))
			.join('\n')

		async function redo() {
			currentPoem.englishText = joinedText
		}

		async function undo() {
			currentPoem.englishText = originalText
		}

		return await makeUndoable(state, {
			undo,
			redo
		})
	},

	async addJapaneseTextToCurrentPoem(
		state: State,
		action: Actions['addJapaneseTextToCurrentPoem']
	): Promise<State> {
		const { currentPoem } = state

		const text = getSelectedText(state, action.getTextFrom)

		if (Strings.isBlank(text)) {
			return state
		}

		const originalText = currentPoem.japaneseText

		const cleanedText = removeUnnecessaryText(text)

		const joinedText = [originalText, cleanedText]
			.filter((text) => !Strings.isBlank(text))
			.join('\n')

		async function redo() {
			currentPoem.japaneseText = joinedText
		}

		async function undo() {
			currentPoem.japaneseText = originalText
		}

		return await makeUndoable(state, {
			undo,
			redo
		})
	},

	async addNoteToCurrentPoem(
		state: State,
		action: Actions['addNoteToCurrentPoem']
	): Promise<State> {
		const text = getSelectedText(state, action.getTextFrom)

		if (Strings.isBlank(text)) {
			return state
		}

		const originalNotes = state.currentPoem.notes

		async function undo() {
			state.currentPoem.notes = originalNotes
		}

		async function redo() {
			state.currentPoem.notes = text
		}

		return await makeUndoable(state, {
			undo,
			redo
		})
	},

	async deletePoem(state: State, action: Actions['deletePoem']): Promise<State> {
		const idToDelete = action.id
		const originalPoems = state.poems

		async function undo() {
			state.poems = originalPoems
		}

		async function redo() {
			state.poems = state.poems.filter((poem) => poem.id !== idToDelete)
		}

		return await makeUndoable(state, {
			undo,
			redo
		})
	},

	// async deletePublication(state: State, action: Actions['deletePublication']): Promise<State> {
	// 	const idToDelete = action.id
	// 	const originalPublications = state.publications
	// 	const originalPoems = state.poems

	// 	async function undo() {
	// 		state.publications = originalPublications
	// 	}

	// 	async function redo() {
	// 		state.publications = state.publications.filter((publication) => publication.id !== idToDelete)
	// 	}

	// 	return await makeUndoable(state, {
	// 		undo,
	// 		redo
	// 	})
	// },

	async resetCurrentPoem(
		state: State
		// action: Actions['resetCurrentPoem']
	): Promise<State> {
		const originalCurrentPoem = state.currentPoem

		async function undo() {
			state.currentPoem = originalCurrentPoem
		}

		async function redo() {
			state.currentPoem = Poems.createCurrentPoem(
				state.currentPoem.id,
				state.currentTranslatorId,
				state.currentPublicationId
			)
		}

		return await makeUndoable(state, {
			undo,
			redo
		})
	},

	async moveSelectedElementIndex(
		state: State,
		action: Actions['moveSelectedElementIndex']
	): Promise<State> {
		const lastIndex = Arrays.lastIndex(state.elements)
		const { distanceToMove } = action

		let newIndex = state.selectedElementIndex + distanceToMove
		newIndex = Numbers.clamp({ value: newIndex, min: 0, max: lastIndex })

		return actionHandlers.setSelectedElementIndex(state, {
			type: 'setSelectedElementIndex',
			newIndex
		})
	},

	async setSelectedElementIndex(
		state: State,
		action: Actions['setSelectedElementIndex']
	): Promise<State> {
		const originalSelectedElementIndex = state.selectedElementIndex
		const originalSelectedElement = state.elements[originalSelectedElementIndex]

		const { newIndex } = action
		const newlySelectedElement = state.elements[newIndex]

		async function undo() {
			newlySelectedElement.classList.remove('selected')
			originalSelectedElement.classList.add('selected')
			state.selectedElementIndex = originalSelectedElementIndex
			originalSelectedElement.scrollIntoView({
				behavior: 'smooth',
				inline: 'center',
				block: 'center'
			})
		}

		async function redo() {
			originalSelectedElement.classList.remove('selected')
			newlySelectedElement.classList.add('selected')
			state.selectedElementIndex = action.newIndex
			newlySelectedElement.scrollIntoView({
				behavior: 'smooth',
				inline: 'center',
				block: 'center'
			})
		}

		return await makeUndoable(state, {
			undo,
			redo
		})
	},

	async saveCurrentPoem(
		state: State
		// _action: Actions['saveCurrentPoem']
	): Promise<State> {
		const { poems, currentPoem, currentPublicationId, currentTranslatorId } = state
		const originalCurrentPoem = currentPoem

		const completePoem = Poems.complete(currentPoem)

		if (completePoem == null) {
			const prettifiedPoem = Strings.prettyPrint(currentPoem)
			throw new Error(
				`Tried to mark the current poem as complete, but it's not complete: ${prettifiedPoem}`
			)
		}

		// TypeScript's inference can't figure out that completePoem is guaranteed to be non-null
		// when we read it in the undo and redo functions below. So give it a little help.
		const nonNullCompletePoem = completePoem as Poem

		if (completePoem.japaneseText == null) {
			console.warn(`Saving a poem without Japanese text.`)
		}

		const alreadySavedPoem = state.poems.find((poem) => {
			return poem.englishText === completePoem.englishText
		})

		async function undo() {
			state.currentPoem = originalCurrentPoem
			state.poems = state.poems.filter((poem) => poem.id !== nonNullCompletePoem.id)
		}

		async function redo() {
			if (alreadySavedPoem) {
				console.warn(`Potential duplicate poem.`)
				console.warn(`Already saved version:`, alreadySavedPoem)
				console.warn(`Current version:`, completePoem)
			}

			state.currentPoem = Poems.createCurrentPoem(
				currentPoem.id + 1,
				currentTranslatorId,
				currentPublicationId
			)

			poems.push(nonNullCompletePoem)
		}

		return await makeUndoable(state, {
			undo,
			redo
		})
	},

	async createPublication(state: State, action: Actions['createPublication']): Promise<State> {
		const text = getSelectedText(state, action.getTextFrom)

		if (Strings.isBlank(text)) {
			return state
		}

		const originalCurrentPublicationId = state.currentPublicationId
		const originalCurrentPoemPublicationId = state.currentPoem.publicationId

		async function undo() {
			state.currentPublicationId = originalCurrentPublicationId
			state.currentPoem.publicationId = originalCurrentPoemPublicationId
			state.publications = state.publications.filter(
				(publication) => publication.description !== text
			)
		}

		async function redo() {
			let publication = state.publications.find((publication) => publication.description === text)
			if (!publication) {
				const id = getNextId(state.publications)
				publication = {
					id,
					description: text as string
				}
				state.publications.push(publication)
			}

			state.currentPublicationId = publication.id
			state.currentPoem.publicationId = publication.id
		}

		return await makeUndoable(state, {
			undo,
			redo
		})
	},

	async createTranslator(state: State, action: Actions['createTranslator']): Promise<State> {
		const text = getSelectedText(state, action.getTextFrom)

		if (Strings.isBlank(text)) {
			return state
		}

		const originalCurrentTranslatorId = state.currentTranslatorId
		const originalCurrentPoemTranslatorId = state.currentPoem.translatorId

		async function undo() {
			state.currentTranslatorId = originalCurrentTranslatorId
			state.currentPoem.translatorId = originalCurrentPoemTranslatorId
			state.translators = state.translators.filter((translator) => translator.name !== text)
		}

		async function redo() {
			let translator = state.translators.find((translator) => translator.name === text)
			if (!translator) {
				const id = getNextId(state.translators)
				translator = {
					id,
					name: text as string
				}
				state.translators.push(translator)
			}
			state.currentTranslatorId = translator.id
			state.currentPoem.translatorId = translator.id
		}

		return await makeUndoable(state, {
			undo,
			redo
		})
	},

	async undo(state: State /* action: Actions['undo']*/): Promise<State> {
		const mostRecentUndoAction = state.undoActions.pop()

		if (mostRecentUndoAction == null) {
			return state
		}

		state.redoActions.push(mostRecentUndoAction)

		mostRecentUndoAction.undo()

		return state
	},

	async redo(state: State /* action: Actions['redo'] */): Promise<State> {
		const mostRecentRedoAction = state.redoActions.pop()

		if (mostRecentRedoAction == null) {
			return state
		}

		state.undoActions.push(mostRecentRedoAction)

		mostRecentRedoAction.redo()

		return state
	}
}

export function getSelectedElement(state: State) {
	const { selectedElementIndex, elements } = state
	return elements[selectedElementIndex]
}

function getSelectedText(state: State, getTextFrom: GetTextFrom): string | undefined {
	const lines = getSelectedLines(state, getTextFrom)

	if (lines.length === 0) {
		return
	}

	return lines.join('\n')
}

async function separateLanguages(
	lines: string[]
): Promise<{ englishText: string | null; japaneseText: string | null }> {
	const linesWithLanguagePromises = lines.map(async (line) => {
		const language = await detectLanguage(line)
		return {
			line,
			language
		}
	})

	const linesWithLanguage = await Promise.all(linesWithLanguagePromises)

	const englishText = joinLinesOfTheSameLanguage(linesWithLanguage, 'english')
	const japaneseText = joinLinesOfTheSameLanguage(linesWithLanguage, 'japanese')

	return { englishText, japaneseText }
}

function getSelectedLines(state: State, getFrom: GetTextFrom): string[] {
	if (getFrom.source === 'selectedElement') {
		return getTextLinesFromSelectedElement(state)
	} else if (getFrom.source === 'selectedText') {
		return getTextLinesFromTextSelection()
	} else {
		return [getFrom.text]
	}
}

function getTextLinesFromTextSelection(): string[] {
	const selection = document.getSelection()?.toString().trim()

	if (selection == null) {
		return []
	}

	return [selection]
}

function getTextLinesFromSelectedElement(state: State) {
	const element = getSelectedElement(state)
	const subspan = element.querySelector(`span[style="font-size: 10pt; font-family: Verdana"]`)

	if (subspan == null) {
		console.warn(`No subspan found in element`, element)
		return []
	}

	const innerHtml = subspan.innerHTML
		.trim()
		.replace(/\s/g, ' ') // Replace all whitespace (including, for example, tabs and newlines) with space characters.
		.replace(/\s{2,}/g, ' ') // Then replace any sequences of two or more whitespace characters with a single space.

	const lines = innerHtml
		.split('<br>')
		.map((line) => {
			// Remove HTML tags. the `.` character will match anything except line breaks, so this only works if line breaks have been removed as above.
			// +? means "match as few as possible".
			return line.replace(/<.+?>/g, '').trim()
		})
		.filter((line) => line.length > 0)

	return lines
}

interface LineWithLanguage {
	line: string
	language: Language
}

function joinLinesOfTheSameLanguage(
	linesWithLanguage: Array<LineWithLanguage>,
	language: Language
): string | null {
	const lines = linesWithLanguage
		.filter((lineWithLanguage) => lineWithLanguage.language === language)
		.map((lineWithLanguage) => lineWithLanguage.line)

	if (lines.length === 0) {
		return null
	} else {
		return lines.join('\n')
	}
}

function getNextId(records: Array<{ id: number }>): number {
	let max = 0
	for (const record of records) {
		max = Math.max(max, record.id)
	}
	return max + 1
}

function removeUnnecessaryText(text: string | Nullish): string | null {
	if (text == null) {
		return null
	}

	return text
		.replace(/\(tr\. by .*\)/gi, '')
		.replace(/^\d+\.\s/g, '')
		.replace(/^\(/gm, '')
		.replace(/\)$/gm, '')
		.trim()
}

async function makeUndoable(state: State, undoableAction: UndoableAction) {
	// Actually apply the change.
	await undoableAction.redo()

	// Record the change in the undo history.
	state.undoActions.push(undoableAction)
	state.redoActions = []

	// Return the mutated state.
	return state
}
