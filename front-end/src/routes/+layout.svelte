<script lang="ts">
	import { navigating } from '$app/stores';
	import Header from '$lib/components/Header/Header.svelte';
	import Loader from '$lib/components/Loader/Loader.svelte';
	import Notification from '$lib/components/Notification/Notification.svelte';
	import PageTransition from '$lib/components/Transition/PageTransition.svelte';
	import '../app.css';
	// import '../styles.min.css';
	import { loading } from '$lib/stores/loading.store';
	import type { PageData } from './$types';

	export let data: PageData;

	$: loading.setNavigate(!!$navigating);
	$: loading.setLoading(!!$navigating, 'Loading, please wait...');
</script>

<svelte:head>
	<meta
		name="description"
		content="Building a secure and performant authentication system using rust (actix-web) and sveltekit"
	/>
	<meta
		name="keywords"
		content="rust, actix-web, typescript, javascript, svelte, tailwindcss, tutorial, software, coding, development, engineering, inclusive, community"
	/>

</svelte:head>

<PageTransition key={data.url} duration={600}>
	<Header />
	<Notification />
	<Loader />
	<slot />
</PageTransition>