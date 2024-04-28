<script lang="ts" context="module">
export interface Option<Value> {
	label: string
	value: Value
}
</script>

<script lang="ts">
type Value = $$Generic

export let options: Option<Value>[]
export let selectedValue: Value
</script>

<nav class="flex space-x-4" aria-label="Tabs">
	<!-- Current: "bg-gray-100 text-gray-700", Default: "text-gray-500 hover:text-gray-700" -->
	{#each options as option}
		<a
			href="#"
			class:selected={selectedValue === option.value}
			on:click={() => (selectedValue = option.value)}
		>
			{option.label}
		</a>
	{/each}
</nav>

<style lang="postcss">
option {
	@apply capitalize;
}

a {
	@apply rounded-md px-3 py-2 text-sm font-medium capitalize;
}

a.selected {
	@apply bg-gray-100 text-gray-700;
}

a:not(.selected) {
	@apply text-gray-500 hover:text-gray-700;
}
</style>
