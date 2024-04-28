<script lang="ts">
import type { StateStore } from '$lib/States'
import PoemTable from './PoemTable.svelte'
import PublicationTable from './PublicationTable.svelte'
import TranslatorTable from './TranslatorTable.svelte'
import Tabs from './Tabs.svelte'

export let state: StateStore

let selectedValue: 'poems' | 'publications' | 'translators' = 'poems'
</script>

<div class="p-4 sm:p-6 lg:p-8">
	<div class="sm:flex sm:items-center">
		<div class="sm:flex-auto">
			<Tabs
				bind:selectedValue
				options={[
					{ label: `Poems ${$state.poems.length}`, value: 'poems' },
					{ label: `Publications ${$state.publications.length}`, value: 'publications' },
					{ label: `Translators ${$state.translators.length}`, value: 'translators' }
				]}
			/>
		</div>
	</div>

	<div class="mt-4 flow-root">
		<div class="-mx-4 -my-2 overflow-x-auto sm:-mx-6 lg:-mx-8">
			<div class="inline-block min-w-full py-2 align-middle sm:px-6 lg:px-8">
				<div class="overflow-hidden shadow ring-1 ring-black ring-opacity-5 sm:rounded-lg">
					<table class="min-w-full divide-y divide-gray-300">
						{#if selectedValue === 'poems'}
							<PoemTable {state} />
						{:else if selectedValue === 'publications'}
							<PublicationTable {state} />
						{:else if selectedValue === 'translators'}
							<TranslatorTable {state} />
						{/if}
					</table>
				</div>
			</div>
		</div>
	</div>
</div>
