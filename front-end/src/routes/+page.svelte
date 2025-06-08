<script>
    import { onMount } from 'svelte';
    import Banner from '../lib/Banner.svelte';
    import ProductCard from '../lib/ProductCard.svelte';

    const certificate1 = "/SSL-certificate.webp";
    const certificate2 = "/Google-safe-Browsing.webp";
    const certificate3 = "/SiteLock.png";
    const certificate4 = "/selo-compre-confie.webp";

    let marks = ['Nike', 'Adidas', 'Puma', 'Reebok'];
    let cores = ['Red', 'Blue', 'Green', 'Black'];

    let filtroMarcas = new Set();
    let filtroCores = new Set();
    let filtroFrete = false;
    let filtroOutlet = false;
    let filtroCupons = new Set();

    let paginaAtual = 1;
    const produtosPorPagina = 40;


    let products = [];

    function toggleFiltro(tipo, valor) {
        if (tipo === 'marca') {
            filtroMarcas = new Set(filtroMarcas);
            filtroMarcas.has(valor) ? filtroMarcas.delete(valor) : filtroMarcas.add(valor);
        } else if (tipo === 'cor') {
            filtroCores = new Set(filtroCores);
            filtroCores.has(valor) ? filtroCores.delete(valor) : filtroCores.add(valor);
        } else if (tipo === 'cupom') {
            filtroCupons = new Set(filtroCupons);
            filtroCupons.has(valor) ? filtroCupons.delete(valor) : filtroCupons.add(valor);
        }
    }

    $: produtosFiltrados = products.filter(p => {
        const marcaOk = filtroMarcas.size === 0 || filtroMarcas.has(p.brand);
        const corOk = filtroCores.size === 0 || filtroCores.has(p.color);
        const freteOk = !filtroFrete || p.frete;
        const outletOk = !filtroOutlet || p.outlet;
        const cupomOk = filtroCupons.size === 0 || [...filtroCupons].some(c => p.cupons.includes(c));
        return marcaOk && corOk && freteOk && outletOk && cupomOk;
    });

    $: totalPaginas = Math.ceil(produtosFiltrados.length / produtosPorPagina);
    $: produtosPaginados = produtosFiltrados.slice((paginaAtual - 1) * produtosPorPagina, paginaAtual * produtosPorPagina);

    onMount(async () => {
        const res = await fetch("http://localhost:3000/produtos");
        const data = await res.json();

        products = data.map(p => ({
            ...p,
            price: p.valor,
            brand: p.marca,
            color: p.cor,
            cupons: [],
            frete: false,
            outlet: false,
            image: "/placeholder.jpg"
        }));

        marks = [...new Set(products.map(p => p.brand))];
        cores = [...new Set(products.map(p => p.color))];
    });
</script>

<!--Certificates-->
<Banner />
<div class="__certificates__container__">
    <div class="__certificates__container__item__">
        <img class="__certificates__container__item__img__" style="width: 9rem; height: 4rem;" src={certificate1} alt="Certificate 1" />
    </div>
    <div class="__certificates__container__item__">
        <img class="__certificates__container__item__img__" style="width: 15rem; height: 5rem;" src={certificate2} alt="Certificate 2" />
    </div>
    <div class="__certificates__container__item__">
        <img class="__certificates__container__item__img__" style="width: 10rem; height: 6rem;" src={certificate3} alt="Certificate 3" />
    </div>
    <div class="__certificates__container__item__">
        <img class="__certificates__container__item__img__" style="width: 10rem; height: 4rem; border-radius: 0.5rem;" src={certificate4} alt="Certificate 4" />
    </div>
</div>

<!--Shop-->
    <div class="titles_products" style="text-align: center;">
        <h1 style="font-size: 2.3rem; margin-top: 1rem;">Produtos</h1>
        <hr style="color: rgba(0, 0, 0, 0.200); margin: auto; width: 70%; margin-top: 1rem; margin-bottom: 1rem;">
    </div>
<!--pagination-->
    {#if totalPaginas > 1}
        <div style="width: 80%; display: flex; justify-content: end; margin-top: 1rem;">
            <select bind:value={paginaAtual}>
                {#each Array(totalPaginas) as _, i}
                    <option value={i + 1}>Página {i + 1}</option>
                {/each}
            </select>
        </div>
    {/if}
<!--Main Content-->
<main>
    <sidebar style="margin-top: 2rem;">

        <div class="filter">
            <h2 style="margin-bottom: 1rem;">PRODUTOS COM</h2>

            <div class="Frete Gratis" style="display: flex; justify-content: space-between; align-items: center; width: 100%;">
                <h3>Frete Gratis</h3>
                <input type="checkbox" class="switch" on:change={e => filtroFrete = e.target.checked}>
            </div>

            <div class="Outlet" style="display: flex; justify-content: space-between; align-items: center; width: 100%;">
                <h3>Outlet</h3>
                <input type="checkbox" class="switch" on:change={e => filtroOutlet = e.target.checked}>
            </div>
            
        </div>


        <div class="filter">
            <h2 style="margin-bottom: 1rem;">TOP OFERTAS</h2>

            <div class="cupom1" style="display: flex; justify-content: space-between; align-items: center; width: 100%;">
                <h3>PRIMEIRACOMPRA</h3>
                <input type="checkbox" class="switch" on:change={() => toggleFiltro('cupom', 'PRIMEIRACOMPRA')}>
            </div>
            <div class="cupom2" style="display: flex; justify-content: space-between; align-items: center; width: 100%;">
                <h3>CAMISETAS22</h3>
                <input type="checkbox" class="switch" on:change={() => toggleFiltro('cupom', 'CAMISETAS22')}>
            </div>
            <div class="cupom3" style="display: flex; justify-content: space-between; align-items: center; width: 100%;">
                <h3>CUSTO0</h3>
                <input type="checkbox" class="switch" on:change={() => toggleFiltro('cupom', 'CUSTO0')}>
            </div>
        </div>

        <div class="filter">
            <h2>MARCAS</h2>

            <div style="width: 100%; padding: 0 1rem;">
                <input type="text" placeholder="Buscar por marca" style="width: 100%; border: 1px solid black; padding: 0.5rem; margin-bottom: 1rem;">
                
                <div class="mark" style="display: flex; flex-direction: column; gap: 0.5rem;">
                    {#each marks as mark}
                        <label style="display: flex; align-items: center; gap: 0.5rem;">
                            <input type="checkbox" on:change={() => toggleFiltro('marca', mark)}>
                            {mark}
                        </label>
                    {/each}
                </div>
            </div>
        </div>

        <div class="filter">
            <h2>CORES</h2>

            <div style="width: 100%; padding: 0 1rem;">                
                <div class="cores" style="display: flex; flex-direction: column; gap: 0.5rem;">
                    {#each cores as cor}
                        <label style="display: flex; align-items: center; gap: 0.5rem;">
                            <input type="checkbox" on:change={() => toggleFiltro('cor', cor)}>
                            {cor}
                        </label>
                    {/each}
                </div>
            </div>

        </div>
    </sidebar>

    <!--Products-->
    <div class="products" style="width: 65%; gap: 3rem; display: flex; flex-wrap: wrap; margin-top: 1.5rem;">
        {#each produtosPaginados as product}
            <ProductCard product={product} />
        {/each}
    </div>
</main>
{#if totalPaginas > 1}
        <div style="width: 100%; display: flex; justify-content: center; gap: 0.5rem; margin-top: 2rem; margin-bottom: 2rem;">
            <button on:click={() => paginaAtual = Math.max(paginaAtual - 1, 1)} disabled={paginaAtual === 1}>&lt;</button>
            {#each Array(totalPaginas) as _, i}
                <button on:click={() => paginaAtual = i + 1} class:selected={paginaAtual === i + 1}>{i + 1}</button>
            {/each}
            <button on:click={() => paginaAtual = Math.min(paginaAtual + 1, totalPaginas)} disabled={paginaAtual === totalPaginas}>&gt;</button>
        </div>
    {/if}

<style>
    .__certificates__container__ {
        display: flex;
        gap: 10rem;
        width: 100%;
        height: 5rem;
        justify-content: center;
        align-items: center;
        padding: 0;
        margin: 0;
        background-color: rgb(200, 200, 200);
    }
    main {
        display: flex;
        align-items: flex-start;
        gap: 2rem;
    }
    .filter {
        display: flex;
        flex-direction: column;
        justify-content: center;
        align-items: center;
        width: 80%;
        height: auto;
        margin-top: 1.5rem;
        margin-left: 2rem;
        margin-bottom: 0;
        border: 1px solid rgba(0, 0, 0, 0.200);
        padding: 0.75rem 1rem;
    }
    .switch {
        appearance: none;
        position: relative;
        background: rgb(182, 187, 194);
        width: 1.5rem;
        height: 0.625rem;
        border-radius: 0.4375rem;
        cursor: pointer;
        transition: background 200ms, left 200ms;
        will-change: background, left;
    }
    .switch::after {
        position: absolute;
        content: "";
        width: 0.875rem;
        height: 0.875rem;
        background: rgb(242, 243, 244);
        box-shadow: rgba(0, 0, 0, 0.12) 0px 0px 1px, rgba(0, 0, 0, 0.25) 0px 1px 1px;
        border-radius: 50%;
        left: -0.125rem;
        top: -0.125rem;
        transition: background 200ms, left 200ms;
        will-change: background, left;
    }
    .switch:checked {
        background: rgb(255, 169, 115);
    }
    .switch:checked::after {
        left: 1.125rem;
    }
</style>