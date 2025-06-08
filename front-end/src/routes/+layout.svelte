<script>
    import "../app.css";
    import 'flowbite';
    import Header from "../lib/Header.svelte";
    import Banner from "../lib/Banner.svelte";
    import Footer from "../lib/Footer.svelte";
    let { children } = $props();
</script>


<Header />
<slot />
<Footer />