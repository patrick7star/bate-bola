#![allow(unused_variables)]

// minha biblioteca:
use bate_bola::*;


// execução de testes...
fn main() {
   /* ativando unicode characteres...
   let local = LcCategory::all;
   setlocale(local, "pt.UTF-8"); */

   // instanciando elementos da janela.
   let janela = initscr();
   // dimensão da janela.
   let dim_j:Dimensao = Dimensao {
      altura: janela.get_max_y() as u16,
      largura: janela.get_max_x() as u16,
   };
   // criando tela de tabuleiro...
   let tabuleiro = newwin(
      (dim_j.altura-3) as i32, 
      dim_j.largura as i32, 
      0, 0
   );
   // obtendo dimensão do tabuleiro.
   let dim:Dimensao = Dimensao {
      altura: tabuleiro.get_max_y() as u16,
      largura: tabuleiro.get_max_x() as u16,
   };

   // configuração da janela:
   tabuleiro.keypad(true);
   tabuleiro.nodelay(true);
   janela.nodelay(true);
   curs_set(0);
   noecho();
   // inicia coloração.
   start_color();
   use_default_colors();

   let mut barra = Barra::nova(
      fastrand::u16(3..10), '=',
      Ponto {
         x:dim.largura/2, 
         y:dim.altura-5
      },
      dim
   );
   // isntânciando bolas:
   let mut bola = Bola::nova(
      // direção de partida.
      Direcao::Sudoeste,
      // ponto de partida.
      Ponto { 
         x: fastrand::u16(2..dim.largura-2), 
         y: fastrand::u16(1..dim.altura-7) 
      },
      // dimensão do tabuleiro inserida.
      dim 
   );
   
   // executando o jogo...
   let dados = roda_jogo(
      &mut barra, &mut bola, 
      &tabuleiro, &janela
   );
   println!(
      "--- Dados da Barra --- \n{}\n\n--- Dados da Bola ---\n{}\n",
      dados.0, dados.1
   );

   // gravando metadados da partida.
   banco_de_dados::salva(dados.0, dados.1);
}
