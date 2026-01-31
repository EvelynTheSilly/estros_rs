# show current ARM EL
define show_el
  set $el = (($cpsr >> 2) & 3)
  printf "Current EL: EL%d\n", $el
end
