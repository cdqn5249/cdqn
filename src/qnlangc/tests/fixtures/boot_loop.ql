Q(dcc) dc;
Q(kappa) dyadic;
Q(domain) Qm;
Q(mode) L0;
Q(type) T : u64;
Q(variant) Step { Hit(o : T), Miss(o : T) }
Q(anchor) z : Q(0) : T;
Q(metric) m : eta(1,1,1);
Q(arr_new)<T, cap(4)> -> arr : Q(array)<T, cap(4)>;
Q(store)(arr, Q(1)) -> arr1 : Q(array)<T, cap(4)>;
Q(loop) @[trip(3)] {
  Q(2:cons)(Q(0), Q(1)) -> v : Q(tuple)<u64,u64>;
  Q(match)(v) { Hit -> Q(1); Miss -> Q(2); } -> r : u64;
  -> r
} -> loopres : u64;
Q(drop)(arr1);
emit receipt;
