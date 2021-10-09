# Segunda Avaliação - Sistemas Embarcados
## Sobre este repositório

Repositório destinado a disciplina Sistemas Embarcados, ministrado pelo **Prof. Vandermi Joao da Silva**.
Aqui estará todas as implementações solicitadas.


### Discentes

- Taynara Silva da Costa, 21751227

### Resolução
**Questão 1**-
Construa um circuito com portas lógicas que permitam modificar as saídas quando processadas.
O circuito deverá receber como entrada um registrador A e B e a resposta deverá ser armazenada em X. 
A Saída deverá inverter o resultado do processamento. Após a construção do circuito, codifique-o usando VHDL.

![circuito](https://github.com/taynara-yt/AV2-Embarcados/blob/main/circuito.jpg?raw=true)

~~~VHDL
library ieee;
use ieee.std_logic_1164.all;
-- declaração da entidade
entity porta is
 port (a,b: in std_logic;
       x: out std_logic);
end porta;
-- declaração da arquitetura
architecture questao of porta is
begin
 x <= not (not (a and b));  
end questao;

}
~~~

**Questão 2**-
Codifique o circuito desenvolvido na questão 1 em linguagem de programação Rust.

~~~rust
fn main(){    
    let mut a = Vec::<bool>::new();
    a.push(false);
    a.push(false);
    a.push(true);
    a.push(true);
    
    let mut b = Vec::<bool>::new();
    b.push(false);
    b.push(true);
    b.push(false);
    b.push(true);
    
    let mut x = Vec::<bool>::new();
    let mut xn = Vec::<bool>::new();
    
    println!("A & B = X = !X = !X");
    
    for i in 0..4 {
              // A && B        (A and B)
        x.push(a[i] && b[i]); 
              // !(A && B)    not(A and B)
        xn.push(!(x[i]));     
        println!("{}   {}   {}   {}    {}", match a[i] {true => 1,false => 0}, 
                                            match b[i] {true => 1,false => 0}, 
                                            match x[i] {true => 1,false => 0}, 
                                            match xn[i] {true => 1,false => 0},
                                            match !xn[i] {true => 1,false => 0}); // !(!(A && B))    not(not(A and B))
    }
}
~~~

**Questão 4**-
Com base na Figura acima explique o processo de implementação Hardware/Software 
~~~txt
A figura representa a forma para o processo de implementação Hardware/Software, onde o código é implementado no microprocessador, e este possui unidades de hardware que são circuitos específicos de uma placa.

}
~~~





