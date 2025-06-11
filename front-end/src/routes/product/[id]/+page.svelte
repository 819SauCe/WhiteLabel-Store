<script>
  import { onMount } from 'svelte';
  import { page } from '$app/stores';
  import { get } from 'svelte/store';

  let isDark = false;
  let product = null;

  onMount(async () => {
    const id = get(page).params.id;
    const res = await fetch("http://localhost:3000/produtos");
    const data = await res.json();
    product = data.find(p => p.id.toString() === id);
    if (product) {
      product = {
        name: product.nome,
        price: product.valor,
        description: product.descricao
      };
    } else {
      product = { name: "Product not found", description: "No details available." };
    }
  });

</script>


{#if product}
<section class:dark={isDark}>
  <div class="container">
    <div class="image-container">
      <img src="https://flowbite.s3.amazonaws.com/blocks/e-commerce/imac-front.svg" alt="iMac" />
    </div>
    <div>
      <h1 class="title">{product.name}</h1>
      <div class="price">R${product.price}</div>
      <div class="stars">
        <span>★</span><span>★</span><span>★</span><span>★</span><span>★</span>
        <span style="color: gray; font-size: 0.875rem;">(5.0)</span>
        <a href="#" style="text-decoration: underline; margin-left: 0.5rem; color: inherit;">345 Reviews</a>
      </div>
      <div class="buttons">
        <a href="#" class="btn btn-outline">Add to favorites</a>
        <a href="#" class="btn btn-fill">Add to cart</a>
      </div>
      <hr />
      <p>{product.description}</p>
    </div>
  </div>
</section>
{:else}
<div class="loader"></div>
{/if}

<style>
  section {
    padding-top: 2rem;
    padding-bottom: 2rem;
    background-color: white;
    font-family: sans-serif;
  }
  .container {
    max-width: 1280px;
    margin: 0 auto;
    padding: 0 1rem;
    display: grid;
    grid-template-columns: 1fr;
    gap: 2rem;
  }
  .image-container {
    max-width: 500px;
    margin: 0 auto;
  }
  .image-container img {
    width: 100%;
  }
  .title {
    font-size: 1.25rem;
    font-weight: 600;
    margin-bottom: 1rem;
  }
  .price {
    font-size: 1.75rem;
    font-weight: 800;
  }
  .stars {
    display: flex;
    gap: 0.25rem;
    color: #FBBF24;
  }
  .buttons {
    display: flex;
    gap: 1rem;
    margin-top: 2rem;
    flex-wrap: wrap;
  }
  .btn {
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 0.75rem 1.25rem;
    font-size: 0.875rem;
    border-radius: 0.5rem;
    cursor: pointer;
    text-decoration: none;
  }
  .btn-outline {
    background: white;
    color: black;
    border: 1px solid #ccc;
  }
  .btn-fill {
    background: #1D4ED8;
    color: white;
    border: none;
  }
  hr {
    margin: 2rem 0;
    border: none;
    border-top: 1px solid #e5e7eb;
  }
  .dark hr {
    border-color: #1f2937;
  }
  p {
    color: #6B7280;
    margin-bottom: 1rem;
  }
  .dark p {
    color: #9CA3AF;
  }
  .loader {
    border: 16px solid #f3f3f3;
    border-radius: 50%;
    border-top: 16px solid #3498db;
    width: 120px;
    height: 120px;
    -webkit-animation: spin 2s linear infinite;
    animation: spin 2s linear infinite;
  }
@-webkit-keyframes spin {
  0% { -webkit-transform: rotate(0deg); }
  100% { -webkit-transform: rotate(360deg); }
}
@keyframes spin {
  0% { transform: rotate(0deg); }
  100% { transform: rotate(360deg); }
}
@media(min-width: 1024px) {
    .container {
      grid-template-columns: 1fr 1fr;
    }
  }
</style>
