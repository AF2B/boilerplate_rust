# Boilerplate Rust com Arquitetura MVC
Este é um projeto boilerplate em Rust que tem como objetivo economizar tempo durante a configuração e definição da arquitetura. Ele fornece uma arquitetura Model-View-Controller (MVC) que pode ser facilmente adaptada para uma configuração apenas de API, removendo a camada de visualização. O projeto vem pré-configurado com as seguintes dependências:

- actix-web = "4"
- serde = { version = "1.0", features = ["derive"] }
- serde_json = "1.0"
- uuid = { version = "1.3.2", features = ["serde", "v4"] }
- swagger = loading...
- faker = loading...
- bd = loading...

Recursos adicionais e dependências, como Swagger para documentação da API e Faker para testes em ambiente de desenvolvimento e testes, serão implementados em atualizações futuras.

## Pré-requisitos
Certifique-se de ter o Rust instalado em sua máquina antes de executar este template.

## Instalação
Para configurar o projeto, siga estes passos:

1. Clone o repositório ou baixe os arquivos do projeto.
2. Abra um terminal ou prompt de comando e navegue até o diretório do projeto.

## Uso
Para executar o template, use os seguintes comandos:

1. Execute cargo build para compilar o projeto.
2. Execute cargo run para iniciar o projeto.

## Saída de Dados Personalizada
O projeto oferece um formato personalizado para a saída de dados. Aqui está um exemplo:
```
{
  "data": [
    {
      "created_at": "2023-01-01 00:00:00",
      "email": "test@test.com",
      "id": 10831058695010506497,
      "name": "André Borba",
      "password": "123456",
      "updated_at": "2023-01-01 00:00:00"
    },
    {
      "created_at": "2023-01-01 00:00:00",
      "email": "test@testtest.com",
      "id": 13470027875004843227,
      "name": "Filipe Fonsêca",
      "password": "654321",
      "updated_at": "2023-01-01 00:00:00"
    }
  ],
  "status": "success",
  "itemCount": "number",
  "totalCount": "number",
  "page": "number"
}
```
O campo data contém um array de objetos que representam os dados recuperados. Campos adicionais fornecem informações sobre o status da solicitação, a contagem de itens, o número total de itens e a página atual.

Sinta-se à vontade para personalizar o formato de saída e arquitetura do projeto para atender às suas necessidades específicas.

## Contribuindo
Contribuições são bem-vindas! Sinta-se à vontade para abrir um pull request ou enviar um e-mail para andre.borbaaf2b@gmail.com com sugestões de melhorias.

## Novas Atualizações e Melhorias de Qualidade de Código
Melhorias de qualidade de código e novas atualizações serão implementadas em breve.
