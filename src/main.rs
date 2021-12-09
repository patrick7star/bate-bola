#![allow(unused_variables)]

// minha biblioteca:
use bate_bola::*;
// biblioteca padrão Rust:
use std::time::{Duration, Instant};
// biblioteca externa:
extern crate fastrand;


// velocidade(tempo em miliseg de cada novo quadro).
const VELOCIDADE:i32 = 100;
// quantidade limite de toques no chão.
const TOQUES_LIMITE:u8 = 3;


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

/* desenha a cobrinha onde quer que ela vá. Com
 * a array de direções que são dado para ela "virar"
 * a cada novo passo. Retorna todos os dados 
 * que foram gerados durante tanta iteração.
 */
fn roda_jogo(barra:&mut Barra, bola:&mut Bola,
tabuleiro:&Window, janela:&Window) -> (BarraMetadados, BolaMetadados) {
   // quantia de colisões abaixo.
   let mut toques_no_chao:u8 = 0;
   // quantia de choques com a barra.
   let mut qtd_rebates_barra:u16 = 0;
   // variável para obter tempo.
   let ti:Instant = Instant::now();
   // mensagem de ínicio.
   mensagem_inicio(&tabuleiro, bola.area);
   // coleta de dados.
   let mut dados_brr = BarraMetadados::gera(barra.comprimento as u8);
   let mut dados_bl = BolaMetadados::gera();
   // laço que executa o jogo.
   'unico:loop {
      // derrota.
      colisoes_monitoramento(
         bola, barra,
         &mut toques_no_chao,
         &mut qtd_rebates_barra
      );
      if toques_no_chao > TOQUES_LIMITE 
         { break }
      // apaga "frame" anterior.
      tabuleiro.clear();
      // desenha as bordas do tabuleiro.
      tabuleiro.draw_box(0,0);

      // coletando dados antes do "evento".
      dados_brr.qtd_rebatidas = qtd_rebates_barra;
      dados_brr.atualiza(
         barra,
         (barra.esqueleto.posicao, barra.esqueleto.sentido)
      );
      // renomeando para legibilidade.
      let pos = bola.esqueleto.posicao;
      let sent = bola.esqueleto.sentido;
      let (bateu, parede):(bool, Parede) = bola.colidiu();
      if bateu { 
         // colocar ambos.
         dados_bl.atualiza(
            Some((pos, sent)), 
            Some((pos, parede, sent))
         ); 
      } 
      // apenas colocar localização e vetor-sentido.
      else
         { dados_bl.atualiza(Some((pos, sent)), None); }

      // implemetando rebote caso bate na barra.
      colisao_bola_barra(bola, barra);
      // move a bola e a barra:
      bola.r#move();
         // está baseado na direção dada.
      let instrucao = match tabuleiro.getch() {
         Some(Input::KeyRight) => {
            // pegando comandos dado a barra.
            dados_brr.total_comandos_dados += 1;
            // acelerar se o comando for igual a direção atual.
            if barra.esqueleto.sentido == Direcao::Leste
               { barra.r#move(Direcao::Leste); }
            Direcao::Leste
         },
         Some(Input::KeyLeft) => {
            // contando comandos dado a barra.
            dados_brr.total_comandos_dados += 1;
            // acelerar se o comando for igual a direção atual.
            if barra.esqueleto.sentido == Direcao::Oeste
               { barra.r#move(Direcao::Oeste); }
            Direcao::Oeste
         },
         // também termina o laço.
         Some(Input::Character(ch)) => {
            if ch == 's' { break 'unico }
            else { barra.esqueleto.sentido }
         },
         Some(_) | None =>
            barra.esqueleto.sentido
      };
      barra.r#move(instrucao);
      // desenha bola e barra:
      bola.plota_bola(&tabuleiro);
      barra.plota_barra(&tabuleiro);
      // informação barra de status.
      barra_status(
         barra, bola, janela, 
         &toques_no_chao, 
         &qtd_rebates_barra,
         ti.elapsed()
      );
      // limpa tela.
      janela.refresh();
      tabuleiro.refresh();
      napms(VELOCIDADE);
   }

   // animação de fim de jogo.
   let tempo_animacao:Duration = Duration::new(14,500); 
   let contador:Instant = Instant::now();
   while contador.elapsed() < tempo_animacao {
      // apaga "frame" anterior.
      tabuleiro.clear();
      // mensagem de status do jogo.
      tabuleiro.mv(
         (bola.area.altura/2) as i32, 
         ((bola.area.largura-13)/2) as i32
      );
      // desenha as bordas do tabuleiro.
      tabuleiro.draw_box(0,0);
      // implemetando rebote caso bate na barra.
      colisao_bola_barra(bola, barra);
      // mensagem de termino.
      mensagem_termino(tabuleiro, bola.area);
      // move a bola e a barra:
      bola.r#move();
      barra.r#move(barra.esqueleto.sentido);
      // desenha bola e barra:
      bola.plota_bola(&tabuleiro);
      barra.plota_barra(&tabuleiro);
      // limpa tela.
      janela.refresh();
      tabuleiro.refresh();
      napms(VELOCIDADE);
   }
   // termina ambiente gráfico.
   endwin();
   // retornando dados coletados.
   return (dados_brr, dados_bl);
}


// exibe uma mensagem de termino do jogo. 
fn mensagem_termino(t:&Window, d:Dimensao) {
   // paleta de cor:
   init_pair(3, COLOR_BLUE, TRANSPARENTE);
   // atributos e cores...
   t.attrset(A_BLINK);
   t.attrset(A_BOLD);
   t.color_set(3);
   // o que mostrar.
   t.mv( (d.altura/2) as i32, ((d.largura-13)/2) as i32);
   t.addstr("O jogo acabou!");
   // redefinindo novamente...
   t.color_set(0);
   t.attrset(A_NORMAL);
}

// mensagem de ínicio, para prepara-se do jogo.
fn mensagem_inicio(t:&Window, d:Dimensao) {
   let texto = "o jogo inicia em ... ";
   // paleata de cores.
   init_pair(2, COLOR_RED, TRANSPARENTE);
   t.attrset(A_BOLD);
   t.color_set(2);
   // um segundo(1)
   t.mv(
      (d.altura/2) as i32, 
      (d.largura/2-(texto.len() as u16)/2) as i32
   );
   t.deleteln();
   t.addstr(texto);
   t.addstr("1");
   t.refresh();
   napms(1_000);
   // dois segundos(2)
   t.mv(
      (d.altura/2) as i32, 
      (d.largura/2-(texto.len() as u16)/2) as i32
   );
   t.deleteln();
   t.addstr(texto);
   t.addstr("2");
   t.refresh();
   napms(1_000);
   // três segundos(3)
   t.mv(
      (d.altura/2) as i32, 
      (d.largura/2-(texto.len() as u16)/2) as i32
   );
   t.deleteln();
   t.addstr(texto);
   t.addstr("3");
   t.refresh();
   napms(1_000);
   t.color_set(0);
   t.attrset(A_NORMAL);
}
