Q(dcc) dc;
Q(kappa) dyadic;
Q(domain) (Qs, Qlog);
Q(mode) L3;
Q(type) T : Qm[-255..255];
Q(anchor) z : Q(0) : T;
Q(metric) m : eta(1,1,1);
Q(pattern) step(k : T) -> T ![Qlog, budget(64 U), may_emit(receipt)] {
  Q(tbl_new)<T, T, cap(2)> -> tbl : Q(table)<T, T, cap(2)> ![budget(20 U)];
  Q(store)(tbl, (k, Q(1))) -> tbl1 : Q(table)<T, T, cap(2)>;
  Q(load)(tbl1, k) @[trip(1)] -> hit or_else {
    -> Q(0)
  } -> out : T;
  Q(drop)(tbl1);
  -> out
}
step(Q(1)) -> res : T;
emit receipt;
