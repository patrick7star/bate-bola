
// biblioteca externas.
extern crate fastrand;
extern crate pancurses;
use pancurses::*;

// biblioteca do Rust:
use std::time::{Instant, Duration};

// módulos propriamente implementados.
use super::modelos::{
   Bola, Dimensao, Direcao, Barra,
   Parede
};
use crate::{TRANSPARENTE, TOQUES_LIMITE, VELOCIDADE};
use super::estatisticas::{BarraMetadados, BolaMetadados};

// implementando fora do módulo a função de plotar
// os objetos do jogo:
impl Bola {
   // desenha na tela os bichinhos a serem devorados.
   pub fn plota_bola(&self, tabuleiro:&Window) { 
      // nomeando a coordenada de modo mais legível...
      let l:i32 = self.esqueleto.posicao.y as i32;
      let c:i32 = self.esqueleto.posicao.x as i32;
      // desenhando objeto propriamente...
      init_pair(1, COLOR_RED, TRANSPARENTE);
      tabuleiro.attrset(A_BOLD);
      tabuleiro.color_set(1);
      tabuleiro.mvaddch(l,c,self.esqueleto.forma);
      tabuleiro.attrset(A_NORMAL);
      tabuleiro.color_set(0);
      // plotando alteração.
      tabuleiro.refresh();
   }
}

impl Barra {
   // desenha na tela a cobrinha.
   pub fn plota_barra(&self, tabuleiro:&Window) {
      // apelidando variáveis importantes...
      let l:i32 = self.esqueleto.posicao.y as i32;
      let c:i32 = self.esqueleto.posicao.x as i32;
      // só move respeitando o limite da parede. 
      // string formando barra a ser "impressa".
      let formato:String = {
         self.esqueleto.forma
         .to_string()
         .repeat(self.comprimento as usize)
      };
      // palete de cor:
      init_pair(0, COLOR_WHITE, TRANSPARENTE);
      init_pair(2, COLOR_YELLOW, TRANSPARENTE);
      // desenha.
      tabuleiro.attrset(A_BOLD);
      tabuleiro.color_set(2);
      tabuleiro.mvaddstr(l, c, formato.as_str());
      tabuleiro.color_set(0);
      tabuleiro.attrset(A_NORMAL);
      // plotando alteração.
      tabuleiro.refresh();
   }
}

/* escrevendo simetria reflexiva para o tipo
 * direção. */
impl Direcao {
   pub fn simetrica(&self) -> Self {
      match *self {
         Direcao::Norte => Direcao::Sul,
         Direcao::Sul => Direcao::Norte,
         Direcao::Leste => Direcao::Oeste,
         Direcao::Oeste => Direcao::Leste,
         Direcao::Noroeste => Direcao::Sudoeste,
         Direcao::Nordeste => Direcao::Sudeste,
         Direcao::Sudeste => Direcao::Nordeste,
         Direcao::Sudoeste => Direcao::Noroeste,
      }
   }
}


/* Altera rota da bolinha após colisão
 * levando em conta seu sentido atual,
 * assim com o da barra.
 */
pub fn colisao_bola_barra(bo:&mut Bola, ba:&mut Barra) {
   // verifica se tocou o campo da barra.
   if ba.foi_acertada(bo.esqueleto.posicao) { 
      // apelido com direção atual.
      let sentido = bo.esqueleto.sentido;
      /* aplicando dado viciado ao determinar direção,
       * então 70% das colisão refletem na direção
       * simétrica. */
      if fastrand::u8(1..10) <= 7 {
         bo.esqueleto.sentido = match sentido { 
            Direcao::Sul => {
               // 20% na direção convêncional.
               if fastrand::u8(1..10) <= 8 { 
                  impulsiona_bola(bo, sentido.simetrica())
               }
               // 80% vai precisamente as diagonais.
               else {
                  match fastrand::bool() {
                     true => { 
                        impulsiona_bola(bo, Direcao::Nordeste)
                     },
                     false => { 
                        impulsiona_bola(bo, Direcao::Noroeste)
                     },
                  }
               }
            },
            _ => sentido.simetrica()
         };
      }
      /* 30% dos demais casos; eles serão tratados todos
       * podendo ou não ir na direção "convêncional"
       * ou perpendicular a barra. O "norte" e "sul"
       * tem tratamentos especiais para não permitir
       * um "loop" de rebotes. */
      else {
         bo.esqueleto.sentido = match sentido {
            // tratando colisão superior da barra.
            Direcao::Sudeste => {
               match fastrand::bool() {
                  true => Direcao::Norte,
                  false => impulsiona_bola(bo, Direcao::Noroeste),
               }
            },
            Direcao::Sudoeste => {
               match fastrand::bool() {
                  true => Direcao::Norte,
                  false => impulsiona_bola(bo, Direcao::Nordeste),
               }
            },
            Direcao::Sul => {
               match fastrand::bool() {
                  true => impulsiona_bola(bo, Direcao::Noroeste),
                  false => impulsiona_bola(bo, Direcao::Nordeste),
               }
            },
            // agora da parte inferior...
            Direcao::Nordeste => {
               match fastrand::bool() {
                  false => impulsiona_bola(bo, Direcao::Sul),
                  true => impulsiona_bola(bo, Direcao::Sudoeste),
               }
            },
            Direcao::Noroeste => {
               match fastrand::bool() {
                  false => impulsiona_bola(bo, Direcao::Sul),
                  true => impulsiona_bola(bo, Direcao::Sudeste),
               }
            },
            // para não ficar num laço-infinito cima-baixo.
            Direcao::Norte => {
               match fastrand::bool() {
                  false => impulsiona_bola(bo,Direcao::Sudoeste),
                  true => impulsiona_bola(bo, Direcao::Sudeste),
               }
            },
            // caso contrário direção convencional.
            _ => sentido,
         };
      }
   }
}

/* Representa informações no "rodapé" da 
 * tela, tipo: o tempo de jogo, colisões
 * da bolinha com as paredes; colisão com 
 * a barra, e etc...
 */
pub fn barra_status( brr:&Barra, bl:&Bola, 
janela:&Window, qtd:&u8, qtd_i:&u16, t:Duration) {
   // dimensão da janela.
   let dim = Dimensao {
      altura: janela.get_max_y() as u16,
      largura: janela.get_max_x() as u16
   };
   let _debaixo:bool = {
      brr.esqueleto.posicao.x > 3
   };
   // escrevendo legendas e info sobre o jogo.
   janela.mv( (dim.altura-1) as i32, 0);
   janela.addstr(format!("dimensao do tabuleiro: {}", bl.area));
   janela.addstr(format!( "\ttempo decorrido:{:3.2}seg", t.as_secs()));
   janela.mv( (dim.altura-2) as i32, 0);
   janela.addstr(format!("toques no chao:{:3.3}", *qtd));
   janela.addstr(format!("\tnum. de rebatidas: {}", *qtd_i)); 
}

/* conta a quantia de vezes que a bola bate 
 * no "piso" do tabuleiro, e passa tal valor
 * a referência passada. */
pub fn colisoes_monitoramento(bl:&Bola, brr:&Barra, 
contador:&mut u8, rebatidas:&mut u16) {
   if bl.esqueleto.posicao.y == bl.area.altura-1
      { *contador += 1; }
   if brr.foi_acertada(bl.esqueleto.posicao)
      { *rebatidas += 1; }
}

/* da um impulso na direção para que fica 
 * ainda mais caótica o movimento da bolinha.
 * Envia a direção dado para que possa entrar
 * em 'códigos de desvio' sem precisar alterar
 * mais e gerar muita gambiarra. */
fn impulsiona_bola(bl:&mut Bola, dir:Direcao) -> Direcao{
   // trabalhando dado a direção.
   match dir {
      Direcao::Nordeste | Direcao::Sudeste => {
         // mudando de direção em ante-mão.
         bl.esqueleto.sentido = dir;
         /* seleciona se vai fazer uma curva ou,
          * acelera na direção dada. Ambas opções
          * com 50% de chance de ocorrer, no
          * fim, quanto mais rebatidas, ocorre
          * metade de cada tipo. */
         match fastrand::bool() {
            // curva mais a trajetória.
            true => bl.esqueleto.posicao.x += 1,
            // damos um passo para que assemelhe a aceleração.
            false => bl.r#move(),
         };
         /* e mais um deslocamento a direção horizontal
          * para que no próximo movimento, sem ser 
          * aqui a bola "curve" mais. Porém este
          * encurvamento extra será aleatório(não toda vez). */
         match fastrand::bool() {
            true => { bl.esqueleto.posicao.x += 1; },
            false => (),
         };
      },
      Direcao::Noroeste | Direcao::Sudoeste => {
         bl.esqueleto.sentido = dir;
         // alternativas no cara ou coroa:
         match  fastrand::bool() {
            // curvar mais a direção.
            true => bl.esqueleto.posicao.x -= 1,
            // aplicar uma aceleração.
            false => bl.r#move(),
         };
         // pode ou não entortar mais à trajetória.
         match fastrand::bool() {
            true => { bl.esqueleto.posicao.x -= 1; },
            false => (),
         };
      },
      Direcao::Norte => {
         bl.esqueleto.sentido = dir;
         /* ou acontece uma aceleração, ou 
          * ele desvia um pouco para esquerda. */
         match fastrand::bool() {
            true => bl.esqueleto.posicao.y -= 1,
            false => bl.esqueleto.posicao.x -= 1,
         };
      },
      // as demais, não fazer nada por enquanto...
      _ => (),
   };
   return dir;
}

/* desenha a cobrinha onde quer que ela vá. Com
 * a array de direções que são dado para ela "virar"
 * a cada novo passo. Retorna todos os dados 
 * que foram gerados durante tanta iteração.
 */
pub fn roda_jogo(barra:&mut Barra, bola:&mut Bola,
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
