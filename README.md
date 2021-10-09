# Segunda Avaliação - Sistemas Embarcados
## Sobre este repositório

Repositório destinado a disciplina Sistemas Embarcados, ministrado pelo **Prof. Vandermi Joao da Silva**.
Aqui estará todas as implementações solicitadas.


### Discentes

- Taynara Silva, 21751227

### Resolução
**Questão 1**
Construa um circuito com portas lógicas que permitam modificar as saídas quando processadas.
O circuito deverá receber como entrada um registrador A e B e a resposta deverá ser armazenada em X. 
A Saída deverá inverter o resultado do processamento. Após a construção do circuito, codifique-o usando VHDL.

![circuito](https://github.com/taynara-yt/AV2-Embarcados/blob/main/image.jpg?raw=true)

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

