spellchecker-autocorrect
========================

Spellchecker forked from WillSen/spellchecker-autocorrect and ported to rust.

It's been modified to use any path file it's given as a corpus file (no error checking has been implemented), and it will take the results of whatever corpus file it's given and cache it in a "corpus_count.txt" file for quicker use later.

just call up `suggest(<dir>, "foo")` and it'll return it's best guess as a String.
