/* Aqui vamos filtrar e trabalhar na 
 * visualização de todos os dados que 
 * uma partida gera, sejam implicitas,
 * ou caso contrário. */

extern crate pancurses;
use pancurses::*;

// biblioteca do Rust:
use std::time::{Instant, Duration};

// importando da minha biblioteca:
use bate_bola::{Parede, Bola, Ponto, Dimensao, Direcao, Barra};
use bate_bola::estatisticas::*;


 #[test]
 #[ignore]
fn metadados_da_barra() {
   // toda estrutura de um jogo simples.
   let janela = initscr();
   let tabuleiro = newwin(0, 0, 0, 0);
   // configuração:
   tabuleiro.keypad(true);
   tabuleiro.nodelay(true);
   janela.nodelay(true);

   // tempo inicial.
   let t0 = Instant::now();

   // obtendo dimensão do tabuleiro.
   let dim:Dimensao = Dimensao {
      altura: tabuleiro.get_max_y() as u16,
      largura: tabuleiro.get_max_x() as u16,
   };

   let mut barra = Barra::nova(
      15, '=',
      Ponto {x:dim.largura-19, y:dim.altura-5},
      dim
   );
   // criando instância primeiramente....
   let mut dados_brr:BarraMetadados = BarraMetadados::gera(15);
   // isntânciando bolas:
   let mut bola_i = Bola::nova(
      // direção de partida.
      Direcao::Sudoeste,
      // ponto de partida.
      Ponto { x:10, y:8 },
      // dimensão do tabuleiro inserida.
      dim 
   );
   let mut bola_ii = Bola::nova(
      // direção de partida.
      Direcao::Noroeste,
      // ponto de partida.
      Ponto { x:60, y:15 },
      // dimensão do tabuleiro inserida.
      dim 
   );

   // nove segundos de animação.
   while t0.elapsed() < Duration::new(34, 500) {
      let rastro = (barra.esqueleto.posicao, barra.esqueleto.sentido);
      dados_brr.atualiza(&barra, rastro);
      tabuleiro.clear();
      // bolas:
      bola_i.plota_bola(&tabuleiro);
      bola_ii.plota_bola(&tabuleiro);
      // rebote caso bate na barra.
      if barra.foi_acertada(bola_i.esqueleto.posicao) { 
         bola_i.esqueleto.sentido = { 
            bola_i
            .esqueleto
            .sentido
            .simetrica() 
         };
         dados_brr.qtd_rebatidas += 1;
      }
      if barra.foi_acertada(bola_ii.esqueleto.posicao) { 
         bola_ii.esqueleto.sentido = { 
            bola_ii
            .esqueleto
            .sentido
            .simetrica()
         };
         dados_brr.qtd_rebatidas += 1;
      }
      bola_ii.r#move();
      bola_i.r#move();
      // barra:
      barra.r#move(
         match tabuleiro.getch() {
            Some(Input::KeyRight) => { 
               dados_brr.total_comandos_dados += 1;
               Direcao::Leste 
            },
            Some(Input::KeyLeft) => {
               dados_brr.total_comandos_dados += 1;
               Direcao::Oeste
            },
            Some(_) | None =>
               barra.esqueleto.sentido
         }
      );
      barra.plota_barra(&tabuleiro);
      // rastro:
      let r:(Ponto, Direcao) = (
         barra.esqueleto.posicao, 
         barra.esqueleto.sentido
      );
      dados_brr.atualiza(&barra, r);

      // limpa tela.
      janela.refresh();
      tabuleiro.refresh();
      napms(100);
   }
   println!("{}", dados_brr);
   endwin();
   // colocar "true" apenas quando consertar tal colisão.
   assert!(true);
}

#[test]
fn metadados_da_bola() {
   // toda estrutura de um jogo simples.
   let janela = initscr();
   let tabuleiro = newwin(0, 0, 0, 0);
   // configuração:
   tabuleiro.keypad(true);
   tabuleiro.nodelay(true);
   janela.nodelay(true);

   // tempo inicial.
   let t0 = Instant::now();

   // obtendo dimensão do tabuleiro.
   let dim:Dimensao = Dimensao {
      altura: tabuleiro.get_max_y() as u16,
      largura: tabuleiro.get_max_x() as u16,
   };

   let mut barra = Barra::nova(
      15, '#',
      Ponto {x:dim.largura/2 + 1, y:dim.altura/2},
      dim
   );
   // criando instância primeiramente....
   let mut dados_bl:BolaMetadados = BolaMetadados::gera();
   // isntânciando bolas:
   let mut bola = Bola::nova(
      // direção de partida.
      Direcao::Sudoeste,
      // ponto de partida.
      Ponto { x:30, y:5 },
      // dimensão do tabuleiro inserida.
      dim 
   );

   // nove segundos de animação.
   while t0.elapsed() < Duration::new(34, 500) {
      tabuleiro.clear();
      // bolas:
      bola.plota_bola(&tabuleiro);
      // rebote caso bate na barra.
      if barra.foi_acertada(bola.esqueleto.posicao) { 
         bola.esqueleto.sentido = { 
            bola
            .esqueleto
            .sentido
            .simetrica() 
         };
      }
      bola.r#move();
      // se detecta colisão.
      let info = bola.colidiu();
      if info.0 {
         let colisao:(Ponto, Parede, Direcao) = {
            ( bola.esqueleto.posicao, 
               info.1, 
               bola.esqueleto.sentido)
         };
         // colhe dado convieniente.
         dados_bl.atualiza(None, Some(colisao));
      }
      // barra:
      barra.r#move(
         match tabuleiro.getch() {
            Some(Input::KeyRight) => { 
               Direcao::Leste 
            },
            Some(Input::KeyLeft) => {
               Direcao::Oeste
            },
            Some(_) | None =>
               barra.esqueleto.sentido
         }
      );
      barra.plota_barra(&tabuleiro);
      // rastro:
      let rastro = (bola.esqueleto.posicao, bola.esqueleto.sentido);
      dados_bl.atualiza(Some(rastro), None);

      // limpa tela.
      janela.refresh();
      tabuleiro.refresh();
      napms(100);
   }
   println!("{}", dados_bl);
   endwin();
   // colocar "true" apenas quando consertar tal colisão.
   assert!(true);
}
