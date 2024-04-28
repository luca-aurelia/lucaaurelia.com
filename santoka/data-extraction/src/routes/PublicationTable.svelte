<script lang="ts">
import type { StateStore } from '$lib/States'

export let state: StateStore
</script>

<thead class="bg-gray-50">
	<tr>
		<th scope="col" class="py-3.5 pl-4 pr-3 text-left text-sm font-semibold text-gray-900 sm:pl-6">
			Description
		</th>
		<th scope="col" class="relative py-3.5 pl-3 pr-4 sm:pr-6">
			<span class="sr-only"> Delete </span>
		</th>
	</tr>
</thead>

<tbody class="divide-y divide-gray-200 bg-white">
	{#each $state.publications as publication}
		<tr>
			<td class="max-w-sm py-4 pl-4 pr-3 text-sm font-medium text-gray-900 sm:pl-6">
				{publication.description}
			</td>
			<td class="relative whitespace-nowrap py-4 pl-3 pr-4 text-right text-sm font-medium sm:pr-6">
				<!-- on:click={() => state.dispatch({ type: 'deletePublication', id: publication.id })} -->
				<a href="#" class="text-orange-600 hover:text-orange-900"> Delete </a>
			</td>
		</tr>
	{/each}
</tbody>
