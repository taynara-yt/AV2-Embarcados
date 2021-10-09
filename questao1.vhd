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
