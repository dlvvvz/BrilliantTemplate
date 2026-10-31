<script lang="ts">
	import favicon from '#lib/assets/favicon.svg';
	import type { LayoutProps } from './$types';
	import { ModeWatcher } from 'mode-watcher'
	import '#lib/layout.css'

	let { children }: LayoutProps = $props();
</script>

<svelte:head>
	<link rel="icon" href={favicon} />
</svelte:head>

<ModeWatcher/>

<main class="h-screen flex bg-gray-200 text-black dark:bg-gray-950 dark:text-white">
	{@render children()}
</main>

