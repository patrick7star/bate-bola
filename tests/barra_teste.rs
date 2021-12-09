
// biblioteca externa:
extern crate pancurses;
use pancurses::*;

// biblioteca do Rust:
use std::time::{Instant, Duration};

// importando da minha biblioteca:
use bate_bola::{Bola, Ponto, Dimensao, Direcao, Barra};


#[test]
fn controlador_teste() {
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
      7, '=',
      Ponto {x:dim.largura-19, y:dim.altura/2},
      dim
   );

   // nove segundos de animação.
   while t0.elapsed() < Duration::new(14, 500) {
      tabuleiro.clear();
      // escreve informação necessária.
      let _str:String = {
         format!(
            "{:#?}, colidiu?{}", 
            barra.esqueleto.posicao,
            barra.colidiu_na_parede()
         )
      };
      tabuleiro.mvaddstr( 
         (dim.altura-2) as i32,
         (dim.largura-30) as i32, 
         _str.as_str()
      ); 
      // desenha bola na tela, e a move.
      barra.plota_barra(&tabuleiro);
      barra.r#move(
         match tabuleiro.getch() {
            Some(Input::KeyRight) =>
               Direcao::Leste,
            Some(Input::KeyLeft) =>
               Direcao::Oeste,
            Some(_) | None =>
               barra.esqueleto.sentido
         }
      );

      // limpa tela.
      janela.refresh();
      tabuleiro.refresh();
      napms(100);
   }
   endwin();
   // colocar "true" apenas quando consertar tal colisão.
   assert!(true);
}

#[test]
fn testando_colisao_na_barra() {
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
      tabuleiro.clear();
      /*
      // escreve informação necessária.
      let _str:String = {
         format!(
            "{:#?}, colidiu?{}", 
            barra.esqueleto.posicao,
            barra.colidiu_na_parede()
         )
      };
      tabuleiro.mvaddstr( 
         (dim.altura-2) as i32,
         (dim.largura-30) as i32, 
         _str.as_str()
      ); 
      */
      // desenha bola e barra na tela, e a move.
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
      }
      if barra.foi_acertada(bola_ii.esqueleto.posicao) { 
         bola_ii.esqueleto.sentido = { 
            bola_ii
            .esqueleto
            .sentido
            .simetrica()
         };
      }
      bola_ii.r#move();
      bola_i.r#move();
      // barra:
      barra.r#move(
         match tabuleiro.getch() {
            Some(Input::KeyRight) =>
               Direcao::Leste,
            Some(Input::KeyLeft) =>
               Direcao::Oeste,
            Some(_) | None =>
               barra.esqueleto.sentido
         }
      );
      barra.plota_barra(&tabuleiro);

      // limpa tela.
      janela.refresh();
      tabuleiro.refresh();
      napms(100);
   }
   endwin();
   // colocar "true" apenas quando consertar tal colisão.
   assert!(true);
}
