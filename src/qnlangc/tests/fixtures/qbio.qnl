Q(dcc) dcc3;
Q(kappa) dyadic;
Q(domain) Qbio;
Q(mode) L2;
Q(type) BioT : Qm[0..255];
Q(blanket) Blanket {
  sense : x : BioT;
  state : s : BioT;
  act : a : BioT;
}
Q(anchor) bzero : Q(0) : BioT;
Q(metric) bio : eta(1,1,1);
Blanket -> inst : BioT;
metabolic_cap(inst, Q(64)) -> clamped : BioT;
may_emit(clamped, receipt) -> ok : bool;
emit receipt;
