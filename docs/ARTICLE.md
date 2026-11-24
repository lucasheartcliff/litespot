# Litespot: tentei fazer um Spotify leve e encerrei o projeto depois de medir

*Um webview do Spotify inspirado no Dorion, e o que os números disseram.*

## A inspiração: Dorion

Tudo começou com o [Dorion](https://github.com/SpikeHD/Dorion), um cliente
alternativo do Discord feito em Rust com Tauri. Em vez de embarcar um Chromium
inteiro, como o app oficial, ele usa o webview do próprio sistema operacional
(WebView2 no Windows, WebKitGTK no Linux, WKWebView no macOS). O projeto se
posiciona para PCs mais modestos e para quem se importa com espaço em disco, e
ainda acrescenta temas e plugins
([README](https://github.com/SpikeHD/Dorion/blob/main/README.md),
[LinuxLinks](https://www.linuxlinks.com/dorion-lightweight-discord-web-client/)).

A pergunta que ficou na minha cabeça foi simples: **dá para fazer a mesma coisa
com o Spotify?** O cliente desktop oficial também carrega um navegador embutido.
Se um webview nativo funciona bem para o Discord, talvez funcionasse para a
música.

## A ideia

Um app pequeno em Rust + Tauri, usando o WebView2 do Windows, que desse a mesma
experiência do Spotify gastando menos disco, menos memória e menos CPU.

## Primeiro caminho: um cliente próprio

Comecei do jeito "certo": uma interface em Svelte, login por OAuth PKCE, tokens
no Gerenciador de Credenciais do Windows, a Web API do Spotify servida pelo
host em Rust e o Web Playback SDK tocando o áudio. Isso exige registrar um app
no painel de desenvolvedores do Spotify, ter uma conta Premium e manter uma
interface inteira. Era bastante código para um projeto cujo objetivo era ser
*leve*.

## Segundo caminho: só um webview

Mudei para o modelo do Dorion: uma janela nativa que carrega
`open.spotify.com`. O login e a reprodução ficam por conta do próprio site. O
Rust cuida só da janela, das teclas de mídia (clicando nos botões do player) e
de restringir a navegação aos domínios do Spotify. O resultado foi um projeto
de poucas centenas de linhas, sem Client ID, sem OAuth e com um binário de 3 MB.
O cliente antigo ficou guardado num branch de arquivo.

## Medindo de verdade

Como a promessa era "leve", montei um benchmark em PowerShell comparando o
Litespot com o Spotify oficial na mesma máquina (Windows 11, 20 núcleos), somando
a árvore inteira de processos de cada app.

O benchmark em si tinha bugs que invalidavam as primeiras medições:

- a árvore de processos parava no processo raiz (uma comparação entre tipos
  numéricos diferentes no PowerShell), então só um processo era medido;
- a limpeza matava **todos** os processos do WebView2 da máquina, de qualquer app;
- em máquinas com vírgula decimal, `968,2` virava `9682` no relatório.

Depois de corrigir tudo, estes foram os números (medianas):

| Métrica | Litespot | Spotify oficial |
|---|---|---|
| Instalado em disco | **3 MB** | 436 MB |
| Startup até a janela visível | **229 ms** | 1031 ms |
| Memória (working set), idle | ~968 MB | ~954 MB |
| Memória privada, idle | ~625 MB | ~710 MB |
| CPU em idle | ~5% | ~0,2% |

## O que os números disseram

- **Disco e startup:** vitória clara. Cerca de 145 vezes menor e 4,5 vezes mais
  rápido para mostrar a janela.
- **Memória:** empate. O host em Rust usa só ~24 MB. O resto são oito processos do
  WebView2 rodando o player web, que eu não controlo.
- **CPU:** derrota. Cerca de dez vezes mais CPU parado do que o app oficial. O
  player web dentro do WebView2 continua trabalhando mesmo sem foco.

Ou seja, o que o Dorion consegue para o Discord, no Spotify só se confirmou para
o instalador e a abertura. Para um app que fica aberto o dia inteiro, o custo de
execução é o que importa, e ali não ganhei.

## Por que encerrei

Existiam ideias de otimização, como reduzir os processos do WebView2,
desligar o isolamento de sites e devolver memória quando a janela é minimizada.
Mas elas eram palpites que ainda precisariam de testes cuidadosos de reprodução e
do Widevine, e o ganho era incerto. Além disso, o projeto depende de um site que o
Spotify pode mudar ou restringir quando quiser: as teclas de mídia usam seletores
do DOM, e o login por terceiros pode ser recusado num webview embutido.

Somando tudo, eu veria um esforço grande para um resultado que não seria
relevante: ganhar em tamanho de instalação e abertura, empatar em memória e
perder em CPU. Decidi parar aqui e arquivar o repositório.

## O que ficou sem medir

Sendo honesto sobre os limites:

- não medi memória e CPU **tocando música** (o DRM não se automatiza);
- não consegui confirmar que o Litespot estava logado na segunda rodada;
- não verifiquei se o áudio protegido (Widevine) toca de fato no app instalado;
- os seletores das teclas de mídia não foram checados no site logado;
- o tempo de startup de 229 ms pode contar a janela antes de a página renderizar.

## Lições

1. **Meça antes de construir.** Um benchmark de meia hora teria dito que o
   ganho estava só no disco e na abertura.
2. **Teste a ferramenta de teste.** O harness tinha cinco bugs, e os primeiros
   números pareciam bons justamente porque estavam errados.
3. **Um wrapper só vale se o app original for o problema.** No Dorion, o
   diferencial vai além do tamanho: temas e plugins. Um webview puro do Spotify
   não acrescentava nada além da casca nativa.
4. **Arquivar é um resultado válido.** O código, os dados e as decisões estão no
   repositório para quem quiser retomar.

## Onde está tudo

O código, os resultados completos, os dados brutos e as decisões (ADRs) estão no
repositório do projeto, em `docs/RESULTS.md`, `docs/results/` e `docs/adr/`. O
cliente próprio original está no branch `archive/custom-client`.

**Fontes sobre o Dorion:** [repositório](https://github.com/SpikeHD/Dorion) ·
[README](https://github.com/SpikeHD/Dorion/blob/main/README.md) ·
[LinuxLinks](https://www.linuxlinks.com/dorion-lightweight-discord-web-client/)
