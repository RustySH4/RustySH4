// TODO: Write readme
// TODO: Implement notes from ghidra: https://github.com/NationalSecurityAgency/ghidra/blob/master/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc

# RustySH4

biblia: https://bible.planet-casio.com/yatis/hardware/

## ISA

- https://github.com/shared-ptr/sh_insns
- http://www.shared-ptr.com/sh_insns.html

## Part list

- https://pl.aliexpress.com/item/1005005630743750.html?spm=a2g0o.productlist.main.1.65fb0jeP0jePCP&algo_pvid=3f57bed2-39e1-4865-9df7-89fe65422408&algo_exp_id=3f57bed2-39e1-4865-9df7-89fe65422408-0&pdp_npi=4%40dis%21PLN%2118.35%2118.35%21%21%2130.17%21%21%40211b88ef16953104151046873e1a96%2112000033820257848%21sea%21PL%212925687436%21S&curPageLogUid=yYAh3F3bVZfH
- https://pl.aliexpress.com/item/1005005968916380.html?pdp_npi=3%40dis%21%2115%2C12%20z%C5%82%2115%2C12%20z%C5%82%21%21%21%21%21%40211b88f016953717655848819e6c9c%2112000035184604783%21sh%21PL%212925687436&spm=a2g0o.store_pc_aeChoiceSimplified.allitems_choice_2004575061944.1&gatewayAdapt=glo2pol
- https://pl.aliexpress.com/item/4000271223861.html?spm=a2g0o.productlist.main.41.f8b565e6PvKHW6&algo_pvid=1f50a21b-9173-4bd2-ba74-50c883c698bd&algo_exp_id=1f50a21b-9173-4bd2-ba74-50c883c698bd-20&pdp_npi=4%40dis%21PLN%214.67%214.62%21%21%211.05%21%21%40211b88f116953709755648965e86dc%2110000001104188457%21sea%21PL%212925687436%21&curPageLogUid=OzevLsbCLYti
- https://pl.aliexpress.com/item/1005001999296476.html?spm=a2g0o.productlist.main.13.4eaa2298y1VudT&algo_pvid=e90244b4-39c1-4770-8993-21fde5e67bcd&algo_exp_id=e90244b4-39c1-4770-8993-21fde5e67bcd-6&pdp_npi=4%40dis%21PLN%2141.67%2141.67%21%21%219.37%21%21%40211b88ef16953104822158400e1a96%2112000018365356573%21sea%21PL%212925687436%21S&curPageLogUid=HeT5AUIEVvaC
- https://pl.aliexpress.com/item/1865616455.html?spm=a2g0o.productlist.main.3.4be06RkN6RkNJt&algo_pvid=6317015a-d2c6-4ffb-bb2e-8c1f0ec6ed77&algo_exp_id=6317015a-d2c6-4ffb-bb2e-8c1f0ec6ed77-1&pdp_npi=4%40dis%21PLN%213.65%213.65%21%21%210.82%21%21%40211b88ef16953107764865452e1a96%2112000020106297436%21sea%21PL%212925687436%21S&curPageLogUid=JoM1px0n0iT4
- https://pl.aliexpress.com/item/32786777105.html?spm=a2g0o.productlist.main.13.58e1618eyI5Wq9&algo_pvid=a426562f-9932-4bfd-b15f-40383150b14c&algo_exp_id=a426562f-9932-4bfd-b15f-40383150b14c-6&pdp_npi=4%40dis%21PLN%216.66%216.66%21%21%211.50%21%21%40211b88ef16953689055551718e1aa6%21-1%21sea%21PL%212925687436%21&curPageLogUid=dq1VGtMkVkDQ
- https://pl.aliexpress.com/item/1005002475325986.html?spm=a2g0o.productlist.main.51.4c8e4cb6JmElB7&algo_pvid=da0f0b2c-9532-45ba-9f5c-b54bdb9594d8&algo_exp_id=da0f0b2c-9532-45ba-9f5c-b54bdb9594d8-25&pdp_npi=4%40dis%21PLN%2112.17%2112.17%21%21%2120.00%21%21%40211b88ef16953690095102982e1aa6%21-1%21sea%21PL%212925687436%21&curPageLogUid=qdGa7ulnJUZ8
- https://pl.aliexpress.com/item/1005004782287622.html?spm=a2g0o.productlist.main.35.117e2hu22hu2nU&algo_pvid=930f0003-fbbf-45ba-8144-ba2d80f76f7f&algo_exp_id=930f0003-fbbf-45ba-8144-ba2d80f76f7f-17&pdp_npi=4%40dis%21PLN%215.69%214.0%21%21%211.28%21%21%40211b88ec16953707862302436e503e%2112000030466757690%21sea%21PL%212925687436%21&curPageLogUid=yrHm6Yjgt0zC
- https://pl.aliexpress.com/item/1005004818919331.html?spm=a2g0o.detail.0.0.15305cceYjYvE0&gps-id=pcDetailTopMoreOtherSeller&scm=1007.40000.327270.0&scm_id=1007.40000.327270.0&scm-url=1007.40000.327270.0&pvid=ab9c0d22-11eb-4d6e-a7f3-304fe21c5e3d&_t=gps-id:pcDetailTopMoreOtherSeller,scm-url:1007.40000.327270.0,pvid:ab9c0d22-11eb-4d6e-a7f3-304fe21c5e3d,tpp_buckets:668%232846%238113%231998&pdp_npi=4%40dis%21PLN%2110.84%2111.2%21%21%212.44%21%21%40211b5e2b16953709235144692e9d7b%2112000030607054927%21rec%21PL%212925687436%21
