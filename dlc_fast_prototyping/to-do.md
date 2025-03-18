# TO-DO

- ✅ finalize_tx_valid musi byt ASigS agnosticka - musi byt schopna rozlisit ASigS a podla toho spravit korektnu validaciu tx
- ❌ Oracle v Controlleri v trait params nepouzivam. Takze to pravdepodobne musi ist urobit nejak inak. Prerobit - asi to nejde nijak inak urobit ak nechcem pouzivat dynamic dispatch

- zgeneralizovat kod, dlc_controller (mozno aj storage a computation, ten neviem) controller tam ma nahardcodene veci
 controller by mal byt hotovy, teraz este computation a storage

- velmi casto v kode pouzivam ako trait parameter *AdaptorSignatureScheme* pricom realne, jedina vec ktoru potrebujem odtial tak je *ASigS::AdaptorSignature*. Nejde to urobit nejak inak, viac "krajsie" ? [ ]
- ✅ CryptoUtils budem musiet prerobit na stavove kvoli BasalCryptoUtils a caching (otazka je ze ked CryptoUtils budu stavove, tak ze ci poto aj DlcComputation potrebuje byt stavova)
- eventualne by mi mozno bolo dobre pouzivat XOnlyPublicKey kvoli optimalizacii? Zalezi...
- ✅ pozriet licenciu na vykradnuteho schnorra (MAL BY TO BYT MIT A TEDA CAJK)

- (in progress) paralelne pristupy pravdepodobne neni potreba davat do oosobitnych implementacii, ale asi by stacilo iba urobit jak to maju oni v `rust-dlc` ze `#[cfg(feature = "parallel")]` a `#[cfg(not(feature = "parallel"))]` v jednom kode. takto nemusime duplikovat rovnaky kod a zaroven mozeme dat testy rovno do implementacie a ked tomu paralelizacia nevadi, tak ani testy neduplikujeme a mame rovno otestovanu parallel aj non-parallel verziu. dlc_computation hotove, este parser
- create tests for adaptor signatures, that pass and fail on verification or also some other parts of protocol
- ✅ asi prerobit payout na pouzivanie u64 namiesto u32. Ak mam U32, ta viem poslat nanajvys ~40BTC.

[10.3.2025]
todo na dnes
- ✅ dokoncit parser - urobit ho error proof a robustny do istej miery
- ✅ prerobit interface asigs aby brali keypair namiesto secret_key kvoli schnorr adaptor
- ✅ spolocne s parserom bude treba pomenit typy outcomes, payouts atd tak, aby davali zmysel
    + ✅ delenie integer si myslim ze nedava dobry increment do step pre naplnanie payoutu, takze tiez premysliet
    + takisto asi bude treba urobit druhu funkciu, ktora vracia len `ContractInput` pretoze max collateral a toto sa vytahuje z kontraktu a tu naparsovanu strukturu ContractInput potrebujem byt schopny pouzivat v kode
- rozmysliet ci nechat ParsedInput ako vektor parov alebo index bude outcome a payout bude iba hodnota

- teoreticky, eventy v oracle by mi mohli sluzit ako MAX_OUTCOME identifier. rozmysliet potom neskor