# TO-DO

- ✅ ~~finalize_tx_valid musi byt ASigS agnosticka - musi byt schopna rozlisit ASigS a podla toho spravit korektnu validaciu tx~~

- ✅ ~~CryptoUtils budem musiet prerobit na stavove kvoli BasalCryptoUtils a caching (otazka je ze ked CryptoUtils budu stavove, tak ze ci poto aj DlcComputation potrebuje byt stavova)~~

- ✅ ~~paralelne pristupy pravdepodobne neni potreba davat do oosobitnych implementacii, ale asi by stacilo iba urobit jak to maju oni v `rust-dlc` ze `#[cfg(feature = "parallel")]` a `#[cfg(not(feature = "parallel"))]` v jednom kode. takto nemusime duplikovat rovnaky kod a zaroven mozeme dat testy rovno do implementacie a ked tomu paralelizacia nevadi, tak ani testy neduplikujeme a mame rovno otestovanu parallel aj non-parallel verziu. dlc_computation hotove, este parser~~
- ✅ <del>asi prerobit payout na pouzivanie u64 namiesto u32. Ak mam U32, ta viem poslat nanajvys ~40BTC. </del>

**[10.3.2025]**
- ✅ ~~dokoncit parser - urobit ho error proof a robustny do istej miery~~
- ✅ ~~prerobit interface asigs aby brali keypair namiesto secret_key kvoli schnorr adaptor~~
- ✅ ~~spolocne s parserom bude treba pomenit typy outcomes, payouts atd tak, aby davali zmysel~~
    + ✅ ~~delenie integer si myslim ze nedava dobry increment do step pre naplnanie payoutu, takze tiez premysliet~~
    + ✅ ~~takisto asi bude treba urobit druhu funkciu, ktora vracia len `ContractInput` pretoze max collateral a toto sa vytahuje z kontraktu a tu naparsovanu strukturu ContractInput potrebujem byt schopny pouzivat v kode~~

## [THINK ABOUT]
- ❔rozmysliet ci nechat ParsedInput ako vektor parov alebo index bude outcome a payout bude iba hodnota
- ❔teoreticky, eventy v oracle by mi mohli sluzit ako MAX_OUTCOME identifier. rozmysliet potom neskor
- ❌ Oracle v Controlleri v trait params nepouzivam. Takze to pravdepodobne musi ist urobit nejak inak. Prerobit - asi to nejde nijak inak urobit ak nechcem pouzivat dynamic dispatch
- zgeneralizovat kod, dlc_controller (mozno aj storage a computation, ten neviem) controller tam ma nahardcodene veci... controller by mal byt hotovy, teraz este computation a storage
- ❔velmi casto v kode pouzivam ako trait parameter *AdaptorSignatureScheme* pricom realne, jedina vec ktoru potrebujem odtial tak je *ASigS::AdaptorSignature*. Nejde to urobit nejak inak, viac "krajsie" ?

## [MUST HAVE]
- ✅ ~~zjednotit nejakym sposobom runparams, constants atd.~~
    + ~~k tomu takisto treba potom premysliet kde a jak sa budu nastavovat parametre behu, zeby to slo nejak jednoducho uzivatelsky menit. Teraz to mam cez tie cfg features, ale nie vsetko, typy niektore napr. CryptoUtils, parser a oracle sa nastavuju jak typy. No proste to premysliet aj toto jak to nejak zjednotit zeeby to slo lahko menit parametre a zeby lahko slo spustat z "user" hladiska.~~ _(constants a runparams su zjednotene do `config.rs` a ostatne sa musia spustat cez features)_
- 🔄 (in progress) pozriet co sa podpisuje pri realnom BTC pouziti v tx (ja aktualne totiz podpisujem iba String, co vsak nemusi byt dobra analogia 1:1 ku btc tx, ak sa nepodpisuje priamo btc tx), a pripadne to zmenit tak, aby to viac odrazalo skutocnost. Mozno podpisujeme len nejaky hash abo nieco take
- 🔄 (in progress) overit spravnost schnorr verify
- pozriet licenciu na vykradnuteho schnorra (MAL BY TO BYT MIT A TEDA CAJK, ALE POZRIET ESTE)
- vyriesit compile warnings
    + k tomu takisto spustit `clippy` a vyriesit aj jeho
- vyriesit a vymazat **TODO komentare** z kodu
- rozumne okomentovat kod
- porobit testy tam, kde to dava zmysel
    + create tests for adaptor signatures, that pass and fail on verification or also some other parts of protocol


## [NICE TO HAVE]
- asig optimization (relevant)
- do benchmark run dodat aj mensie funkcie z `init_storage()` a `verify_adaptors()` (sice to mam v osobitnom benchmarku tieto mensie funkcie, ale neni to na real datach)
- eventualne by mi mozno bolo dobre pouzivat XOnlyPublicKey kvoli optimalizacii? Zalezi...
- ❔caching (nepomoze az tolko ako schnorr asig, ev. staci argumentacia v DP)
- regtest
- rework parameters for correct pattern (This pattern is not so much a hard rule as it is an idiomatic style that typically leads to clearer, safer APIs in Rust.)
    + _Reading only? Use &T._
    + _Mutate in place? Use &mut T._
    + _Consume/transform/own? Use T by value._

## [podla files] (mimo tych nadomnou)
- `adaptor_signature_scheme`
    + globalny secp kontext
- `common`
    - `constants.rs`
        + ✅ ~~nejak zgeneralizovat "Alice" a "Bob", aby to nezaviselo len na jednom stringu (to je aj vo `very_simple_controller.rs`)~~ _(pouzivame Offerer & Accepter)_
        + ✅ ~~eventualne nastavit `MAX_OUTCOME` niekde inde v kode, aby sme mohli parametrizovat iba pomocou `NB_DIGITS`~~
    - `error.rs`
        + po ukonceni refactoringu ak nepotrebujem unused errory tak ich vymazat
    - `fun.rs`
        + TODO komentar
        + BTC tx task; rozmysliet realny CET
    - `runparams.rs`
        + zjednotit parametre task;
    - `types.rs`
        + ParsedContract Outcome parameter nejak inak urobit
        + CET bude treba urobit iny alebo aspon rozmysliet aby som mal dobru analogiu so stringom
        + ContractInput vycistit rules (ako komentare) ktore sa nepouzivaju
- `dlc_controller`
    + nemam nic konkretne teraz, ale bude to treba este raz prejst a vycistit tak, ked uz ostatne veci budu tak nejak poriesene (napr. MAX_OUTCOME atd)
- `oracle`
    + nepouzivam nikde `event_id` ani `next_attestation_time` -> nejak to treba zdokumentovat potom v praci/dokumentacii