<footer class="footer">
  <div class="footer-container">
    <div class="footer-top">
      <div class="footer-brand">
        <a href="https://flowbite.com/" class="footer-logo">
          <img src="https://flowbite.com/docs/images/logo.svg" alt="FlowBite Logo" class="footer-logo-img" />
          <span class="footer-logo-text">Flowbite</span>
        </a>
      </div>
      <div class="footer-links">
        <div>
          <h2 class="footer-title">Resources</h2>
          <ul>
            <li><a href="https://flowbite.com/">Flowbite</a></li>
            <li><a href="https://tailwindcss.com/">Tailwind CSS</a></li>
          </ul>
        </div>
        <div>
          <h2 class="footer-title">Follow us</h2>
          <ul>
            <li><a href="https://github.com/themesberg/flowbite">Github</a></li>
            <li><a href="https://discord.gg/4eeurUVvTy">Discord</a></li>
          </ul>
        </div>
        <div>
          <h2 class="footer-title">Legal</h2>
          <ul>
            <li><a href="#">Privacy Policy</a></li>
            <li><a href="#">Terms & Conditions</a></li>
          </ul>
        </div>
      </div>
    </div>
    <hr class="footer-divider" />
    <div class="footer-bottom">
      <span class="footer-copy">© 2023 <a href="https://flowbite.com/">Flowbite™</a>. All Rights Reserved.</span>
      <div class="footer-social">
        <a href="#"><svg class="footer-icon" viewBox="0 0 8 19"><path d="..."/></svg></a>
        <a href="#"><svg class="footer-icon" viewBox="0 0 21 16"><path d="..."/></svg></a>
        <a href="#"><svg class="footer-icon" viewBox="0 0 20 17"><path d="..."/></svg></a>
        <a href="#"><svg class="footer-icon" viewBox="0 0 20 20"><path d="..."/></svg></a>
        <a href="#"><svg class="footer-icon" viewBox="0 0 20 20"><path d="..."/></svg></a>
      </div>
    </div>
  </div>
</footer>

<style>
  .footer {
    margin-top: 2rem;
    background: rgb(28, 28, 28);
    color: #ffffff;
  }

  .footer-container {
    max-width: 1280px;
    margin: auto;
    padding: 1.5rem;
  }

  .footer-top {
    display: flex;
    flex-direction: column;
    gap: 2rem;
  }

  .footer-brand {
    margin-bottom: 1.5rem;
  }

  .footer-logo {
    display: flex;
    align-items: center;
    text-decoration: none;
  }

  .footer-logo-img {
    height: 2rem;
    margin-right: 0.75rem;
  }

  .footer-logo-text {
    font-size: 1.5rem;
    font-weight: 600;
    color: #ffffff;
    white-space: nowrap;
  }

  .footer-links {
    display: contents;
  }

  .footer-title {
    font-size: 1.3rem;
    font-weight: 600;
    text-transform: uppercase;
    margin-bottom: 1.5rem;
    color: #b1b1b1;
  }

  .footer-links ul {
    list-style: none;
    padding: 0;
  }

  .footer-links li {
    margin-bottom: 1rem;
  }

  .footer-links a {
    color: #ffffff;
    text-decoration: none;
  }

  .footer-links a:hover {
    text-decoration: underline;
  }

  .footer-divider {
    margin: 2rem 0;
    border-color: #e5e7eb;
  }

  .footer-bottom {
    display: flex;
    flex-direction: column;
    gap: 1rem;
    align-items: center;
    justify-content: space-between;
  }

  .footer-copy {
    font-size: 0.875rem;
    color: #6b7280;
  }

  .footer-copy a {
    color: #6b7280;
    text-decoration: underline;
  }

  .footer-social {
    display: flex;
    gap: 1.25rem;
  }

  .footer-icon {
    width: 1rem;
    height: 1rem;
    fill: currentColor;
    color: #6b7280;
    transition: color 0.3s;
  }

  .footer-icon:hover {
    color: #111827;
  }

  @media (min-width: 768px) {
    .footer-top {
      flex-direction: row;
      justify-content: space-between;
    }

    .footer-bottom {
      flex-direction: row;
    }
  }
</style>
