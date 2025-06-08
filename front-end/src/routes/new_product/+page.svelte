<script>
  let nome = '', marca = '', cor = '', valor = null, desconto = null, descricao = '', categoria_id = null, loja_id = null, registrado_por = null;

  async function salvar() {
    await fetch('http://localhost:3000/produtos/register', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ nome, marca, cor, valor, desconto, descricao, categoria_id, loja_id, registrado_por })
    });
    alert("Produto salvo!");
  }
</script>

<form on:submit|preventDefault={salvar} class="form">
  <input bind:value={nome} placeholder="Nome" />
  <input bind:value={marca} placeholder="Marca" />
  <input bind:value={cor} placeholder="Cor" />
  <input type="number" bind:value={valor} placeholder="Valor" />
  <input type="number" bind:value={desconto} placeholder="Desconto (%)" />
  <textarea bind:value={descricao} placeholder="Descrição" />
  <input type="number" bind:value={categoria_id} placeholder="ID da Categoria" />
  <input type="number" bind:value={loja_id} placeholder="ID da Loja" />
  <input type="number" bind:value={registrado_por} placeholder="ID do Usuário" />
  <button type="submit">Salvar</button>
</form>

<style>
  .form {
    display: flex;
    flex-direction: column;
    gap: 10px;
    max-width: 400px;
    margin: auto;
  }
  input, textarea, button {
    padding: 8px;
    font-size: 16px;
  }
</style>