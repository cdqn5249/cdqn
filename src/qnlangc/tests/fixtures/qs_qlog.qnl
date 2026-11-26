Q(dcc) dcc4;
Q(kappa) dyadic;
Q(domain) (Qs, Qlog);
Q(mode) L3;
Q(type) KeyT : u64;
Q(type) StepOut : Q(tuple)<bool, u64>;
Q(anchor) kzero : Q(0) : KeyT;
Q(metric) lg : eta(1,1,1);
Q(variant) Step { Hit(o : StepOut), Miss(o : StepOut) }
Q(falsify) bad(k : KeyT) ![Qlog, may_emit(receipt)] {
  Q(halt_with)(receipt);
}
Q(pattern) step(k : KeyT) -> StepOut ![Qlog, budget(64 U)] {
  Q(tbl_new)<KeyT, bool, cap(2)> -> tbl : Q(table)<KeyT, bool, cap(2)> ![budget(20 U)];
  Q(store)(tbl, (k, Q(1))) -> tbl1 : Q(table)<KeyT, bool, cap(2)>;
  Q(load)(tbl1, k) @[trip(1)] -> hit or_else {
    Q(cut)(Q(0));
    -> (Q(0), Q(0))
  } -> out : StepOut;
  Q(drop)(tbl1);
  -> hit
}
Q(workflow) route(k : KeyT) -> StepOut ![Qlog, budget(80 U), may_emit(receipt)] {
  step(k) -> res : StepOut;
  Q(reuse)(res) -> memo : StepOut;
  -> memo
}
modal_as(route, Qlog) -> view : Qlog;
emit receipt;
