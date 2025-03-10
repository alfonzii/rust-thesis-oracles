- Main mam non-updated - ale ten sa aj tak bude menit takze je to jedno. [x]
- nemam tu cet_manager - ale to tiez nevadi, pozriem na Mac ak by bolo treba [ ]
- dokumentaciu nemam prekopirovanu [ ]
- dlc_controller je tu len velmi riedky vycuc [x]


[20.2.2025]
Ked opominieme benchmarking `math-bench.rs` ktory by mal sluzit na bench cisto krypto veci, tak __prostredie by malo byt schopne benchmarkovat vsetky sucasti DLC__ (to znamena, ze nase prostredie dovoli benchmarkovat kazdy jeden krok controllera a na zaklade toho sa vieme potom sustredit ze ktora cast stoji za to pozriet sa na nu lepsie a zoptimalizovat ju).
V maine budeme mat (pravdepodobne) 4 typy ktore by sme mali byt schopny menit dle libosti, a to:
1. *AdaptorSignatureScheme* -> ecdsa/schnorr (mala by byt nezavisla)
2. *CryptoUtils* -> simple/basal (tiez nezavisla)
3. *Oracle* -> rand_int resp. lubovolne ine take, ktore si ziada prostredie. -> je zavisle na CryptoUtils kvoli pocitaniu atestacie, inak nezavisle
4. *DlcController* -> tento bude obsahovat DlcComputation a DlcStorage v sebe, pretoze tie 2 su naviazane na DlcController.
5. hypoteticky Parser, ak sa k tomu dostaneme.

Zatial je to vsetko postavene okolo simple-all a basal systemu, tie su vzajomne kompatibilne. Ako to bude s digit-decomposition, to zatial neriesim.

[25.2.2025]
Dneska ideme naimplementovat basal crypto utils. Pravdepodobne nam bude treba poprerabat do stavoveho formatu, otazka je kde vsade. Ale kvoli cachingu a precomputed points to potrebne bude.

[3.3.2025]
Schnorr pre_signature funguje uz aj v Ruste. Avsak, v C dostavame hodnoty okolo 16μs a v Ruste okolo 30μs. Bolo by fajn zistit, ze preco je to takto. Ako prve sa ponuka pravdepodobne to, ze prechadzame skrz FFI boundary a dostavame tam overhead. Jedno riesenie ktore navrhol Habo bolo take, ze treba pouzit LTO (Link Time Optimization).
Mne vsak napadlo ine riesenie, ktore by bolo fajn skusit spolu s indom, a to by bolo batchovanie presigns. Takymto sposobom crossnem FFI boundary iba raz, v poli mam ulozene messages a atp_points, tie na strane C presignem, a potom crossnem FFI zase iba raz, ale vratim pole s vysledkami. Toto riesenie podla mna je najjednoduchsie co do principu "vela muziky za malo penazi". Let's see...

...
pri compute_anticipation_point v Basis verzii namiesto `combined.combine()` pouzijeme `PublicKey::combine_keys` lebo to spravi scitanie atp_points naraz. Ja si myslim, ze to bude mat suvis s prechadzanim cez FFI. a ze ked dame `combine_keys` tak prejdeme FFI len raz, scitame pole na strane C a potom dostaneme vysledok.

[9.3.2025]
Nakoniec sa ukazalo ze FFI pre schnorra problem neni, ale ze problem je to ako mam nastaveny interface pre `AdaptorSignatureScheme`. Ja som ho totiz vytvaral tak, aby odzrkadlovalo popis v "Cryptographic Oracle-Based Conditional Payments", cize pouzivam ako jeden z parametrov Secret Key. Lenze <u>Schnorr adaptor Signature od Inda</u> berie `keypair` ako input parameter do presign. Lenze, kedze ja mam ako input parameter Secret key, znamena to, ze zo secret key musim vyrobit keypair s pomocou funkcie `&signing_key.keypair(SECP256K1)`. No a tato funkcia, sa nakoniec ukazalo, ze zabera ~15µs. To tym padom znamena, ze polovicu casu z nasich 30µs na schnorr presign travime len vytvaranim `keypair`. Ked som urobil potom test priamo iba na `SchnorrAdaptorPreSignature::presign` s priamym pouzitim keypair, tak ta funkcia sa hybala potom okolo 15-18µs, co omnoho viac a presnejsie odraza cca 16µs ktore sme namerali v C. Preto zvolime dalsi postup, ze interface pre `AdaptorSignatureScheme` trochu prerobime, mierne sa odklonime od popisu v paper a namiesto SecretKey tam dame KeyPair. Kedze inde by sme mohli potrebovat SecretKey namiesto Keypair - napr. v ECDSA - tak vsade budeme zas volat z KeyPair funkciu, ktora vyrobi secret key. Toto je vsak zanedbatelne, pretoze ad. 1, keypair obsahuje vsetko na to, aby nemusel nic pocitat a vyslovene vytvarat secret key a ad2. potvrdil nam to aj benchmark, kde som zmeral beh funckie na return secretkey z keypair a rychlost behu je zhruba **1.33ns**. 

Spravili sme prvu verziu parsera, aj paralelnu aj seriovu. Zatial bez nejakych extra safety features a error catching, len bare bones. Primarne mi islo o to, zistit, ze kolko casu sa stravi parsovanim vstupu a ze ci je worth the time investovat nejak do toho dalsi cas na optimalizaciu. Uplne prve vysledky ukazuju, ze naparsovat vstup vskutocnosti trva menej nez vytvorit `controller` objekt. Preto nebudeme dalej optimalizovat tuto cast kodu. Je to uplne zbytocne. Akurat to bude treba dotiahnut do poriadneho konca, zeby parser bol aspon ako tak robustny. Dalsia zaujimavost je toto, ze seriovy pristup je rychlejsi nez paralelny. Pravdepodobne overhead v tomto pripade prevysuje samotny efektivny runtime. Parsovanie vstupu robime v kroku `Load input`.
Pozn.: teoreticky, mozno v buducnosti uvidime, ze ak vstup bude komplikovanejsi, tak mozno bude parsovanie trvat dlhsie. Avsak, aktualne je ten cas tak velmi kratky, ze si nemyslim, ze aj keby bol vstup komplexnejsi a vacsi a komplikovanejsi, tak ze to bude mat nejaky razantny dopad na runtime parsovania.

![alt text](images/image.png)

Spravili sme update AdaptorSignatureScheme, a dostali sme skoro dvojnasobne zrychlenie. Nejake milisekundy navyse ze to nedava presne polovicu casu mozu byt sposobene tym, ze sa tam pocita aj atp_point. 

![alt text](images/schnorr-keypair-update.png)