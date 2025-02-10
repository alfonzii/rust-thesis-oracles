# NAMING CONVENTION
PARAMETRY FUNKCIE
full name (vynimka je ze namiesto adaptor_signature -> adaptor)
typy input parametrov - normalne podla typu; + nazov parametru nech odpoveda tomu co robi - zase vynimka bude cp_adaptors (counterparty_adaptor_signatures)

typy output parametrov - ak je alias viac deskriptivny, tak alias; tu davame typ kvoli tomu, ze return value nema nazov, takze nevieme vzdy dedukovat typ jednoducho

vsade kde sa pouzivaju moje custom typy, tak pouzivame naming `types::Type` kvoli tomu, aby bolo uplne jasne, ze sa jedna o lokalny custom typ

LOKALNE PREMENNE
na rozdiel od parametrov funkcie, tak pouzivame skor skratene nazvy nejak rozumne, na odlisenie

TRAITY:
Out - Outcome
O - Oracle
ASigS - adaptor signature scheme
CU - crypto utils