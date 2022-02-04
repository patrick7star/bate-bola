
// importando modelos:
mod modelos;
pub use self::modelos::*;
pub mod estatisticas;
pub use self::estatisticas::*;
pub mod banco_de_dados;

// biblioteca externa:
extern crate pancurses;
extern crate fastrand;

// oculta documentação da "exportação".
#[doc(hidden)]
pub use pancurses::*;

// biblioteca do Rust:
use std::time::Duration;

// cor transparente:
pub const TRANSPARENTE:i16 = -1;
// velocidade(tempo em miliseg de cada novo quadro).
pub const VELOCIDADE:i32 = 100;
// quantidade limite de toques no chão.
pub const TOQUES_LIMITE:u8 = 3;


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


/**
 Aqui vamos criar um simulador que roda
 como se desenvolveu as partidas realizadas
 anteriormente.
 */

/* Cede um método para o tipo de dado, onde
 * ele segue a posição injetada, independente 
 * como interaje com os demais objetos da tela. */
trait MovimentoRestrito {
   /* faz o movimento, independente da colisão com 
    * qualquer outro objeto em tela. */
   fn move_a_posicao(&mut self, nova_posicao:Ponto); 
}

// implementando o trait para o tipo 'Bola'.
impl MovimentoRestrito for Bola {
   fn move_a_posicao(&mut self, nova_posicao:Ponto) { 
      self.esqueleto.posicao = nova_posicao;
   }
}

// mesma implementação para o tipo 'Barra'.
impl MovimentoRestrito for Barra {
   fn move_a_posicao(&mut self, nova_posicao:Ponto) { 
      self.esqueleto.posicao = nova_posicao;
   }
}


// computa a possível dimensão do "tabuleiro" do jogo.
fn acha_dimensao_tabuleiro(dados_i:&BarraMetadados, dados_ii:&BolaMetadados)
-> Dimensao {
   // váriaveis que guardam máximo e mínimo.
   let (mut min_y, mut max_y):(u16, u16) = (u16::MAX, u16::MIN);
   let (mut min_x, mut max_x):(u16, u16) = (u16::MAX, u16::MIN);

   // primeiro, buscando nos dados da barra.
   let coordenadas_barra = {
      dados_i.rastro
      .iter()
      .map(|tupla| tupla.0)
   };
   //obtem apenas os valores referentes 'Ponto's da tupla.
   let coordenadas_bola = {
      dados_ii.rastro
      .iter()
      .map(|tupla| tupla.0)
   };
   // mesclando ambos iteradores para avaliar em apenas um laço.
   let coordenadas = coordenadas_barra.chain(coordenadas_bola);
   
   /* primeiro busca computar 'Dimensão baseado no
    * rastro da 'Barra', posteriormente, melhora
    * à aproximação mastigando dados da 'Bola'. */
   for ponto in coordenadas {
      // pega um máximo 'x' se houver.
      if max_x < ponto.x 
         { max_x = ponto.x; } 
      // pega um máximo 'y' se houver.
      if max_y < ponto.y 
         { max_y = ponto.y; }
      // pega um mínimo 'y' se houver.
      if min_y > ponto.y 
         { min_y = ponto.y; }
      // pega um mínimo 'x' se houver.
      if min_x > ponto.x 
         { min_x = ponto.x; }
   }

   /* baseado na diferença do menor e maior
    * de cada coordenada, calculamos as 
    * proporções da 'Dimensão' de cada 
    * eixo aqui. */
   Dimensao { 
      altura: max_y - min_y,
      largura: max_x - min_x
   }
}

/// tanto metadados da 'Barra' e da 'Bola'.
type Metadados = (BarraMetadados, BolaMetadados);

/** executa a simulação de uma partida antiga 
 dado todos suas posições iniciais e finais,
 direções a cada instante de tempo, em cada
 posição. */
pub fn simulacao(dados:Metadados) {
   // computando a área do tabuleiro, baseado nos dados.
   let area:Dimensao = acha_dimensao_tabuleiro(&dados.0, &dados.1);
   // dados iniciais da 'Bola'.
   let parametros = dados.1;
   let mut bola = Bola::nova(
      parametros.rastro[0].1, 
      parametros.rastro[0].0 , 
      area
   );
   // todas posições da 'Bola' em cada instante.
   let rastros_bola = {
      parametros
      .rastro[1..]
      .iter()
      .map(|t| t.0)
   };
   // criando uma instância inicial.
   let parametros = dados.0;
   let mut barra = Barra::nova(
      parametros.comprimento as u16, 
      '^', parametros.rastro[0].0,
      area
   );
   // todas posições da 'Barra' em cada instante.
   let rastros_barra = {
      parametros
      .rastro[1..]
      .iter()
      .map(|t| t.0)
   };
   // tem que ter o mesmo tamanho
   assert_eq!(rastros_barra.len(), rastros_bola.len());

   /* criando tela para demonstração. Todo
    * o ambiente do ncurses será refeito aqui
    * especificamente para a simulação, algumas
    * coisas serão diferentes do layout original,
    * mas o tabuleiro o mesmo no final. */
   // instanciando elementos da janela.
   let janela = initscr();
   // criando tabuleiro com dimensão exatada da partida.
   let tabuleiro = newwin(
      area.altura as i32, 
      area.largura as i32, 
      0, 0
   );
   // configuração da janela:
   tabuleiro.keypad(true);
   tabuleiro.nodelay(true);
   janela.nodelay(true);
   curs_set(0);
   noecho();
   // inicia coloração.
   start_color();
   use_default_colors();

   /* executa o jogo até que todas posições dos
    * objetos tenham sido escritas pelos lugares
    * já destinados na tela. */
   for (p1, p2) in rastros_barra.zip(rastros_bola) { 
      // apaga "frame" anterior.
      tabuleiro.clear();
      // desenha as bordas do tabuleiro.
      let at = tabuleiro.get_max_y();
      let lt = tabuleiro.get_max_x();
      /* se a largura ou altura tiver sido
       * automaticamente redimensionada, então
       * voltar ela para proporção normal antes
       * de redesenhar a borda novamente. */
      if  (at > area.altura as i32) ||
      (lt > area.largura as i32) { 
         resize_term(area.altura as i32, area.largura as i32); 
      }
      tabuleiro.border(0, 0, 0, 0, 0, 0, 0, 0);

      // se detectar redimensionamento de janela...
      if let Some(Input::KeyResize) = janela.getch() {
         // variável com nomes curtos para legibilidade.
         let aj = janela.get_max_y();
         let lj = janela.get_max_x();
         /* se espaçar muito da janela principal
          * então centralizar menorzinha. */
         if aj - at > 5 || lj - lt > 3 {
            /* computando coordenadas do centro-superior-esquerdo
             * da janelinha para centralizá-la. */
            let yc = (aj/2) - (at/2);
            let xc = (lj/2) - (lt/2);
            tabuleiro.mvwin(yc, xc);
            // limpa janela maior para não deixar rastros.
            janela.clear();
         }
      }

      // coloca bola na posição tal.
      bola.move_a_posicao(p2.clone());
      // coloca barra na posição tal.
      barra.move_a_posicao(p1.clone());

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
}
