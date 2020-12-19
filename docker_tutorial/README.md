# Instalando e utilizando o Docker

## O que é o docker?

O docker é uma ferramenta, ou melhor, uma suite de ferramentas feita para facilitar a vida do desenvolvedor, no caso, a sua vida! Ele utiliza o conceito de *"containerização"* de aplicações, onde um container guarda todo o código e dependencias do seu projeto, facilitando a execução da sua aplicação em qualquer sistema com o docker engine instalado!

Antes do Docker existir nossa opção de separação da aplicação e suas dependencias da maquina base se restringia ao uso de VMs (Maquinas Virtuais), onde cada vm criada é bem pesada, tanto em execução, quanto espaço. Hoje em dia o docker permite a construção de todo um ambiente de desenvolvimento "on-the-go" com simples comandos e interface, além de garantir o envio e execução de todo o seu codebase em outra maquina com a facilidade de um `docker run`.

![Containers x VM](https://i2.wp.com/www.docker.com/blog/wp-content/uploads/Blog.-Are-containers-..VM-Image-1.png?resize=1024%2C435&ssl=1)
*Diferença arquitetural entre containers e VMs [2]*

No nosso contexto de aprendizagem isso implica em, primeiramente, não encher nossas máquinas com várias dependencias e programas que podem ser inuteis fora do contexto da disciplina. Outro ponto, mais importante, é a capacidade do seu produto/trabalho rodar igualmente em qualquer maquina em que ele é instalado. Isso significa que será mais dificil o seu código quebrar no Mac de seu professor porque você escreveu seu código C na sua máquina Windows. O objetivo mor do docker é acabar com a expressão: *"Na minha máquina roda..."*.

Além de todas essas vantagens, a que mais se sobressai é a grande comunidade por de tras do docker, presente principalmente no [dockerhub](https://hub.docker.com), que contém imagens já prontas para diversas aplicações, principalmente os bancos de dados que estaremos aqui utilizando@

![Exemplo de maquina rodando container com diversas aplicações diferentes](https://i1.wp.com/www.docker.com/blog/wp-content/uploads/011f3ef6-d824-4d43-8b2c-36dab8eaaa72-2.jpg?w=650&ssl=1)
*Exemplo de maquina rodando container com diversas aplicações diferentes [5]*


Para saber mais recomendo a leitura da [pagina de apresentação do docker [1]](https://www.docker.com/why-docker) e entendimento sobre [containers [3]](https://www.docker.com/resources/what-container).

## Instalação

Aqui será explicitado apenas a instalação do docker-engine no ubuntu. Para outras distros: <https://docs.docker.com/engine/install>.

Para o uso do docker-engine no Windows ou Mac será necessário a instalação do docker desktop.  
***ATENÇÂO***: A instalação do docker no Windows 10 só é possível na versão PRO.

### Ubuntu

#### Prerequisitos

Para instalar o docker engine no ubuntu ([retirado da documentação oficial [4]](https://docs.docker.com/engine/install/ubuntu/)), instale as seguintes dependencias:

```sh
sudo apt-get update

sudo apt-get install apt-transport-https ca-certificate curl gnupg-agent software-properties-common
```

Então adicione a chave GPG oficial do Docker:

```sh
curl -fsSL https://download.docker.com/linux/ubuntu/gpg | sudo apt-key add -
```

Para adicionar o repositorio a arvore "stable" selectione:

```sh
sudo add-apt-repository "deb [arch-amd64] https://download.docker.com/linux/ubuntu $(lsb_release -cs) stable"
```

#### Instalação do docker engine

Rode os comandos:

```sh
sudo apt-get update
sudo apt-get installl docker-ce docker-ce-cli containerd.io
```

Verifique a instalação rodando um container pela primeira vez!

```sh
sudo docker run hello-world
```

## Pós instalação em sistemas linux

Para facilitar o uso do docker no seu sistema, retirando a necessidade de usar `sudo` para cada comando docker, sigue os seguintes passos:

Crie o grupo `docker`:

```sh
sudo groupadd docker
```

Adicione seu usuario ao grupo `docker`:

```sh
sudo usermod -aG docker $USER
```

Resete o login de seu user para que as mudanças sejam aplicadas reiniciando sua máquina ou rodando o seguinte comando:

```sh
newgrp docker
```

Teste se o passo a passo funcionou com:

```sh
docker run hello-world
```

## Como iremos utiliza-lo nessa disciplina?

Para a disciplina de banco de dados utilizaremos o docker para principalmente facilitar o uso e instalação do seu SGBD favorito. *A partir daqui todos os comandos docker são feitos com a suposição de que os passos de pós instalação foram feitos.*

Vamos ver o poder do docker mostrando a facilidade de uso da ferramenta!

### mySQL

### postgres

Para levantar um container do postgres com docker, basta rodarmos o seguinte comando:

```sh
docker run --name some-postgres -e POSTGRES_PASSWORD=mysecretpassword -d postgres
```

E caso quisermos algumas versão especifica (no exemplo postgres versão 12 em uma imagem base alpine) do postgres que esteja no [dockerhub](https://hub.docker.com/_/postgres):

```sh
docker run --name some-postgres -e POSTGRES_PASSWORD=mysecretpassword -d postgres:12-alpine
```

A chave `--name` sera o nome do container criado a partir da imagem `-d` postgres. A chave `-e`serve para qualquer variavel de ambiente para a definição do database. No caso para o postgres subir é necessário a chave `POSTGRES_PASSWORD` que será a chave de segurança do seu banco no docker!

Após isso rode o comando ```docker ps``` para checar se ele esta de pé!! Mas espera um pouco.. Como utilizar esse postgres que supostamente foi levantado em minha maquina...? Da forma que esta agora ele esta apartado da minha maquina e a única forma de acessa-lo é via o proprio docker em seu modo interativo com o bash ```docker exec -it some-postgres bash```. Mas aqui nesse caso não consigo fazer muito além de entrar dentro do database com o comando `postgres`.

Para termos acesso ao container de dentro da nossa maquina local, podemos executar:

```sh
docker run --name some-postgres -e POSTGRES_PASSWORD=mysecretpassword -d postgres -p "5432:5432"
```

Dessa forma você conseguira acessar o seu banco postgres a partir da sua maquina local usando qualquer conector que você quiser em qualquer linguagem de programação de interesse!

> *"Mas eu quero um CLI... :cryface:"*

Para isso recomendo a instalação do PGadmin4... NÃAAAO! Estamos usando docker! Nada mais disso de instalar softwares e lotar o HD de forma desnecessária e eventualmente irreversivel. Vamos utilizar o docker do pgadmin4 também!!! Para tal:

```sh
docker pull dpage/pgadmin4
docker run -p 80:80 -p 5432:5432 -p  \
    -e 'PGADMIN_DEFAULT_EMAIL=user@domain.com' \
    -e 'PGADMIN_DEFAULT_PASSWORD=SuperSecret' \
    -d dpage/pgadmin4
```

E pronto! Você terá um CLI no seu navegador de preferencia com acesso ao seu DB.

Mais informações sobre o uso do container postgres em: <https://hub.docker.com/_/postgres>
Documentação do postgres: <https://www.postgresql.org/docs/>
Informações e documentação do pgadmin4: <https://www.pgadmin.org/docs/pgadmin4/latest/container_deployment.html>

Recomendo seguir o próximo tutorial! "Pequeno projeto com flask e postgres usando docker-compose!"  

## CheatSheet

Para buildar uma imagem local:

```shell  
docker build -t ${tag_da_imagem} -f Dockerfile .  # contexto do build
```

Para iniciar um container:

```shell  
docker run ${tag_ou_id_da_imagem} -f Dockerfile .  # contexto do build
```

Para acessar um shell dentro do container:

```shell  
docker exec -it ${id_ou_tag_do_container}
```

## Referências

[1] DOCKER, INC. (Estados Unidos). Docker: Orientation and Setup. In: DOCKER, INC. (Estados Unidos). Docker: Orientation and Setup. [S. l.], [2012-2020]. Disponível em: <https://docs.docker.com/get-started/>. Acesso em: 16 dez. 2020.

[2] FONG, Jenny. Are Containers Replacing Virtual Machines?. In: DOCKER, INC. (Estados Unidos). Are Containers Replacing Virtual Machines?. [S. l.], 30 ago. 2018. Disponível em: <https://www.docker.com/blog/containers-replacing-virtual-machines/>. Acesso em: 16 dez. 2020.

[3] DOCKER, INC. (Estados Unidos). What is a container?. In: DOCKER, INC. (Estados Unidos). What is a container?. [S. l.], [2012-2020]. Disponível em: <https://www.docker.com/resources/what-container>. Acesso em: 16 dez. 2020.

[4] DOCKER, INC. (Estados Unidos). Installing Docker. In: DOCKER, INC. (Estados Unidos). Installing Docker. [S. l.], [2012-2020]. Disponível em: <https://docs.docker.com/engine/install>. Acesso em: 16 dez. 2020.

[5] FONG, Jenny. Docker 101: Introduction to Docker webinar recap. In: DOCKER, INC. (Estados Unidos). Docker 101: Introduction to Docker webinar recap. [S. l.], 3 ago. 2017. Disponível em: <https://www.docker.com/blog/containers-replacing-virtual-machines/>. Acesso em: 16 dez. 2020.
