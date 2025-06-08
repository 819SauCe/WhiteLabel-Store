<script>
	import { fly } from 'svelte/transition';

	let imagens = [
		"https://t3.ftcdn.net/jpg/02/62/18/46/360_F_262184611_bXhmboL9oE6k2ILu4qXxNWFhNJCEbTn2.jpg",
		"https://t4.ftcdn.net/jpg/03/06/69/49/360_F_306694930_S3Z8H9Qk1MN79ZUe7bEWqTFuonRZdemw.jpg",
		"https://cdn.vectorstock.com/i/500p/57/56/shopping-cart-banner-online-store-vector-42935756.jpg"
	];
	let atual = 0;
	let direcao = 1;

	function proxima() {
		direcao = 1;
		atual = (atual + 1) % imagens.length;
	}

	function anterior() {
		direcao = -1;
		atual = (atual - 1 + imagens.length) % imagens.length;
	}

    proxima();
    setInterval(proxima, 5000);
</script>
<div style="width: 100%; margin: auto; overflow: hidden;">
	<div style="position: relative; height: 30rem;">
		{#each imagens as img, i (i)}
			{#if i === atual}
				<img
					src={img}
					alt="Carrossel"
					style="width: 100%; height: 100%; object-fit: cover; position: absolute; top: 0; left: 0;"
					in:fly={{ x: direcao * 500, duration: 300 }}
					out:fly={{ x: -direcao * 500, duration: 300 }}
				/>
			{/if}
		{/each}
		<div style="position: absolute; top: 50%; left: 0; right: 0; display: flex; justify-content: space-between; transform: translateY(-50%); padding: 0 1rem; z-index: 1;">
			<button on:click={anterior} class="botao-carrossel">◀</button>
            <button on:click={proxima} class="botao-carrossel">▶</button>
		</div>
	</div>
</div>

<style>
	.botao-carrossel {
		background-color: white;
		width: 2.5rem;
		height: 2.5rem;
		border-radius: 50%;
		display: flex;
		align-items: center;
		justify-content: center;
		font-size: 1.25rem;
		border: none;
		line-height: 1;
        transition: background-color 0.3s ease;
	}
    .botao-carrossel:hover {
        background-color: #f0f0f0;
        cursor: pointer;
    }
</style>
