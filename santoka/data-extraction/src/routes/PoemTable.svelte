<script lang="ts">
import type { StateStore } from '$lib/States'
import * as States from '$lib/States'
import * as Strings from '$lib/Strings'

export let state: StateStore

$: reversedPoems = [...$state.poems].reverse()
$: poems = [$state.currentPoem, ...reversedPoems]
</script>

<thead class="bg-gray-50">
	<tr>
		<th scope="col" class="py-3.5 pl-4 pr-3 text-left text-sm font-semibold text-gray-900 sm:pl-6">
			English
		</th>
		<th scope="col" class="px-3 py-3.5 text-left text-sm font-semibold text-gray-900">
			Japanese
		</th>
		<th scope="col" class="px-3 py-3.5 text-left text-sm font-semibold text-gray-900">
			Translator
		</th>
		<th scope="col" class="px-3 py-3.5 text-left text-sm font-semibold text-gray-900">
			Publication
		</th>
		<th scope="col" class="px-3 py-3.5 text-left text-sm font-semibold text-gray-900"> Notes </th>
		<th scope="col" class="relative py-3.5 pl-3 pr-4 sm:pr-6">
			<span class="sr-only"> Delete </span>
		</th>
	</tr>
</thead>

<tbody class="divide-y divide-gray-200 bg-white">
	{#each poems as poem}
		{@const translator = States.findTranslatorById($state, poem.translatorId)}
		{@const publication = States.findPublicationById($state, poem.publicationId)}
		<tr>
			<td class="max-w-sm py-4 pl-4 pr-3 text-sm font-medium text-gray-900 sm:pl-6">
				{@html Strings.convertNewlinesToBrTags(poem.englishText ?? '')}
			</td>
			<td class="px-3 py-4 text-sm text-gray-500">
				{@html Strings.convertNewlinesToBrTags(poem.japaneseText ?? '')}
			</td>
			<td class="whitespace-nowrap px-3 py-4 text-sm text-gray-500">
				{translator?.name ?? ''}
			</td>
			<td class="px-3 py-4 text-sm text-gray-500">
				{Strings.truncate(publication?.description, 32) ?? ''}
			</td>
			<td class="px-3 py-4 text-sm text-gray-500">
				{Strings.truncate(poem.notes, 32) ?? '—'}
			</td>
			<td class="relative whitespace-nowrap py-4 pl-3 pr-4 text-right text-sm font-medium sm:pr-6">
				{#if poem.editing}
					<a
						href="#"
						class="text-orange-600 hover:text-orange-900"
						on:click={() => state.dispatch({ type: 'resetCurrentPoem' })}
					>
						Reset
					</a>
				{:else}
					<a
						href="#"
						class="text-orange-600 hover:text-orange-900"
						on:click={() => state.dispatch({ type: 'deletePoem', id: poem.id })}
					>
						Delete
					</a>
				{/if}
			</td>
		</tr>
	{/each}
</tbody>
