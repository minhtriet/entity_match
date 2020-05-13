from flair.models import SequenceTagger
from flair.data import Sentence


class FlairNERParser:
    def __init__(self):
        # load the NER tagger
        self.tagger = SequenceTagger.load('ner')

    # make a sentence
    def parse(self, sentence):
        sentence = Sentence(sentence)
        # run NER over sentence
        self.tagger.predict(sentence)
        print(sentence)
        for entity in sentence.get_spans('ner'):
            yield entity.start_pos, entity.end_pos
