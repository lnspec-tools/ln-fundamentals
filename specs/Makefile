BOLTDIR=bolts
LNGEN=lncodegen-cli -l rust generate -b

all: bolt1.csv bolt2.csv bolt4.csv bolt7.csv

bolt1.csv: $(BOLTDIR)/01-messaging.md
	$(LNGEN) bolt1.csv ../fundamentals/src/bolt/bolt1.rs
bolt2.csv: $(BOLTDIR)/01-messaging.md
	$(LNGEN) bolt2.csv ../fundamentals/src/bolt/bolt2.rs
#bolt4.csv: $(BOLTDIR)/04-onion-routing.md
#	$(LNGEN) bolt4.csv ../fundamentals/src/bolt/bolt4.rs
bolt7.csv: $(BOLTDIR)/07-routing-gossip.md
	$(LNGEN) bolt7.csv ../fundamentals/src/bolt/bolt7.rs

bolt%.csv:
	$(BOLTDIR)/tools/extract-formats.py $< > $@
