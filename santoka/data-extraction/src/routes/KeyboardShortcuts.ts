import type { Dispatch, GetTextFrom, Action } from '$lib/States'

interface KeyboardShortcutHandlers {
	[key: string]: undefined | KeyboardShortcutHandler
}

type KeyboardShortcutHandler = (shiftPressed: boolean, dispatch: Dispatch) => void

export const handlers: KeyboardShortcutHandlers = {
	enter: (shiftPressed: boolean, dispatch: Dispatch) => {
		const getTextFrom = decideWhereToGetText(shiftPressed)

		dispatchAndMoveSelection({
			shiftPressed,
			dispatch,
			actions: [
				{ type: 'addEnglishAndJapaneseTextToCurrentPoem', getTextFrom },
				{ type: 'saveCurrentPoem' }
			]
		})
	},

	a: (shiftPressed: boolean, dispatch: Dispatch) => {
		const getTextFrom = decideWhereToGetText(shiftPressed)

		dispatchAndMoveSelection({
			shiftPressed,
			dispatch,
			actions: [{ type: 'addEnglishAndJapaneseTextToCurrentPoem', getTextFrom }]
		})
	},

	e: (shiftPressed: boolean, dispatch: Dispatch) => {
		const getTextFrom = decideWhereToGetText(shiftPressed)

		dispatchAndMoveSelection({
			shiftPressed,
			dispatch,
			actions: [{ type: 'addEnglishTextToCurrentPoem', getTextFrom }]
		})
	},

	// All the letters in Japanese are already taken,
	// so use i because it's the second letter in Nihongo.
	i: (shiftPressed: boolean, dispatch: Dispatch) => {
		const getTextFrom = decideWhereToGetText(shiftPressed)

		dispatchAndMoveSelection({
			shiftPressed,
			dispatch,
			actions: [{ type: 'addJapaneseTextToCurrentPoem', getTextFrom }]
		})
	},

	s: (_shiftPressed: boolean, dispatch: Dispatch) => {
		dispatch([
			//
			{ type: 'saveCurrentPoem' }
			// { type: 'moveSelectedElementIndex', distanceToMove: 1 }
		])
	},

	n: (shiftPressed: boolean, dispatch: Dispatch) => {
		const getTextFrom = decideWhereToGetText(shiftPressed)

		dispatchAndMoveSelection({
			shiftPressed,
			dispatch,
			actions: [{ type: 'addNoteToCurrentPoem', getTextFrom }]
		})
	},

	p: (shiftPressed: boolean, dispatch: Dispatch) => {
		const getTextFrom = decideWhereToGetText(shiftPressed)

		dispatchAndMoveSelection({
			shiftPressed,
			dispatch,
			actions: [{ type: 'createPublication', getTextFrom }]
		})
	},

	t: (shiftPressed: boolean, dispatch: Dispatch) => {
		const getTextFrom = decideWhereToGetText(shiftPressed)

		dispatchAndMoveSelection({
			shiftPressed,
			dispatch,
			actions: [{ type: 'createTranslator', getTextFrom }]
		})
	},

	j: (_shiftPressed: boolean, dispatch: Dispatch) => {
		dispatch({ type: 'moveSelectedElementIndex', distanceToMove: 1 })
	},

	k: (_shiftPressed: boolean, dispatch: Dispatch) => {
		dispatch({ type: 'moveSelectedElementIndex', distanceToMove: -1 })
	},

	u: (_shiftPressed: boolean, dispatch: Dispatch) => {
		dispatch({ type: 'undo' })
	},

	r: (_shiftPressed: boolean, dispatch: Dispatch) => {
		dispatch({ type: 'redo' })
	}
}

function dispatchAndMoveSelection({
	shiftPressed,
	dispatch,
	actions
}: {
	shiftPressed: boolean
	dispatch: Dispatch
	actions: Action[]
}) {
	const actionsWithMove: Action[] = shiftPressed
		? actions
		: [...actions, { type: 'moveSelectedElementIndex', distanceToMove: 1 }]

	dispatch(actionsWithMove)
}

function decideWhereToGetText(shiftPressed: boolean): GetTextFrom {
	if (shiftPressed) {
		return { source: 'selectedText' }
	}

	return { source: 'selectedElement' }
}
