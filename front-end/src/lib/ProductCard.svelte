<script>
  export let product;

  let product_url = `/product/${product.id}`;
</script>

<div class="container">
  <a class="image-wrapper" href={product_url}>
    <img class="product-image" src={product.image} alt={product.name} />
    <span class="discount-badge">39% OFF</span>
  </a>
  <div class="content">
    <a href={product_url}>
      <h5 class="title">{product.name}</h5>
    </a>
    <div class="price-rating">
      <p>
        <span class="price">R${product.price.toFixed(2)}</span>
        <span class="old-price">R${product.oldPrice.toFixed(2)}</span>
      </p>
      <div class="stars">
        {#each Array(5) as _, i}
          <svg aria-hidden="true" class="star" fill="currentColor" viewBox="0 0 20 20">
            <path d="M9.049 2.927c.3-.921 1.603-.921 1.902 0l1.07 3.292a1 1 0 00.95.69h3.462c.969 0 
              1.371 1.24.588 1.81l-2.8 2.034a1 1 0 00-.364 1.118l1.07 3.292c.3.921-.755 
              1.688-1.54 1.118l-2.8-2.034a1 1 0 00-1.175 0l-2.8 
              2.034c-.784.57-1.838-.197-1.539-1.118l1.07-3.292a1 1 
              0 00-.364-1.118L2.98 8.72c-.783-.57-.38-1.81.588-1.81h3.461a1 1 
              0 00.951-.69l1.07-3.292z" />
          </svg>
        {/each}
        <span class="rating">5.0</span>
      </div>
    </div>
    <a href={product_url} class="cart-button">
      <svg xmlns="http://www.w3.org/2000/svg" class="cart-icon" fill="none" viewBox="0 0 24 24" stroke="currentColor">
        <path stroke-linecap="round" stroke-linejoin="round" d="M3 3h2l.4 2M7 13h10l4-8H5.4M7 13L5.4 
          5M7 13l-2.293 2.293c-.63.63-.184 1.707.707 
          1.707H17m0 0a2 2 0 100 4 2 2 0 000-4zm-8 
          2a2 2 0 11-4 0 2 2 0 014 0z" />
      </svg>
      Add to cart
    </a>
  </div>
</div>

<style>
  .container {
    position: relative;
    display: flex;
    flex-direction: column;
    width: 18rem;
    max-width: 20rem;
    overflow: hidden;
    border-radius: 0.5rem;
    border: 1px solid #e5e7eb;
    background-color: #fff;
    box-shadow: 0 1px 2px rgba(0,0,0,0.1);
  }

  .image-wrapper {
    position: relative;
    margin: 0.75rem;
    height: 15rem;
    overflow: hidden;
    border-radius: 1rem;
    display: flex;
  }

  .product-image {
    object-fit: cover;
    width: 100%;
    height: 100%;
  }

  .discount-badge {
    position: absolute;
    top: 0;
    left: 0;
    margin: 0.5rem;
    background-color: black;
    color: white;
    font-size: 0.875rem;
    font-weight: 500;
    padding: 0 0.5rem;
    border-radius: 9999px;
  }

  .content {
    margin-top: 1rem;
    padding: 0 1.25rem 1.25rem;
  }

  .title {
    font-size: 1.25rem;
    color: #0f172a;
    letter-spacing: -0.01em;
  }

  .price-rating {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin: 0rem 0rem 0.3rem 0rem;
  }

  .price {
    font-size: 1.3rem;
    font-weight: bold;
    color: #0f172a;
  }

  .old-price {
    font-size: 0.875rem;
    color: #0f172a;
    text-decoration: line-through;
  }

  .stars {
    display: flex;
    align-items: center;
    margin-top: 2rem;
  }

  .star {
    width: 1.25rem;
    height: 1.25rem;
    color: #facc15;
  }

  .rating {
    margin-left: 0.75rem;
    background-color: #fde68a;
    font-size: 0.75rem;
    font-weight: 600;
    padding: 0.125rem 0.625rem;
    border-radius: 0.25rem;
  }

  .cart-button {
    display: flex;
    justify-content: center;
    align-items: center;
    background-color: #0f172a;
    color: white;
    padding: 0.625rem 1.25rem;
    font-size: 0.875rem;
    font-weight: 500;
    border-radius: 0.375rem;
    text-align: center;
    text-decoration: none;
    transition: background-color 0.3s;
  }

  .cart-button:hover {
    background-color: #374151;
  }

  .cart-icon {
    width: 1.5rem;
    height: 1.5rem;
    margin-right: 0.5rem;
  }
</style>