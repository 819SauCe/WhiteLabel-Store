<script>
  export let product;
  let fallbackURL = "https://static.vecteezy.com/system/resources/previews/016/026/442/non_2x/empty-shopping-cart-concept-illustration-flat-design-eps10-modern-graphic-element-for-landing-page-empty-state-ui-infographic-icon-vector.jpg";
</script>

<div class="product-card">
  <img src={product.image || fallbackURL} alt={product.name} />
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
