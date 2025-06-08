<script>
  export let product;
</script>

<div class="product-card">
  <img src={product.image} alt={product.name} />
  <h2>{product.name}</h2>
  <p>R$ {product.price.toFixed(2)}</p>
</div>

<style>
  .product-card {
    border: 1px solid #ccc;
    border-radius: 8px;
    padding: 1rem;
    text-align: center;
    width: 200px;
  }
  .product-card img {
    width: 100%;
    height: auto;
  }
</style>
