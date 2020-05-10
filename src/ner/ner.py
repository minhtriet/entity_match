from flair.models import SequenceTagger

from flair.data import Sentence

tagger = SequenceTagger.load('ner')

# make a sentence
sentence = Sentence('I love Jonathan Joestar.')

# load the NER tagger
tagger = SequenceTagger.load('ner')

# run NER over sentence
tagger.predict(sentence)
print(sentence)
for entity in sentence.get_spans('ner'):
    print(entity)
