
/**! criar um jeito de armazenar os dados 
 gerados durante o jogo "bate-bola", pois
 quando termina dá uma informação geral 
 sobre a partida, porém aquilo fica perdido.
 A proposta aqui é pegar não só está informação
 de fim de partida, mais também o que permite 
 simular a partida jogada futuramente. 
*/

// importando objetos a transformar em bytes.
use super::estatisticas::{BolaMetadados, BarraMetadados};
use super::modelos::{Direcao, Ponto, Parede};
// biblioteca padrão do Rust:
use std::time::{Duration};
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::collections::HashMap;


// definindo trait de dados.
trait ByterizacaoI {
   fn serializa(&self) -> [u8; 8];
   fn deserializa(&self, _:[u8; 8]) -> u64;
}

// implementando serialização do tipo 'Duration'.
impl ByterizacaoI for Duration {
   fn serializa(&self) -> [u8; 8] {
      self.as_secs().to_le_bytes()  
   }
   fn deserializa(&self, bytes:[u8; 8]) -> u64 {
      u64::from_le_bytes(bytes)
   }
}

impl Direcao {
   /* transforma o tipo de enum num byte.
    * Dando apenas letras estranhas aos demais
    * pontos cardiais, pois se for colocar a 
    * nomeclatura oficial, irá precisar modificar
    * toda a serialização para 2 bytes. Portanto,
    * ficará: Noroeste(A), Nordeste(B), Sudeste(C)
    * e Sudoeste(D). Sim é uma renomeação baseado
    * na leitura das setas no sentido horário. */
   pub fn serializa(&self) -> u8 {
      // baseado no tipo:
      match *self {
         Direcao::Norte => b'N',
         Direcao::Sul => b'S',
         Direcao::Leste => b'L',
         Direcao::Oeste => b'O',
         Direcao::Noroeste => b'A',
         Direcao::Nordeste => b'B',
         Direcao::Sudeste => b'C',
         Direcao::Sudoeste => b'D',
      }
   }
   // transforma mero byte na direção equivalente.
   pub fn deserializa(byte:u8) -> Direcao {
      if (byte as char) == 'N' 
         { Direcao::Norte }
      else if (byte as char) == 'S' 
         { Direcao::Sul }
      else if (byte as char) == 'L' 
         { Direcao::Leste }
      else if (byte as char) == 'A'
         { Direcao::Noroeste }
      else if (byte as char) == 'B'
         { Direcao::Noroeste }
      else if (byte as char) == 'C'
         { Direcao::Sudeste }
      else if (byte as char) == 'D'
         { Direcao::Sudeste }
      else { Direcao::Oeste }
   }
}

impl Ponto {
   // transforma o tipo de enum num byte.
   pub fn serializa(&self) -> [u8; 4] {
      // array com valores da posição 'x' e 'y'.
      let mut bytes:[u8; 4] = [0, 0, 0, 0];
      // byte do valor 'x':
      let x_bytes = self.x.to_le_bytes();
      // byte do valor 8-bits 'y':
      let y_bytes = self.y.to_le_bytes();
      /* primeiro colocamos o valor de 'x', e 
       * posteriormente o de 'y'. */
      bytes[0] = x_bytes[0];
      bytes[1] = x_bytes[1];
      bytes[2] = y_bytes[0];
      bytes[3] = y_bytes[1];
      // retornando dois bytes, representando cada atributo.
      return bytes;
   }
   // transforma mero byte na direção equivalente.
   pub fn deserializa(bytes:[u8; 4]) -> Ponto {
      /* pega um byte da array de dois, e 
       * converte num valor 8-bits. */
      let bytes_valor_x = [bytes[0], bytes[1]];
      let bytes_valor_y = [bytes[2], bytes[3]];
      // retorna nova instância do ponto.
      return Ponto {
         // convertendo para um valor 8-bits sem sinal...
         x: u16::from_le_bytes(bytes_valor_x),
         // o outro valor...
         y: u16::from_le_bytes(bytes_valor_y),
      };
   }
}

// implementado serialização para os dois tipos...
impl BarraMetadados {
   /* De novo, um array-dinâmica, pois 
    * o tipo de dado também têm uma array
    * deste tipo internamente, o que faz a
    * computação do tamanho em tempo de
    * compilação impossível. */
   pub fn serializa(&self) -> Vec<u8> {
      // array-dinâmico que conterá todos dados.
      let mut todos_bytes = Vec::new();
      /* o tipo 'Duration' já tem uma serialização 
       * própria implementada. O tamanho é 2 bytes.*/
      let aux = self.tempo_travessia.serializa();
      todos_bytes.extend(&aux[..]);
      // valor de 16-bits(portanto, 2 bytes).
      let aux = self.qtd_rebatidas.to_le_bytes();
      todos_bytes.extend(&aux[..]);
      // outro valor de 16-bits(já disse, 2 bytes).
      let aux = self.total_comandos_dados.to_le_bytes();
      todos_bytes.extend(&aux[..]);
      /* agora vem a implementação da array-dinâmica
       * onde: primeiro serializa seu tamanho, para
       * posteriormente seus valores, sendo que se
       * for uma tupla com tipos diferentes, a ordem
       * será da esquerda-à-direita.
       * Tal tamanho será 32-bits(4 bytes), pois não 
       * é um jogo de termino, sim de recorde, assim
       * uma coisa na casa dos milhões ficam com um
       * bom espaço. */
      let aux = (self.rastro.len() as u32).to_le_bytes();
      todos_bytes.extend(&aux[..]);
      // agora todos os valores dela.
      for tupla in self.rastro.iter() {
         // 'Ponto' que já têm "serialização" implementada.
         let aux_i = tupla.0.serializa();
         todos_bytes.extend(&aux_i[..]);
         // 'Direção' que já têm "serialização" implementada.
         let aux_ii = tupla.1.serializa();
         todos_bytes.push(aux_ii);
      }
      // como têm 1 bytes, apenas coloca na forma crua.
      todos_bytes.push(self.comprimento);
      // os últimos 2 bytes de um valor inteiro-positivo 16-bits.
      let aux = self.colisoes_nas_paredes.to_le_bytes();
      todos_bytes.extend(&aux[..]);
      // retorna o "linguição de bytes".
      return todos_bytes;
   }
   pub fn deserializa(bd:&mut File) -> Option<Self> {
      // 2 bytes do tempo-de-travessia.
      let mut aux:[u8; 8] = [u8::MAX; 8];
      let aux_i:u64;
      match bd.read_exact(&mut aux) {
         Ok(_) => aux_i = u64::from_le_bytes(aux),
         Err(_) => return None
      };
      let tt:Duration = Duration::from_secs(aux_i);
      // 2 bytes da qtd-de-rebatidas.
      let mut aux:[u8; 2] = [u8::MAX; 2];
      let qr:u16; 
      match bd.read_exact(&mut aux) {
         Ok(_) => qr = u16::from_le_bytes(aux),
         Err(_) => return None
      };
      // 2 bytes do total-de-comandos-dados.
      let mut aux:[u8; 2] = [u8::MAX; 2];
      let tcd:u16; 
      match bd.read_exact(&mut aux) {
         Ok(_) => tcd = u16::from_le_bytes(aux),
         Err(_) => return None
      };
      /* 4 bytes do tamanho da array-dinâmica;
       * 2 bytes de cada 'Ponto' da tupla e 1 byte
       * da 'Direcao'. */
      let mut aux:[u8; 4] = [u8::MAX; 4];
      let mut tamanho:u32;
      let mut rst:Vec<(Ponto, Direcao)> = Vec::new();
      match bd.read_exact(&mut aux) {
         Ok(_) => tamanho = u32::from_le_bytes(aux),
         Err(_) => return None
      };
      while tamanho > 0 {
         let mut aux_i:[u8; 4] = [u8::MAX; 4];
         let mut aux_ii:[u8; 1] = [0u8; 1];
         let pt:Ponto;
         let dir:Direcao;
         match bd.read_exact(&mut aux_i) {
            Ok(_) => pt = Ponto::deserializa(aux_i),
            Err(_) => return None
         };
         match bd.read_exact(&mut aux_ii) {
            Ok(_) => dir = Direcao::deserializa(aux_ii[0]),
            Err(_) => return None
         };
         tamanho -= 1;
         // adicionando na fila.
         rst.push((pt, dir));
      }
      // 1 byte crú representando o "comprimento".
      let mut aux:[u8; 1] = [0];
      let c:u8;
      match bd.read_exact(&mut aux) {
         Ok(_) => c = aux[0],
         Err(_) => return None
      };
      // 2 bytes colisões-na-parede.
      let mut aux:[u8; 2] = [0, 0];
      let cp:u16;
      match bd.read_exact(&mut aux) {
         Ok(_) => cp = u16::from_le_bytes(aux),
         Err(_) => return None
      };
      // retornando a instância criado com os dados drenados.
      return Some(Self::gera_via_parametros(tt, qr, tcd, rst, c, cp));
   }
}

impl BolaMetadados {
   pub fn serializa(&self) -> Vec<u8> {
      // array-dinâmico que conterá todos dados.
      let mut todos_bytes = Vec::new();
      /* agora vem a implementação da array-dinâmica
       * onde: primeiro serializa seu tamanho, para
       * posteriormente seus valores, sendo que se
       * for uma tupla com tipos diferentes, a ordem
       * será da esquerda-à-direita.
       * Tal tamanho será 32-bits(4 bytes), pois não 
       * é um jogo de termino, sim de recorde, assim
       * uma coisa na casa dos milhões ficam com um
       * bom espaço. */
      let aux = (self.rastro.len() as u32).to_le_bytes();
      todos_bytes.extend(&aux[..]);
      // agora todos os valores dela.
      for tupla in self.rastro.iter() {
         // 'Ponto' que já têm "serialização" implementada.
         let aux_i = tupla.0.serializa();
         todos_bytes.extend(&aux_i[..]);
         // 'Direção' que já têm "serialização" implementada.
         let aux_ii = tupla.1.serializa();
         todos_bytes.push(aux_ii);
      }
      let aux = (self.colisoes.len() as u32).to_le_bytes();
      todos_bytes.extend(&aux[..]);
      // agora todos os valores dela.
      for tupla in self.colisoes.iter() {
         // 'Ponto' que já têm "serialização" implementada.
         let aux_i = tupla.0.serializa();
         todos_bytes.extend(&aux_i[..]);
         // 'Parede' que terá "serialização" implementada. 
         let aux_ii = tupla.1.serializa();
         todos_bytes.push(aux_ii);
         // 'Direção' que já têm "serialização" implementada.
         let aux_iii = tupla.2.serializa();
         todos_bytes.push(aux_iii);
      }
      // retorna o "linguição de bytes".
      return todos_bytes;
   }
   pub fn deserializa(bd:&mut File) -> Option<Self> {
      /* 4 bytes do tamanho da array-dinâmica;
       * 2 bytes de cada 'Ponto' da tupla e 1 byte
       * da 'Direcao'. */
      let mut aux:[u8; 4] = [0, 0, 0, 0];
      let mut tamanho:u32;
      let mut rastro:Vec<(Ponto, Direcao)> = Vec::new();
      match bd.read_exact(&mut aux) {
         Ok(_) => tamanho = u32::from_le_bytes(aux),
         Err(_) => return None
      };
      while tamanho > 0 {
         let mut aux_i:[u8; 4] = [0,0,0,0];
         let mut aux_ii:[u8; 1] = [0u8; 1];
         let pt:Ponto;
         let dir:Direcao;
         match bd.read_exact(&mut aux_i) {
            Ok(_) => pt = Ponto::deserializa(aux_i),
            Err(_) => return None
         };
         match bd.read_exact(&mut aux_ii) {
            Ok(_) => dir = Direcao::deserializa(aux_ii[0]),
            Err(_) => return None
         };
         tamanho -= 1;
         // adicionando na fila.
         rastro.push((pt, dir));
      }
      /* obtendo dados da outra array-dinâmica, mesmo esquema.
       * como já há variáveis existentes, do mesmo
       * tipo, vamos apenas utilizar-lás. 
       * Computando tamanho: 4 bytes do tamanho da array-dinâmica;
       * 2 bytes do 'Ponto', 1 byte da 'Parede'; e também
       * 1 byte da 'Direção'. */
      match bd.read_exact(&mut aux) {
         Ok(_) => tamanho = u32::from_le_bytes(aux),
         Err(_) => return None
      };
      let mut colisoes:Vec<(Ponto, Parede, Direcao)> = Vec::new();
      while tamanho > 0 {
         // arrays auxiliares para colocar os bytes.
         let mut aux_i:[u8; 4] = [u8::MAX; 4];
         let mut aux_ii:[u8; 1] = [0];
         let mut aux_iii:[u8; 1] = [0u8; 1];
         // tipos que obterão os dados.
         let pt:Ponto;
         let prd:Parede;
         let dir:Direcao;
         // lendo primeiro o 'Ponto'...
         match bd.read_exact(&mut aux_i) {
            Ok(_) => pt = Ponto::deserializa(aux_i),
            Err(_) => return None
         };
         // então a 'Parede'.
         match bd.read_exact(&mut aux_ii) {
            Ok(_) => prd = Parede::deserializa(aux_ii[0]),
            Err(_) => return None
         };
         // por último, a 'Direcao'.
         match bd.read_exact(&mut aux_iii) {
            Ok(_) => dir = Direcao::deserializa(aux_iii[0]),
            Err(_) => return None
         };
         tamanho -= 1;
         // adicionando na fila.
         colisoes.push((pt, prd, dir));
      }
      // retornando a instância criado com os dados drenados.
      return Some( Self { rastro, colisoes } );
   }
}

// implementando serialização/deserialização para o tipo 'Parede'.
impl Parede {
   pub fn serializa(&self) -> u8 {
      match *self {
         Parede::LateralEsquerda => 1 ,
         Parede::LateralDireita => 2,
         Parede::Teto => 3,
         Parede::Chao => 4,
         Parede::Canto => 5,
         Parede::SemContato => 6,
      }
   }
   pub fn deserializa(byte:u8) -> Self {
      // trabalhando no tipo de byte...
      match byte {
         1 => Parede::LateralEsquerda,
         2 => Parede::LateralDireita,
         3 => Parede::Teto,
         4 => Parede::Chao,
         5 => Parede::Canto,
         6 => Parede::SemContato,
         // usar recursão e, basear-se no resto.
         0 | 7.. => Self::deserializa(byte % 6 + 1),
      }
   }
}

/* Agora trabalhando na armazenagem, e no 
 * carregamento de dados. */

// renomeando tupla para melhor legibilidade.
type Dados = (BarraMetadados, BolaMetadados);
// nome do arquivo que representa BD.
const NOME_ARQ:&'static str = "registros_partidas.dat";

pub fn carrega() -> Result<HashMap<u16, Dados>, &'static str> {
   // tabela-hash para pegar 'Dados' por ordem de inserção.
   let mut tabela:HashMap<u16, Dados> = HashMap::new();
   // ordem das inserções é a mesma com que lê.
   let mut indice:u16 = 0;
   // abrindo o arquivo com os dados.
   let mut arquivo:File = {
      OpenOptions::new()
      .read(true)
      .open(NOME_ARQ)
      .unwrap()
   };
   /* tentar lêr todos "bit-patterns" que envoltam tanto
    * os dados 'Barra', como o da 'Bola'. */
   'leitura: loop {
      // tenta lê dados da barra, supondo que foram escritos primeiro.
      let dados_brr:BarraMetadados = {
         match BarraMetadados::deserializa(&mut arquivo) {
            Some(brrdata) => brrdata,
            None => break 'leitura,
         }
      };
      // ...depois os dados da bola.
      let dados_bl:BolaMetadados = {
         match BolaMetadados::deserializa(&mut arquivo) {
            Some(bldata) => bldata,
            None => break 'leitura,
         }
      };
      // agrupa dados numa tupla para inserir na tabela.
      let dados = (dados_brr, dados_bl);
      // inserindo na tabela.
      tabela.insert(indice, dados);
      // indo adiante na próxima inserção.
      indice += 1;
   }
   Ok(tabela)
}

pub fn salva(dados_brr:BarraMetadados, dados_bl:BolaMetadados) {
   // abrindo o arquivo com os dados.
   let mut arquivo:File = {
      OpenOptions::new()
      .create(true)
      .append(true)
      .open(NOME_ARQ)
      .unwrap()
   };
   /* sempre nesta ordem, primeiro grava os dados
    * da barra, posteriormente o da bola. */
   match arquivo.write(&dados_brr.serializa()[..]) {
      Ok(tamanho) => 
         println!("foram gravados {} bytes com sucesso.", tamanho),
      Err(_) => 
         println!("erro ao gravar 'BarraMetadados'"),
   };
   match arquivo.write(&dados_bl.serializa()[..]) {
      Ok(tamanho) => 
         println!("foram gravados {} bytes com sucesso.", tamanho),
      Err(_) => 
         println!("erro ao gravar 'BolaMetadados'"),
   };
}
