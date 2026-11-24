Q(dcc) dcc2;
Q(kappa) fact;
Q(domain) (Qchem, Qm);
Q(mode) L1;
Q(type) Stoich : Qm[-255..255];
Q(type) Pair : Q(tuple)<Stoich, Stoich>;
Q(anchor) zero : Q(0) : Stoich;
Q(metric) ch : eta(1,1,1);
Q(variant) Species { A(s : Stoich), B(s : Stoich) }
Q(2:cons)(Q(0), Q(1)) -> pa : Pair;
Q(2:cons)(Q(1), Q(0)) -> pb : Pair;
Q(arr_new)<Pair, cap(2)> -> arr : Q(array)<Pair, cap(2)> ![budget(20 U)];
Q(store)(arr, pa) -> arr1 : Q(array)<Pair, cap(2)>;
Q(store)(arr1, pb) -> arr2 : Q(array)<Pair, cap(2)>;
Q(fold) arr2 over [Q(0)..<Q(2)] init(Q(0)) as (acc, i) { -> acc + Q(1) } -> flux : Stoich;
Q(drop)(arr2);
Q(metric_dot)(pa, pb) = zero -> balanced : bool;
emit receipt;
