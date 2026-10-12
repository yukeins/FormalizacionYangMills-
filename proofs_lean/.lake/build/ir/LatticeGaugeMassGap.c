// Lean compiler output
// Module: LatticeGaugeMassGap
// Imports: public import Init public meta import Init
#include <lean/lean.h>
#if defined(__clang__)
#pragma clang diagnostic ignored "-Wunused-parameter"
#pragma clang diagnostic ignored "-Wunused-label"
#elif defined(__GNUC__) && !defined(__CLANG__)
#pragma GCC diagnostic ignored "-Wunused-parameter"
#pragma GCC diagnostic ignored "-Wunused-label"
#pragma GCC diagnostic ignored "-Wunused-but-set-variable"
#endif
#ifdef __cplusplus
extern "C" {
#endif
lean_object* lean_nat_add(lean_object*, lean_object*);
lean_object* lean_nat_mul(lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_LatticeGaugeMassGap_LatticeGaugeTheory_electric__casimir(lean_object*);
LEAN_EXPORT lean_object* lp_LatticeGaugeMassGap_LatticeGaugeTheory_electric__casimir___boxed(lean_object*);
LEAN_EXPORT lean_object* lp_LatticeGaugeMassGap_LatticeGaugeTheory_vacuum__j;
LEAN_EXPORT lean_object* lp_LatticeGaugeMassGap_LatticeGaugeTheory_fundamental__j;
LEAN_EXPORT lean_object* lp_LatticeGaugeMassGap_LatticeGaugeTheory_plaquette__trace__bound;
LEAN_EXPORT lean_object* lp_LatticeGaugeMassGap_LatticeGaugeTheory_total__magnetic__bound(lean_object*);
LEAN_EXPORT lean_object* lp_LatticeGaugeMassGap_LatticeGaugeTheory_total__magnetic__bound___boxed(lean_object*);
LEAN_EXPORT lean_object* lp_LatticeGaugeMassGap_LatticeGaugeTheory_electric__casimir(lean_object* v_j_1_){
_start:
{
lean_object* v___x_2_; lean_object* v___x_3_; lean_object* v___x_4_; 
v___x_2_ = lean_unsigned_to_nat(2u);
v___x_3_ = lean_nat_add(v_j_1_, v___x_2_);
v___x_4_ = lean_nat_mul(v_j_1_, v___x_3_);
lean_dec(v___x_3_);
return v___x_4_;
}
}
LEAN_EXPORT lean_object* lp_LatticeGaugeMassGap_LatticeGaugeTheory_electric__casimir___boxed(lean_object* v_j_5_){
_start:
{
lean_object* v_res_6_; 
v_res_6_ = lp_LatticeGaugeMassGap_LatticeGaugeTheory_electric__casimir(v_j_5_);
lean_dec(v_j_5_);
return v_res_6_;
}
}
static lean_object* _init_lp_LatticeGaugeMassGap_LatticeGaugeTheory_vacuum__j(void){
_start:
{
lean_object* v___x_7_; 
v___x_7_ = lean_unsigned_to_nat(0u);
return v___x_7_;
}
}
static lean_object* _init_lp_LatticeGaugeMassGap_LatticeGaugeTheory_fundamental__j(void){
_start:
{
lean_object* v___x_8_; 
v___x_8_ = lean_unsigned_to_nat(1u);
return v___x_8_;
}
}
static lean_object* _init_lp_LatticeGaugeMassGap_LatticeGaugeTheory_plaquette__trace__bound(void){
_start:
{
lean_object* v___x_9_; 
v___x_9_ = lean_unsigned_to_nat(2u);
return v___x_9_;
}
}
LEAN_EXPORT lean_object* lp_LatticeGaugeMassGap_LatticeGaugeTheory_total__magnetic__bound(lean_object* v_N__p_10_){
_start:
{
lean_object* v___x_11_; lean_object* v___x_12_; 
v___x_11_ = lean_unsigned_to_nat(4u);
v___x_12_ = lean_nat_mul(v___x_11_, v_N__p_10_);
return v___x_12_;
}
}
LEAN_EXPORT lean_object* lp_LatticeGaugeMassGap_LatticeGaugeTheory_total__magnetic__bound___boxed(lean_object* v_N__p_13_){
_start:
{
lean_object* v_res_14_; 
v_res_14_ = lp_LatticeGaugeMassGap_LatticeGaugeTheory_total__magnetic__bound(v_N__p_13_);
lean_dec(v_N__p_13_);
return v_res_14_;
}
}
lean_object* initialize_Init(uint8_t builtin);
lean_object* initialize_Init(uint8_t builtin);
void lean_initialize_runtime_module();
static bool _G_initialized = false;
LEAN_EXPORT lean_object* initialize_LatticeGaugeMassGap_LatticeGaugeMassGap(uint8_t builtin) {
lean_object * res;
if (_G_initialized) return lean_io_result_mk_ok(lean_box(0));
_G_initialized = true;
lean_initialize_runtime_module();
res = initialize_Init(builtin);
if (lean_io_result_is_error(res)) return res;
lean_dec_ref(res);
res = initialize_Init(builtin);
if (lean_io_result_is_error(res)) return res;
lean_dec_ref(res);
lp_LatticeGaugeMassGap_LatticeGaugeTheory_vacuum__j = _init_lp_LatticeGaugeMassGap_LatticeGaugeTheory_vacuum__j();
lean_mark_persistent(lp_LatticeGaugeMassGap_LatticeGaugeTheory_vacuum__j);
lp_LatticeGaugeMassGap_LatticeGaugeTheory_fundamental__j = _init_lp_LatticeGaugeMassGap_LatticeGaugeTheory_fundamental__j();
lean_mark_persistent(lp_LatticeGaugeMassGap_LatticeGaugeTheory_fundamental__j);
lp_LatticeGaugeMassGap_LatticeGaugeTheory_plaquette__trace__bound = _init_lp_LatticeGaugeMassGap_LatticeGaugeTheory_plaquette__trace__bound();
lean_mark_persistent(lp_LatticeGaugeMassGap_LatticeGaugeTheory_plaquette__trace__bound);
return lean_io_result_mk_ok(lean_box(0));
}
#ifdef __cplusplus
}
#endif
