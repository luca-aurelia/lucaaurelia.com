<script lang="ts">
import * as States from '$lib/States'
import type { StateStore } from '$lib/States'
import { onMount } from 'svelte'
import * as KeyboardShortcuts from './KeyboardShortcuts'
import RecordTable from './RecordTable.svelte'
import Tabs from './Tabs.svelte'

let state: StateStore
let selected: 'poems' | 'publications' | 'translators' = 'poems'

const parentSelector = 'p'

onMount(async () => {
	const elements = Array.from(document.querySelectorAll(parentSelector))
	state = await States.create(elements)
})

function triggerKeyboardShortcuts(event: KeyboardEvent) {
	console.log(`Received keydown ${event.key}.`)

	const handler = KeyboardShortcuts.handlers[event.key.toLowerCase()]
	if (handler != null) {
		const shiftPressed = event.getModifierState('Shift')
		handler(shiftPressed, state.dispatch)
	}
}

async function selectElement(event: MouseEvent) {
	const target = event.target as HTMLElement
	const selector = `${parentSelector}, ${parentSelector} *`

	if (!target.matches(selector)) {
		return
	}

	const ancestorParagraph = target.closest('p')
	if (ancestorParagraph == null) {
		return
	}

	const ancestorParagraphIndex = $state.elements.findIndex(
		(element) => element === ancestorParagraph
	)

	state.dispatch({
		type: 'setSelectedElementIndex',
		newIndex: ancestorParagraphIndex
	})
}
</script>

<svelte:window on:keydown={triggerKeyboardShortcuts} on:click={selectElement} />

<div class="scraper-tools fixed bottom-0 w-full max-h-[25vh] overflow-y-scroll bg-white">
	{#if state}
		<RecordTable {state} />
	{/if}
</div>

<style lang="postcss">
:global(.terebess p:hover) {
	@apply bg-orange-200/20;
}

:global(.terebess p.selected) {
	@apply bg-orange-200/40;
}

:global(.terebess) {
	@apply p-8;
}

/* .scraper-tools {
	all: unset;
} */
</style>
