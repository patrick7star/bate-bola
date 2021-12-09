
// biblioteca externa:
extern crate pancurses;
use pancurses::*;

// biblioteca do Rust:
use std::time::{Instant, Duration};

// importando da minha biblioteca:
use bate_bola::{Ponto, Dimensao, Direcao, Bola};


#[test]
//#[ignore]
fn rebatida_na_parede_esquerda() {
   let janela = initscr();
   let tabuleiro = newwin(0, 0, 0, 0);

   // tempo inicial.
   let t0 = Instant::now();

   // obtendo dimensão do tabuleiro.
   let tabuleiro_dim:Dimensao = Dimensao {
      altura: tabuleiro.get_max_y() as u16,
      largura: tabuleiro.get_max_x() as u16,
   };

   let mut bola = Bola::nova(
      // direção de partida.
      Direcao::Noroeste,
      // ponto de partida.
      Ponto { x:9, y:tabuleiro_dim.altura-3 },
      // dimensão do tabuleiro inserida.
      tabuleiro_dim 
   );

   // nove segundos de animação.
   while t0.elapsed() < Duration::new(9, 100) {
      // escreve informação necessária.
      let tipo_colisao = bola.colidiu().1;
      let _str:String = {
         format!(
            "{:#?}, {:#?}", 
            bola.esqueleto.sentido,
            tipo_colisao, 
         )
      };
      tabuleiro.mvaddstr( 0, 0, _str.as_str());
      // desenha bola na tela, e a move.
      bola.plota_bola(&tabuleiro);
      bola.r#move();

      // limpa tela.
      janela.refresh();
      tabuleiro.refresh();
      napms(600);
   }
   endwin();
   // colocar "true" apenas quando consertar tal colisão.
   assert!(true);
}

#[test]
//#[ignore]
fn rebote_no_teto() {
   let janela = initscr();
   let tabuleiro = newwin(0, 0, 0, 0);

   // tempo inicial.
   let t0 = Instant::now();

   // obtendo dimensão do tabuleiro.
   let tabuleiro_dim:Dimensao = Dimensao {
      altura: tabuleiro.get_max_y() as u16,
      largura: tabuleiro.get_max_x() as u16,
   };

   let mut bola = Bola::nova(
      // direção de partida.
      Direcao::Nordeste,
      // ponto de partida.
      Ponto { x:7, y:8 },
      // dimensão do tabuleiro inserida.
      tabuleiro_dim 
   );

   // nove segundos de animação.
   while t0.elapsed() < Duration::new(5, 100) {
      // escreve informação necessária.
      let tipo_colisao = bola.colidiu().1;
      let _str:String = {
         format!(
            "{:#?}, {:#?}", 
            bola.esqueleto.sentido,
            tipo_colisao, 
         )
      };
      tabuleiro.mvaddstr( 
         (tabuleiro_dim.altura-2) as i32,
         (tabuleiro_dim.largura-30) as i32, 
         _str.as_str()
      );
      // desenha bola na tela, e a move.
      bola.plota_bola(&tabuleiro);
      bola.r#move();

      // limpa tela.
      janela.refresh();
      tabuleiro.refresh();
      napms(200);
   }
   endwin();
   // colocar "true" apenas quando consertar tal colisão.
   assert!(true);
}

#[test]
//#[ignore]
fn rebote_no_lado_direito() {
   let janela = initscr();
   let tabuleiro = newwin(0, 0, 0, 0);

   // tempo inicial.
   let t0 = Instant::now();

   // obtendo dimensão do tabuleiro.
   let tabuleiro_dim:Dimensao = Dimensao {
      altura: tabuleiro.get_max_y() as u16,
      largura: tabuleiro.get_max_x() as u16,
   };

   let mut bola = Bola::nova(
      // direção de partida.
      Direcao::Sudeste,
      // ponto de partida.
      Ponto { x:tabuleiro_dim.largura-10, y:3 },
      // dimensão do tabuleiro inserida.
      tabuleiro_dim 
   );

   // nove segundos de animação.
   while t0.elapsed() < Duration::new(4, 500) {
      // escreve informação necessária.
      let tipo_colisao = bola.colidiu().1;
      let _str:String = {
         format!(
            "{:#?}, {:#?}", 
            bola.esqueleto.sentido,
            tipo_colisao, 
         )
      };
      tabuleiro.mvaddstr( 
         (tabuleiro_dim.altura-2) as i32,
         (tabuleiro_dim.largura-30) as i32, 
         _str.as_str()
      );
      // desenha bola na tela, e a move.
      bola.plota_bola(&tabuleiro);
      bola.r#move();

      // limpa tela.
      janela.refresh();
      tabuleiro.refresh();
      napms(200);
   }
   endwin();
   // colocar "true" apenas quando consertar tal colisão.
   assert!(true);
}

#[test]
//#[ignore]
fn rebote_no_lado_chao() {
   let janela = initscr();
   let tabuleiro = newwin(0, 0, 0, 0);

   // tempo inicial.
   let t0 = Instant::now();

   // obtendo dimensão do tabuleiro.
   let tabuleiro_dim:Dimensao = Dimensao {
      altura: tabuleiro.get_max_y() as u16,
      largura: tabuleiro.get_max_x() as u16,
   };

   let mut bola = Bola::nova(
      // direção de partida.
      Direcao::Sudoeste,
      // ponto de partida.
      Ponto { x:tabuleiro_dim.largura-45, y:16 },
      // dimensão do tabuleiro inserida.
      tabuleiro_dim 
   );

   // nove segundos de animação.
   while t0.elapsed() < Duration::new(4, 500) {
      // escreve informação necessária.
      let tipo_colisao = bola.colidiu().1;
      let _str:String = {
         format!(
            "{:#?}, {:#?}", 
            bola.esqueleto.sentido,
            tipo_colisao, 
         )
      };
      tabuleiro.mvaddstr( 
         (tabuleiro_dim.altura-2) as i32,
         (tabuleiro_dim.largura-30) as i32, 
         _str.as_str()
      );
      // desenha bola na tela, e a move.
      bola.plota_bola(&tabuleiro);
      bola.r#move();

      // limpa tela.
      janela.refresh();
      tabuleiro.refresh();
      napms(200);
   }
   endwin();
   // colocar "true" apenas quando consertar tal colisão.
   assert!(true);
}

#[test]
//#[ignore]
fn rebotes_lineares() {
   let janela = initscr();
   let tabuleiro = newwin(0, 0, 0, 0);

   // tempo inicial.
   let t0 = Instant::now();

   // obtendo dimensão do tabuleiro.
   let dim:Dimensao = Dimensao {
      altura: tabuleiro.get_max_y() as u16,
      largura: tabuleiro.get_max_x() as u16,
   };

   let mut bola_i = Bola::nova(
      // direção de partida.
      Direcao::Norte,
      // ponto de partida.
      Ponto { x:15, y:dim.altura-8 },
      // dimensão do tabuleiro inserida.
      dim 
   );

   let mut bola_ii = Bola::nova(
      // direção de partida.
      Direcao::Oeste,
      // ponto de partida.
      Ponto { x:dim.largura-35, y:dim.altura/2 },
      // dimensão do tabuleiro inserida.
      dim 
   );

   // nove segundos de animação.
   while t0.elapsed() < Duration::new(30, 500) {
      tabuleiro.clear();
      // escreve informação necessária.
      // info da bola(1).
      let _str:String = {
         format!(
            "BOLA(I): {:#?}, {:#?}", 
            bola_i.esqueleto.sentido,
            bola_i.colidiu().1, 
         )
      };
      tabuleiro.mvaddstr( 
         (dim.altura-2) as i32,
         (dim.largura-30) as i32, 
         _str.as_str()
      );
      // info da bola(2).
      let _str:String = {
         format!(
            "BOLA(II): {:#?}, {:#?}", 
            bola_ii.esqueleto.sentido,
            bola_ii.colidiu().1, 
         )
      };
      tabuleiro.mvaddstr( 
         1, (dim.largura-30) as i32, 
         _str.as_str()
      );
      // desenha bolas na tela, e as move.
      bola_i.plota_bola(&tabuleiro);
      bola_i.r#move();

      bola_ii.plota_bola(&tabuleiro);
      bola_ii.r#move();

      // limpa tela.
      janela.refresh();
      tabuleiro.refresh();
      napms(200);
   }
   endwin();
   // colocar "true" apenas quando consertar tal colisão.
   assert!(true);
}


#[test]
//#[ignore]
fn batendo_nas_paredes_continuamente() {
   let janela = initscr();
   let tabuleiro = newwin(0, 0, 0, 0);

   // tempo inicial.
   let t0 = Instant::now();

   // obtendo dimensão do tabuleiro.
   let dim:Dimensao = Dimensao {
      altura: tabuleiro.get_max_y() as u16,
      largura: tabuleiro.get_max_x() as u16,
   };

   let mut bola = Bola::nova(
      // direção de partida.
      Direcao::Sudoeste,
      // ponto de partida.
      Ponto { x:dim.largura-55, y:16 },
      // dimensão do tabuleiro inserida.
      dim 
   );

   // nove segundos de animação.
   while t0.elapsed() < Duration::new(54, 500) {
      /* ocultando estatus
      // escreve informação necessária.
      let _str:String = {
         format!(
            "{:#?}, {:#?}", 
            bola.esqueleto.sentido,
            bola.colidiu().1
         )
      };
      tabuleiro.mvaddstr( 
         (dim.altura-2) as i32,
         (dim.largura-30) as i32, 
         _str.as_str()
      ); */
      // desenha bola na tela, e a move.
      bola.plota_bola(&tabuleiro);
      bola.r#move();

      // limpa tela.
      janela.refresh();
      tabuleiro.refresh();
      napms(100);
   }
   endwin();
   // colocar "true" apenas quando consertar tal colisão.
   assert!(true);
}


